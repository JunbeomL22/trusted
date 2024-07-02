# Memory

## Struct
Memory alignment depends on the most strict variable. Consider:

```Rust
pub struct A {
    a: u8,
    b: u64,
    c: u16,
}
```
Memory alignment size of the struct A is 8 byte (the variable "b"). "a" takes up the first place of the memory alignment leaving the available space 7byte. However, b can not be placed in the memory block, so put in the next alignment. Then, "c" will be in the third memory alignment. Thus, it takes 3 * 8 byte memory space. 
You can find the memory display of the struct in [/examples/mem_investigation](/examples/mem_investigation/src/main.rs):

```
Size: 24 bytes
Alignment: 8 bytes

Memory contents:
0000: AA 00 00 00
0004: 00 00 00 00
0008: AA AA AA AA
000C: AA AA AA AA
0010: CC CC B4 EB
0014: 6A 02 00 00

Pointer to instance: 0x9cf9cff5d0
No VTable (not a trait object)
```

## Enum
Enum's size is the beggiest element's size, but there is a 4 byte tag for which element is used. For example, consider:
```Rust
pub enum MyEnum16_32_64 {
    A(u16),
    B(u32),
    C(u64),
}
```
The alignment is as follows:
1) tag (4 byte) + pad (4 byte)
2) the enum memory (by "C") (8 byte)

which results in 16 byte.

## Worst case
Consider the case we have a vectors of the following struct A and use only val for a certain calculation. Becuase the _pad makes the size of A becomes double of f64, it 2 times slower than using primitive f64. See [bench](/trading-engine/benches/wrapper_performance.rs)
```Rust
struct A {
    val: f64,
    _pad: u8,
}
```
The benchmark on on AMD Ryzen 7700, multiplcation of 1,000 element of f32, f64, and A is as follows:

* f32: 190 ns
* f64: 394 ns
* A: 937 ns
