/// 증권 체결
/// Stock Trade
/// Message Structure:
/// +----------------------------------------+----------+--------+--------+
/// | ItemName                               | DataType | Length | CumLen |
/// |----------------------------------------|----------|--------|--------|
/// | Data Category                          | String   | 2      | 2      |
/// | Information Category                   | String   | 3      | 5      |
/// | Message sequence number                | Int      | 8      | 13     |
/// | Board ID                               | String   | 2      | 15     |
/// | Session ID                             | String   | 2      | 17     |
/// | ISIN Code                              | String   | 12     | 29     |
/// | A designated number for an issue       | Int      | 6      | 35     |
/// | Processing Time of Trading System      | String   | 12     | 47     |
/// | Price change against previous day      | String   | 1      | 48     |
/// | A Price change against the previous day| Double   | 11     | 59     |
/// | Trading Price                          | Double   | 11     | 70     |
/// | Trading volume                         | Long     | 10     | 80     |
/// | Opening Price                          | Double   | 11     | 91     |
/// | Today's High                           | Double   | 11     | 102    |
/// | Today's Low                            | Double   | 11     | 113    |
/// | Accumulated Trading Volume             | Long     | 12     | 125    |
/// | Accumulated Trading value              | FLOAT128 | 22     | 147    |
/// | Final Ask/Bid Type Code                | String   | 1      | 148    |
/// | LP Holding Quantity                    | Long     | 15     | 163    |
/// | The Best Ask                           | Double   | 11     | 174    |
/// | The Best Bid                           | Double   | 11     | 185    |
/// | End Keyword                            | String   | 1      | 186    |
/// +----------------------------------------+----------+--------+--------+
#[derive(Debug, Clone)]
pub struct IFMSRPD0004 {

}
