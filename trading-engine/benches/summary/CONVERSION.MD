# Conversion

## Source
- [trading-engine/benches/conversion.rs](/trading-engine/benches/conversion.rs)
- [trading-engine/benches/timer.rs](/trading-engine/benches/timer.rs)

## Type Conversion Performance Table

| Type                      | Conversion                | intel i5-14400F | AMD Ryzen 7700 |
|---------------------------|---------------------------|-----------------|----------------|
| chrono::DateTime UTC      | to_string                 | 260 ns           | 192 ns |
| chrono::DateTime Local    | to_string                 | 298 ns           | 206 ns |
| time::OffsetDatetime UTC  | to_string                 | 463 ns           | 294 ns |
| time::OffsetDatetime UTC  | format(&Rfc3339)          | 283 ns           | 174 ns |
| time::OffsetDatetime Local| to_string                 | 461 ns           | 299 ns |
| time::OffsetDatetime Local| format(&Rfc3339)          | 348 ns           | 226 ns |
| u64 (unix_nano)           | chrono::DateTime Local    | 364 ns           | 381 ns |
| u64 (unix_nano)           | time::OffsetDatetime Local| 33 ns            | 27 ns |
| u64 (unix_nano)           | custom_conv (String)      | 337 ns           | 217 ns | 
| u32                       | to_string                 | 50 ns            | 37 ns | 
| u64 (unix_nano)           | to_string                 | 42 ns            | 35 ns | 
| u32                       | itoa                      | 2 ns             | 2 ns |
| u64                       | itoa                      | 2 ns             | 2 ns |
| f64                       | to_string                 | 144 ns           | 105 ns |
| f64                       | ryu                       | 35 ns            | 27 ns |
| u32 | as f32 | 324 ps | 377 ps | 
| u32 | as f64 | 330 ps | 379 ps |
| u64 | as f32 | 315 ps | 378 ps |
| u64 | as f32 | 304 ps | 378 ps |
| f32 | as u32 | 264 ps | 377 ps |
| f32 | as u64 | 262 ps | 377 ps |
| f64 | as u32 | 256 ps | 377 ps |
| f64 | as u64 | 253 ps | 377 ps |
| &str                      | parse\<i32>                | 10 ns            | 7 ns |
| &str                      | parse\<i64>                | 8 ns             | 5 ns |
| &str                      | parse\<i128>               | 12 ns            | 10 ns |
| &str                      | parse\<u32>                | 9 ns             | 6 ns|
| &str                      | parse\<u64>                | 6 ns             | 5 ns | 
| &str                      | parse\<f32>                | 12 ns            | 9 ns |
| &str                      | parse\<f64>                | 8 ns             | 6 ns |
| &str                      | parse\<f64> -> i64 (BookPrice) | 15 ns            | 12 ns
| i64 (BookPrice)           | -> as_f64 -> ryu            | 31 ns            | 21 ns