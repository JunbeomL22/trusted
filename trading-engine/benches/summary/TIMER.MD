# Timer

## Source
- [trading-engine/benches/timer.rs](./trading-engine/benches/timer.rs)

## Type Conversion Performance Table

| Timer                                      | intel i5-14400F | AMD Ryzen 7700 |
|--------------------------------------------|-----------------|----------------|
| chrono::DateTime Utc                       | 40ns            | 32 ns |
| chrono::DateTime Local (Local::now)        | 411ns           | 381 ns | 
| time::OffsetDatetime Utc                   | 43ns            | 37 ns |
| time::OffsetDatetime Local                 | 50ns            | 43 ns |
| minstant now (w.o. tsc)                    | 26ns            | 23 ns |
| minstant now (with tsc)                    | 7ns             | 7 ns |
| SystemTime now                             | 26ns            | 22 ns |
| time now                                   | 26ns            | 22 ns |
| quanta now                                 | 7ns             | 7 ns |
| quanta clock raw                           | 7ns             | 7 ns |
| minstant unix_nano (w.o tsc)               | 33ns            | 29 ns |
| minstant unix_nano (with tsc)              | 8ns             | 7 ns |
| systemtime -> unix_nano                    | 27ns            | 25 ns |
| quanta -> custom unix_nano                 | 8ns             | 7 ns |
| quanta -> custom unix_nano thread_local    | 9ns             | 7 ns |
| quanta -> custom unix_nano 3 threads (why?)| 9ns             | 9 ns |
| quanta -> custom unix_nano thread_local 3 threads | 14ns     | 9 ns |


