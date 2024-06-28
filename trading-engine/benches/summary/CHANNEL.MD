# Channel Performance Comparison Table

## Source
- [examples/communication](./examples/communication/src/main.rs)

## Results

### Test Environment
- Intel Core i5-14400F
- RAM: 16GB
- Windows 10


*Table 1: Average round trip time (in nanoseconds) for a struct1 with Vec\<u64> of size 1*

| cores | std         | crossbeam   | crossbeam-pad | kanal     | flume      |
|-------|-------------|-------------|---------------|-----------|------------|
| (0,0) | 97,972ns    | 12,149ns    | 11,573ns      | 646ns     | 44,436ns   |
| (0,1) | 411ns       | 200ns       | 215ns         | 151ns     | 418ns      |
| (0,2) | 1,033ns     | 449ns       | 630ns         | 537ns     | 948ns      |
| (0,3) | 918ns       | 447ns       | 719ns         | 599ns     | 979ns      |
| (1,1) | 94,163ns    | 12,550ns    | 11,743ns      | 711ns     | 45,626ns   |
| (1,2) | 1,046ns     | 464ns       | 606ns         | 494ns     | 961ns      |
| (1,3) | 975ns       | 441ns       | 574ns         | 565ns     | 871ns      |
| (2,2) | 94,787ns    | 11,629ns    | 11,645ns      | 709ns     | 46,955ns   |
| (2,3) | 335ns       | 198ns       | 215ns         | 171ns     | 399ns      |
| (3,3) | 92,996ns    | 12,084ns    | 12,272ns      | 702ns     | 46,398ns   |

*Table 2: Average round trip time (in nanoseconds) for a struct1 with Vec\<u64> of size 1,000*

| cores | std         | crossbeam   | crossbeam-pad | kanal     | flume      |
|-------|-------------|-------------|---------------|-----------|------------|
| (0,0) | 101,465ns   | 16,276ns    | 14,655ns      | 4,745ns   | 48,338ns   |
| (0,1) | 5,138ns     | 6,184ns     | 6,039ns       | 5,327ns   | 6,547ns    |
| (0,2) | 6,930ns     | 5,788ns     | 5,564ns       | 4,300ns   | 5,631ns    |
| (0,3) | 5,770ns     | 4,914ns     | 5,828ns       | 4,269ns   | 4,687ns    |
| (1,1) | 98,330ns    | 15,591ns    | 14,925ns      | 4,545ns   | 45,121ns   |
| (1,2) | 5,338ns     | 5,644ns     | 4,996ns       | 4,881ns   | 4,701ns    |
| (1,3) | 5,801ns     | 4,935ns     | 5,266ns       | 6,167ns   | 4,749ns    |
| (2,2) | 96,621ns    | 14,656ns    | 15,912ns      | 5,146ns   | 43,392ns   |
| (2,3) | 4,887ns     | 5,383ns     | 5,807ns       | 4,233ns   | 4,404ns    |
| (3,3) | 98,573ns    | 15,387ns    | 14,685ns      | 5,396ns   | 47,905ns   |

