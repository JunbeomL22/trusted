use env_logger;
fn main () 
{
    env_logger::init();
    let x = f64_to_fixed_i64(1.234, 3);
    println!("{:?}", x);
    println!("{:?}", FIXED_PRECISION);
}
