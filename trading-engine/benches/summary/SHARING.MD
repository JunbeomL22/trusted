# Mutex Benchmark Comparison Table

## Source
[trading-engine/benches/sharing.rs](./trading-engine/benches/sharing.rs)
[examples/multi_consumers/main.rs](./examples/multi_consumers/main.rs)

## Benchmark Results

### Test Environment
- Intel Core i5-14400F
- RAM: 16GB
- Windows 10

*Table 1: Performance comparison of Arc\<parking_lot::Mutex\<u64>> vs Arc\<Mutex\<u64>> for 100,000 accesses across different thread counts, tested on Intel Core i5-14400F*

| Threads | Arc<parking_lot::Mutex<A>> | Arc<Mutex<A>>     |
|---------|----------------------------|-------------------|
| 1       | 13 ms                  | 1.3 ms         |
| 3       | 9.6 ms                  | 20.2 ms         |
| 5       | 34 ms                  | 42 ms         |
| 7       | 168 ms                  | 64 ms         |
| 9       | 377 ms                  | 80 ms         |

*Table 2: Crossbeam_channel::unbounded average one way trip time (in nanoseconds) for sending u64 to multi-receiver*

| # of receiver | average time |
|--|--|
|1|69 ns|
|3| 318 ns|
|6| 433 ns |
|7| 443 ns|
|9| 773 ns|