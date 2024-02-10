/// The Price struct represents a price in the trading engine.
/// value: The value of the price.
/// precision: The precision of the price.
/// # Example
/// ```
/// let price = Price {
///   value: 10101,
///   precision: 2,
/// };
/// ```
/// In this example, the price is 101.01.
pub struct Price {
    value: i64,
    precision: u8,
}