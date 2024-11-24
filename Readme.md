# ZK Benchmark Project

## Overview

This repository hosts the source code and resources for benchmarking various Zero-Knowledge (ZK) proofs. It builds upon the original repositories of Halo2, RISC0, Nexus, SP1, Jolt, Circom, and Powdr with our implementations of proofs including but not limited to SHA256, Fibonacci, Poseidon operations.

Our aim is to benchmark the performance of these proofs, comparing them in terms of time, space, and other metrics. This comprehensive comparison will not only highlight the most popular ZK proofs and their implementations but also deepen our understanding of their inner workings, developer friendliness and proving systems.

## Goals

- **Benchmark Operations:**
  - **Fibonacci:** Variants 1, 10, 100, 1000, 10000
  - **SHA256:** Variants 32b, 1k, 10k Bytes
  - **Poseidon:** Variants 32b, 100b

- **Additional Benchmarks:**
  - Arithmetic Operations: Addition, Subtraction, Multiplication, Division
  - Loop: Variants 10, 100, 1000
  - Hash Functions: Pederson, RPO, Keccak (Variants 1k, 10k Bytes)
  - Merkle Tree: Generation, Membership/Inclusion, Merge
  - nth Prime: Variants 1, 10, 100, 1000, 10000
  - Cryptographic Verifications: ECDSA, Elliptic Curve, BLS Verification, BLS Aggregation

## Proving Systems

- [Halo2](./halo2/)
- [Circom](./circom/)
- [Nexus zkVM](./nexus/)
- [Risc Zero](./risc-zero/)
- [SP1](./sp1/)
- [Jolt](./jolt/)
- [Powdr](./powdr/)

#### General overview of features of proving schemes
<img width="600" alt="Screenshot 2024-11-22 at 3 44 30 PM" src="https://github.com/user-attachments/assets/2768a9c5-7477-4e7b-86aa-f80a85818c2f">

## Metrics

- Prover Time
- Verifier Time
- Prover Space
- Verifier Space
- Proof Size
- Parallel Execution
- CPU Usage
- Memory Usage
- Verifier Gas Consumption
- Cycles Count
- Developer Friendliness

## Reports

The detailed reports can be found in the [reports](./reports/) directory.
- [Reports](./reports/)
  - [Jolt](./reports/jolt/)
  - [SP1](./reports/sp1/)
  - [Powdr](./reports/powdr/)
  - [Halo2](./reports/halo2/)
  - [Nexus](./reports/nexus/)
  - [Circom](./reports/circom/)
 
#### Report Summary
> Detailed report has metrics for other variants and additional operations.

##### Fibonacci - 10000 th number
<img width="600" alt="fib" src="https://github.com/user-attachments/assets/e99bd993-ecb3-43dd-8e39-c90a4a8d17f6">

##### SHA256 - 1 KB Input
<img width="600" alt="SHA256" src="https://github.com/user-attachments/assets/bd8a369b-1b4a-44cf-b541-0bdfdd5a5367">

##### Poseidon - 32 Byte input
<img width="600" alt="Poseidon" src="https://github.com/user-attachments/assets/c468f0fc-163d-44f8-9b0e-b726dd7a8653">

## Benchmark Machine Specifications

- **Macbook M1 Pro:** Core 8, Memory 8 GB
- **AlmaLinux 8.10:** Core 16, Memory 32 GB, Disk 1 TB
- **Windows 11:** Core 16, Memory 32 GB, Disk 1 TB
- **Intel(R):** Core 4 - Memory 8GB
- **Macbook M2 Pro:** Core 16, Memory 16 GB

## Installation

Each project has its own README file with instructions on how to install the dependencies and run the benchmarks.

## Contributors

This project is part of the ZK and Scaling Bootcamp organized by Encode club.  
- Tanmoy : https://x.com/mtanm0y
- Rosemary :https://x.com/rosekoikara
- Yogesh : https://x.com/yogesh_eth
- Anubha

## Acknowledgments
Special thanks to the ZK and Scaling Bootcamp team for organizing this event and providing us with the opportunity to learn and contribute to the ZK space.
