# Mutex Benchmark Comparison Table

## Source
[trading-engine/benches/sharing.rs](./trading-engine/benches/sharing.rs)
[examples/multi_consumers/main.rs](./examples/multi_consumers/main.rs)

## Benchmark Results

*Table 1: Performance comparison of Arc\<parking_lot::Mutex\<u64>> vs Arc\<Mutex\<u64>> for 100,000 accesses across different thread counts, tested on Intel Core i5-14400F*


| Threads | Arc<parking_lot::Mutex<A>> | Arc<Mutex<A>>     |
|---------|----------------------------|-------------------|
| 1       | 13 ms                  | 1.3 ms         |
| 3       | 10 ms                  | 20 ms         |
| 5       | 34 ms                  | 42 ms         |
| 7       | 168 ms                  | 64 ms         |
| 9       | 377 ms                  | 80 ms         |

*Table 2: Performance comparison of Arc\<parking_lot::Mutex\<u64>> vs Arc\<Mutex\<u64>> for 100,000 accesses across different thread counts, tested on AMD Ryzen 7700*


| Threads | Arc<parking_lot::Mutex<A>> | Arc<Mutex<A>>     |
|---------|----------------------------|-------------------|
| 1       | 456 us                  | 425 us         |
| 3       | 2.8 ms                  | 10.7 ms         |
| 5       | 8.1 ms                  | 31 ms         |
| 7       | 20 ms                  | 54 ms         |
| 9       | 77 ms                  | 67 ms         |


*Table 3: Crossbeam_channel::unbounded average one way trip time (in nanoseconds) for sending u64 to multi-receiver*

| # of receiver | Intel Core i5-14400F | AMD Ryzen 7700 |
|--|--| -- |
|1|69 ns| 52 ns |
|3| 318 ns| 113 ns | 
|6| 433 ns | 169 ns | 
|7| 443 ns| 222 ns |
|9| 773 ns| 332 ns |