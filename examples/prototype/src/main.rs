use anyhow::{Context, Result};
use pcap::Capture;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use time::{OffsetDateTime, Duration, UtcOffset};
use statrs::statistics::Statistics;

//
use trading_engine::data::krx::interface_map::KRX_TR_CODE_MAP;

const PCAP_FILE: &str = "data/small_20231228105204.pcap";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (k, v) in KRX_TR_CODE_MAP.iter() {
        println!("{:?} -> {:?}", std::str::from_utf8(k).unwrap(), v);
    }

    let local_offset = UtcOffset::from_hms(9, 0, 0)?;

    let tr_keys_str_slice = ["G704F", "G705F"];
    let tr_keys: Vec<&[u8]> = tr_keys_str_slice.iter().map(|&x| x.as_bytes()).collect();
    let mut tr_vec: Vec<Vec<u8>> = Vec::new(); // to store tr data
    let mut packet_time: Vec<u64> = Vec::new(); // unix_nano // to check statistics of packet time

    // Open the PCAP file
    let mut capture = Capture::from_file(PCAP_FILE).context("Failed to open PCAP file")?;
    let mut print_count = 0;
    let print_count_limit = 2;

    while let Ok(packet) = capture.next_packet() {
        if let Some(ethernet_packet) = EthernetPacket::new(packet.data) {
            if ethernet_packet.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
                // Parse the IPv4 packet
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    // Check if the IPv4 packet contains a TCP segment
                    if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                        // Parse the TCP segment
                        if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                            // Extract and print the TCP payload
                            let ts = packet.header.ts;
                            //println!("{:?}", ts);
                            let secs = ts.tv_sec as i64;
                            let nsecs = ts.tv_usec as u32 * 1000; // Convert microseconds to nanoseconds
                            let dt_utc = OffsetDateTime::UNIX_EPOCH + Duration::seconds(secs) + Duration::nanoseconds(nsecs as i64);
                            let dt_local = dt_utc.to_offset(local_offset);

                            packet_time.push(ts.tv_sec as u64 * 1_000_000_000 + ts.tv_usec as u64 * 1_000);
                            let payload = udp_packet.payload();
                            if tr_keys.contains(&&payload[..5]) {
                                tr_vec.push(payload.to_vec());
                                if print_count < print_count_limit {
                                    println!("---");
                                    println!("Captured a UDP packet at timestamp: {}", dt_local);
                                    println!("UDP Payload: {:?}", String::from_utf8_lossy(payload));
                                    print_count += 1;
                                }
                            }
                        }
                    } else if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                        // Parse the TCP segment
                        if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                            // Extract and print the TCP payload
                            let ts = packet.header.ts;
                            //println!("{:?}", ts);
                            let secs = ts.tv_sec as i64;
                            let nsecs = ts.tv_usec as u32 * 1000; // Convert microseconds to nanoseconds

                            let dt_utc = OffsetDateTime::UNIX_EPOCH + Duration::seconds(secs) + Duration::nanoseconds(nsecs as i64);
                            let dt_local = dt_utc.to_offset(local_offset);
                            packet_time.push(ts.tv_sec as u64 * 1_000_000_000 + ts.tv_usec as u64 * 1_000);
                            let payload = tcp_packet.payload();
                            if tr_keys.contains(&&payload[..5]) {
                                tr_vec.push(payload.to_vec());
                                if print_count < print_count_limit {
                                    println!("---");
                                    println!("Captured a TCP packet at timestamp: {}", dt_local);
                                    println!("TCP Payload: {:?}", String::from_utf8_lossy(payload));
                                    print_count += 1;
                                }
                            }
                            
                        }
                    }
                } else {
                    println!("---");
                    println!("Non IPv4 Packet.");
                }
            }
        }
    }
    /*
    for tr in tr_vec {
        println!("---");
        println!("TR: {:?}", String::from_utf8_lossy(&tr));
    }
     */
    let packet_time_diff = packet_time.windows(2).map(|w| w[1] as f64 - w[0] as f64).collect::<Vec<f64>>();
    for (i, diff) in packet_time_diff.iter().enumerate() {
        println!("packet_time_diff[{}]: {:?}", i, diff);
        if i > 10 {
            break;
        }
    }
    println!("packet time minimum: {:?}", OffsetDateTime::from_unix_timestamp_nanos(packet_time.iter().min().unwrap().clone() as i128).unwrap().to_offset(local_offset));
    println!("packet time maximum: {:?}", OffsetDateTime::from_unix_timestamp_nanos(packet_time.iter().max().unwrap().clone() as i128).unwrap().to_offset(local_offset));
    let packet_number = packet_time.len();
    println!("packet number: {:?}", packet_number);
    println!("mean: {:?}", (&packet_time_diff).mean());
    println!("stddev: {:?}", (&packet_time_diff).std_dev());
    println!("min: {:?}", (&packet_time_diff).min());
    println!("max: {:?}", (&packet_time_diff).max());

    let mut sorted_packet_time_diff_u64 = packet_time_diff.iter().map(|&x| x as u64).collect::<Vec<u64>>();
    sorted_packet_time_diff_u64.sort();

    for (i, diff) in sorted_packet_time_diff_u64.iter().enumerate() {
        println!("sorted_packet_time_diff_u64[{}]: {:?}", i, diff);
        if diff > &1_000 {
            break;
        }
    }

    Ok(())
}
