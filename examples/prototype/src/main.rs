use anyhow::{Context, Result};
use pcap::Capture;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

//
use trading_engine::data::krx::interface_map::KRX_TR_CODE_MAP;

const PCAP_FILE: &str = "data/small_20231228105204.pcap";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (k, v) in KRX_TR_CODE_MAP.iter() {
        println!("{:?} -> {:?}", std::str::from_utf8(k).unwrap(), v);
    }

    let tr_keys: Vec<&[u8]> = KRX_TR_CODE_MAP.keys().map(|k| *k).collect();

    let mut tr_vec: Vec<Vec<u8>> = Vec::new();

    // Open the PCAP file
    let mut capture = Capture::from_file(PCAP_FILE).context("Failed to open PCAP file")?;

    let mut count = 0;

    while let Ok(packet) = capture.next_packet() {
        if count > 10 {
            break;
        }
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
                            let secs = ts.tv_sec as i64;
                            let nsecs = ts.tv_usec as u32 * 1000; // Convert microseconds to nanoseconds

                            // Convert to NaiveDateTime and then to DateTime<Utc>
                            let naive = NaiveDateTime::from_timestamp(secs, nsecs);
                            let datetime = DateTime::<Utc>::from_utc(naive, Utc);

                            // Print the timestamp and the packet data
                            println!("Captured a UDP packet at timestamp: {}", datetime);
                            let payload = udp_packet.payload();
                            if tr_keys.contains(&&payload[..5]) {
                                tr_vec.push(payload.to_vec());
                                count += 1;
                                println!("---");
                                println!("UDP Payload: {:?}", String::from_utf8_lossy(payload));
                            }
                        }
                    } else if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                        // Parse the TCP segment
                        if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                            // Extract and print the TCP payload
                            let payload = tcp_packet.payload();
                            if tr_keys.contains(&&payload[..5]) {
                                tr_vec.push(payload.to_vec());
                                count += 1;
                                println!("---");
                                println!("TCP Payload: {:?}", String::from_utf8_lossy(payload));
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

    for tr in tr_vec {
        println!("---");
        println!("TR: {:?}", String::from_utf8_lossy(&tr));
    }
    Ok(())
}
