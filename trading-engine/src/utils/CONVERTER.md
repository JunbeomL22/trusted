# Integer converter using bit operation
reference: [here](https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html)

## Basic token
* ascii number is from 0x30 (="0") ~ 0x39 ("9")
```Rust
let x0 = b"0"; // = 0x30 = 0b 0011 0000
let x1 = b"1"; // = 0x31 = 0b 0011 0001
//..
let x9 = b"9"; // = 0x39 = 0b 0011 0009
```
* number is little endian: 
* string slice memory order is opposite to what we see
```Rust
// Example
let x: u8 = 19; // = 2^4 + 2^1 + 2^0 => (0b) 0001 0011
let s = b"1234"; // in memory [0x34, 0x33, 0x32, 0x31]
```
* when bit shifts, the blank bit is filled with zero
```Rust
let x: u8 = 3; // 0000 0011
assert_eq!(x << 1, 6); // 0000 0110
assert_eq!(x >> 1, 1); // 0000 0001
``` 
## Examples
### single digit

```Rust
let x: &[u8; 1] = b"8";
let x: u8 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u8) };
let y: u8 = x & 0x0f;
//   0011 1000
//   &
//   0000 1111
// = 0000 1000 = 8
assert_eq!(y, 8);
```
### two digits
```Rust
let x: &[u8; 2] = b"12"; // [0x32, 0x31]
let x: u16 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u16) };
let lower: u16 = (x & 0x0f00) >> 8;
//   0011 0002 0011 0001
//   &
//   0000 1111 0000 0000
// = 0000 0002 0000 0000
// =>
// (0000 0002 0000 0001) >> 8 = 0000 0000 0000 0002
let upper: u16 = (x & 0x000f) * 10;
//   0011 0002 0011 0001
//   &
//   0000 0000 0000 1111
// = 0000 0000 0000 0001
// =>
// (0000 0000 0000 0001) * 10 = 10 
let res = lower + upper;
assert_eq!(res, 12);
```

### four digits
```Rust
let x: &[u8; 4] = b"1234"; // [0x34, 0x33, 0x32, 0x31]
let x: u32 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u32) };
let lower: u32 = (x & 0x0f000f00) >> 8; 
// [0x04, 0x00, 0x02, 0x00] >> 8 = [0x00, 0x04, 0x00, 0x02]
let upper: u32 = (x & 0x000f000f) * 10; 
// [0x00, 0x03, 0x00, 0x01] * 10 =  [00, 30, 00, 10] (formally, not rigorous bit representation)
let chunk = lower + upper;
// [00, 34, 00, 12]
let lower: u32 = (chunk & 0x00ff0000) >> 16; 
// [00, 34, 00, 12] >> 16 = [00, 00, 00, 34] = 34
let upper: u32 = (chunk & 0x000000ff) * 100; 
//   [00, 34, 00, 12] 
//   &
//   [00, 00, 00, ff]
// = [00, 00, 00, 12] => *100 => 1200
let res = lower + upper; // 34 + 1200
assert_eq!(res, 1234);
```

### irregular digits
```Rust
let x: &[u8; 3] = b"123";
let x: u32 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u32) };
// in this case x is formally "123? => [??, 0x33, 0x32, 0x31]
let x = x << 8; // [0x33, 0x32, 0x31, 0x00] // something like "0123" as desirable 
```

### negative number
```Rust
let x = b"-123";
let x_u = x[1..]; // b"123"
let x_u: u32 = bit_parse(x_u);
let res: i32 = (!x_u).wrapping_add(1) as i32 
asset_eq!(res, -123);
```