# Jolt

- [Architecture Overview](https://jolt.a16zcrypto.com/how/architecture.html)

- Benchmark Hardware
    - Macbook Pro M1 Pro - Core 8 - Memory 8 GB
    - Macbook Pro M2 Pro - Core 10 - Memory 16 GB

- Benchmark Operations
  - Arithmetic Operations: Addition, Subtraction, Multiplication, Division
  - Fibonacci: Variants 1, 10, 100, 1000, 10000, 100000
  - SHA256: Variants: 32b, 1k bytes, 2k bytes, 3k bytes, 4k bytes, 5k bytes, 10k bytes
  - Poseidon: 32 bytes

Each benchmark code followed by its output metrics is listed below.

## Addition
```rust
fn add(a: u64, b: u64) -> u64 {
    a + b
}
```

```
SRS Compute and Guest Program Compile Time 26.423080166s

Trace length: 141
Prover Time 726.201625ms
Proof Size 266277

Verify Time 40.219625ms

isProofValid: true
Total Time Elapsed: (build + prove + verify) 27.18973875s
```


## Subtraction
```rust
fn mul(a: u64, b: u64) -> u64 {
    a - b
}
```

```shell
SRS Compute and Guest Program Compile Time 27.275818875s

Trace length: 141
Prover Time 725.506417ms
Proof Size 266277

Verify Time 41.788ms

isProofValid: true
Total Time Elapsed: (build + prove + verify) 28.043415584s
```

## Multiplication
```rust
fn mul(a: u64, b: u64) -> u64 {
    a * b
}
```

```shell
SRS Compute and Guest Program Compile Time 38.166960834s

Trace length: 157
Prover Time 749.941917ms
Proof Size 266278

Verify Time 45.77175ms

isProofValid: true
Total Time Elapsed: (build + prove + verify) 38.962964125s
```

## Division

```rust
fn div(a: u64, b: u64) -> u64 {
    a / b
}
```

```shell
SRS Compute and Guest Program Compile Time 35.004701375s

Trace length: 191
Prover Time 715.949875ms
Proof Size 267469

Verify Time 46.196917ms

isProofValid: true
Total Time Elapsed: (build + prove + verify) 35.767325417s
```

## Fibonacci
> variants: 1, 10, 100, 1000, 10000, 100000

```rust
fn fib(n: u64) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    b
}
```


- Input: 1
```shell
Input n read from JSON: 1

Trace length: 102
Prover Time 22.956921625s
Proof Size 251908

Verify Time 36.636334ms

result: 1
valid: true
Total Time elapsed: 23.139271375s
```


- Input: 10
```shell
n: 10
SRS Compute and Guest Program Compile Time: 21.294665666s
Trace length: 369
Prover Time 574.347125ms
Proof Size 281060

Verify Time 39.390292ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 21.9088845s
```

- Input: 100
```shell
n: 100
SRS Compute and Guest Program Compile Time: 20.32695075s
Trace length: 3021
Prover Time 702.883708ms
Proof Size 327917

Verify Time 42.85075ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 21.073325416s
```

- Input: 1000
```shell
n: 1000
SRS Compute and Guest Program Compile Time: 20.224171875s
Trace length: 28280
Prover Time 2.08951825s
Proof Size 378519

Verify Time 48.993625ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 22.363307584s
```

- Input: 10000
```shell
n: 10000
SRS Compute and Guest Program Compile Time: 19.906918667s
Trace length: 280287
Prover Time 21.730433291s
Proof Size 452398

Verify Time 103.7595ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 41.742032542s
```


- Input: 50000
> For 8 GB of RAM, the program can handle inputs up to 37,400.
```shell
n: 50000
SRS Compute and Guest Program Compile Time: 19.734620333s
Trace length: 1400285
Prover Time 231.064130417s
Proof Size 491936

Verify Time 150.672333ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 250.951821958s
```


- Input: 62500
```shell
n: 62500
SRS Compute and Guest Program Compile Time: 23.773564084s
Trace length: 1750285
Prover Time 216.526958625s
Proof Size 491936

Verify Time 169.939709ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 240.474649334s
```

- Input: 70000
```shell
n: 70000
SRS Compute and Guest Program Compile Time: 23.012281208s
Trace length: 1960285
Prover Time 245.362881084s
Proof Size 491936

Verify Time 150.697625ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 268.528882708s
```

## SHA256
> variants: 32b, 100b, 1k bytes, 2k bytes, 3k bytes, 5k bytes, 10k bytes
```rust
fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}
```

- Input Size: 32 Bytes
```shell
SRS Compute and Guest Program Compile Time: 21.304487958s
Trace length: 4662
Prover Time 838.498666ms
Proof Size 348307

Verify Time 46.87975ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 22.190249583s
```

- Input Size: 100 Bytes
```shell
SRS Compute and Guest Program Compile Time: 21.933417208s
Trace length: 8740
Prover Time 1.345950875s
Proof Size 365239

Verify Time 46.762917ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 23.326511375s
```

- Input Size: 1k Bytes
```shell
SRS Compute and Guest Program Compile Time: 20.650488708s
Trace length: 62231
Prover Time 2.198708625s
Proof Size 401116

Verify Time 52.631292ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 22.902206375s
```

- Input Size: 2k bytes
```shell
SRS Compute and Guest Program Compile Time: 21.408510875s
Trace length: 123315
Prover Time 4.712634709s
Proof Size 420428

Verify Time 68.002541ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 26.189522709s
```

- Input Size: 3k bytes
```shell
SRS Compute and Guest Program Compile Time: 22.174915417s
Trace length: 184823
Prover Time 13.464369666s
Proof Size 440156

Verify Time 74.823ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 35.714604708s
```

- Input Size: 5k Bytes
```shell
SRS Compute and Guest Program Compile Time: 22.743658208s
Trace length: 302846
Prover Time 22.862356542s
Proof Size 463220

Verify Time 66.295458ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 45.672874083s
```

- Input Size: 10k Bytes
```shell
SRS Compute and Guest Program Compile Time: 20.949013292s
Trace length: 600815
Prover Time 47.174928041s
Proof Size 489804

Verify Time 171.185666ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 68.295701584s
```

## Poseidon
```rust
use starknet_crypto::{PoseidonHasher};
use starknet_types_core::felt::Felt;

/// Converts an arbitrary byte slice into `Felt` elements.
fn bytes_to_felts(input: &[u8]) -> Vec<Felt> {
    const FELT_BYTE_SIZE: usize = 31; // Maximum bytes for a Felt element in BN254

    input
        .chunks(FELT_BYTE_SIZE)
        .map(|chunk| {
            let mut buffer = [0u8; 32]; // BN254 requires 32 bytes, pad with zeroes
            buffer[32 - chunk.len()..].copy_from_slice(chunk); // Right-align the chunk
            Felt::from_bytes_be(&buffer)
        })
        .collect()
}

/// Ref: https://github.com/xJonathanLEI/starknet-rs/blob/master/starknet-crypto/benches/poseidon_hash.rs
#[no_mangle]
#[jolt::provable]
pub fn pos() {
    let input = &[5u8; 32];
    // Convert input into `Felt` chunks
    let felt_chunks = bytes_to_felts(input);

    let mut hasher = PoseidonHasher::new();
    for chunk in &felt_chunks {
        hasher.update(*chunk);
    }
    let hash = hasher.finalize();
}
``` 

- Input Size: 32 bytes
```shell
SRS Compute and Guest Program Compile Time: 40.689255292s
Trace length: 554595
Prover Time 91.378150083s
Proof Size 477746

Verify Time 194.948792ms

isProofValid: true
Total Time Elapsed: (build + prove + verify): 132.263622292s
```
