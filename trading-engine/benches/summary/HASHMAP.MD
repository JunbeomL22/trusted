# Timer

## Source
- [trading-engine/benches/hashmap.rs](./trading-engine/benches/hashmap.rs)

## HashMap Performance Table 

*Table1: HashMap i32 -> i32*

| Data-structure | Action | intel i5-14400F | AMD Ryzen 7700 |
|-----------------|--------|--------------|-----|
| std::HashMap | 1,000 insertion | 9 us | 6 us |
| hashbrown::HashMap | 1,000 insertion | 3.5 us | 2 us | 
| std::HashMap | all search after 100,000 insertion | 1.2 ms | 838 us |
| hashbrown::HashMap | all search after 100,000 insertion | 412 us | 278 us | 
