fn main() {
    let x: u64 = 1;
    println!("{:?}", x.to_le_bytes());
    println!("{:?}", (x << 16).to_le_bytes());
    println!("{:?}", ((x << 32) >> 16).to_le_bytes());

    let x: u64 = 2_f32.powi(63) as u64;
    println!("{:?}", x.to_le_bytes());
    println!("{:?}", (x >> 16).to_le_bytes());
    println!("{:?}", ((x >> 32) << 16).to_le_bytes());
}
