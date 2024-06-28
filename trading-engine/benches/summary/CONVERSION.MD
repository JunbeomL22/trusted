# Conversion

## Source
- [trading-engine/benches/conversion.rs](./trading-engine/benches/conversion.rs)
- [trading-engine/benches/timer.rs](./trading-engine/benches/timer.rs)

## Type Conversion Performance Table

| Type                      | Conversion                | intel i5-14400F |
|---------------------------|---------------------------|-----------------|
| chrono::DateTime UTC      | to_string                 | 260ns           |
| chrono::DateTime Local    | to_string                 | 298ns           |
| time::OffsetDatetime UTC  | to_string                 | 463ns           |
| time::OffsetDatetime UTC  | format(&Rfc3339)          | 283ns           |
| time::OffsetDatetime Local| to_string                 | 461ns           |
| time::OffsetDatetime Local| format(&Rfc3339)          | 348ns           |
| u64 (unix_nano)           | chrono::DateTime Local    | 364ns           |
| u64 (unix_nano)           | time::OffsetDatetime Local| 33ns            |
| u64 (unix_nano)           | custom_conv (String)      | 337ns           |
| u32                       | to_string                 | 50ns            |
| u64 (unix_nano)           | to_string                 | 42ns            |
| u32                       | iota                      | 2ns             |
| u64                       | iota                      | 2ns             |
| f64                       | to_string                 | 144ns           |
| f64                       | ryu                       | 35ns            |
| u32 | as f32 | 324ps |
| u32 | as f64 | 330ps |
| u64 | as f32 | 315ps |
| u64 | as f32 | 304ps |
| f32 | as u32 | 264ps |
| f32 | as u64 | 262ps |
| f64 | as u32 | 256ps |
| f64 | as u64 | 253ps |
| &str                      | parse\<i32>                | 10ns            |
| &str                      | parse\<i64>                | 8ns             |
| &str                      | parse\<i128>               | 12ns            |
| &str                      | parse\<u32>                | 9ns             |
| &str                      | parse\<u64>                | 6ns             |
| &str                      | parse\<f32>                | 12ns            |
| &str                      | parse\<f64>                | 8ns             |
| &str                      | parse\<f64> -> i64 (BookPrice) | 21ns            |
| i64 (BookPrice)           | -> ryu -> &str            | 31ns            |