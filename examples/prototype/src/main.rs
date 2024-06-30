use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use etherparse::SlicedPacket;
use pcap::Capture;

#[derive(Serialize, Deserialize, Debug)]
struct PacketPayload {
    pub payload: Vec<u8>,
}

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
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                println!("---");
                println!("{:?}", String::from_utf8_lossy(value.ether_payload().unwrap().payload));
            }
        }
    }

    Ok(())
}
