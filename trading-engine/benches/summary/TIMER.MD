# Timer

## Source
- [trading-engine/benches/timer.rs](./trading-engine/benches/timer.rs)

## Type Conversion Performance Table

| Timer                                      | intel i5-14400F |
|--------------------------------------------|-----------------|
| chrono::DateTime Utc                       | 40ns            |
| chrono::DateTime Local (Local::now)        | 411ns           |
| time::OffsetDatetime Utc                   | 43ns            |
| time::OffsetDatetime Local                 | 50ns            |
| minstant now (w.o. tsc)                    | 26ns            |
| minstant now (with tsc)                    | 7ns             |
| SystemTime now                             | 26ns            |
| time now                                   | 26ns            |
| quanta now                                 | 7ns             |
| quanta clock raw                           | 7ns             |
| minstant unix_nano (w.o tsc)               | 33ns            |
| minstant unix_nano (with tsc)              | 8ns             |
| systemtime -> unix_nano                    | 27ns            |
| quanta -> custom unix_nano                 | 8ns             |
| quanta -> custom unix_nano thread_local    | 9ns             |
| quanta -> custom unix_nano 3 threads (why?)| 9ns             |
| quanta -> custom unix_nano thread_local 3 threads | 14ns     |


