use crate::types::base::BookPrice;
/// SimpleTaker enters a position and then acts only either to take profit or to stop loss.
/// If the bid price is higher than the bid_upper, it will sell.
/// If the ask price is lower than the ask_lower, it will buy.
pub struct SimpleTaker {
    pub bid_upper: BookPrice,
    pub ask_lower: BookPrice,
}