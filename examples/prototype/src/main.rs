use std::fs::File;
use std::io::{Write, Read};
use pcap::{Capture, Device};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
struct PacketPayload {
    payload: Vec<u8>,
}

fn main() -> Result<()> {
    let input_file = "data/small_20231228105204.pcap";
    let output_file = "data/payloads.bin";

    // Extract payloads
    let payloads = extract_payloads(input_file)?;
    println!("Extracted {} payloads", payloads.len());

    // Serialize and save payloads
    serialize_payloads(&payloads, output_file)?;
    println!("Serialized payloads saved to {}", output_file);

    // Read and deserialize payloads
    let deserialized_payloads = deserialize_payloads(output_file)?;
    println!("Deserialized {} payloads", deserialized_payloads.len());

    // Verify deserialized data
    assert_eq!(payloads.len(), deserialized_payloads.len(), "Payload count mismatch");
    for (original, deserialized) in payloads.iter().zip(deserialized_payloads.iter()) {
        assert_eq!(original.payload, deserialized.payload, "Payload data mismatch");
    }
    println!("Deserialized data verified successfully");

    Ok(())
}

fn extract_payloads(filename: &str) -> Result<Vec<PacketPayload>> {
    let mut cap = Capture::from_file(filename)
        .context("Failed to open PCAP file")?;

    let mut payloads = Vec::new();

    while let Ok(packet) = cap.next_packet() {
        let payload = PacketPayload {
            payload: packet.data.to_vec(),
        };
        payloads.push(payload);
    }

    Ok(payloads)
}

fn serialize_payloads(payloads: &[PacketPayload], filename: &str) -> Result<()> {
    let serialized = bincode::serialize(payloads)
        .context("Failed to serialize payloads")?;
    
    let mut file = File::create(filename)
        .context("Failed to create output file")?;
    
    file.write_all(&serialized)
        .context("Failed to write serialized data")?;

    Ok(())
}

fn deserialize_payloads(filename: &str) -> Result<Vec<PacketPayload>> {
    let mut file = File::open(filename)
        .context("Failed to open serialized file")?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .context("Failed to read serialized data")?;

    let payloads: Vec<PacketPayload> = bincode::deserialize(&buffer)
        .context("Failed to deserialize payloads")?;

    Ok(payloads)
}