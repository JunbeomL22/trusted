# Timer

## Source
- [trading-engine/benches/hashmap.rs](./trading-engine/benches/hashmap.rs)

## HashMap Performance Table 

*Table1: HashMap i32 -> i32*

| Data-structure | Action | intel i5-14400F |
|-----------------|--------|--------------|
| std::HashMap | 1,000 insertion | 9us |
| hashbrown::HashMap | 1,000 insertion | 3.5us |
| std::HashMap | all search after 100,000 insertion | 1.2ms |
| hashbrown::HashMap | all search after 100,000 insertion | 412us |
