use anyhow::{Context, Result};
use pcap::Capture;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

const PCAP_FILE: &str = "data/small_20231228105204.pcap";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the PCAP file
    let mut capture = Capture::from_file(PCAP_FILE).context("Failed to open PCAP file")?;

    let mut count = 0;

    while let Ok(packet) = capture.next_packet() {
        count += 1;
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
                            let payload = udp_packet.payload();
                            println!("---");
                            println!("UDP Payload: {:?}", String::from_utf8_lossy(payload));
                        }
                    } else if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                        // Parse the TCP segment
                        if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                            // Extract and print the TCP payload
                            let payload = tcp_packet.payload();
                            println!("---");
                            println!("TCP Payload: {:?}", String::from_utf8_lossy(payload));
                        }
                    }
                } else {
                    println!("---");
                    println!("Non IPv4 Packet.");
                }
            }
        }
    }

    Ok(())
}
