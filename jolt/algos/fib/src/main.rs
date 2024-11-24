use jolt::Serializable;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: u64 = contents.trim().parse()?;
    println!("n: {}", n);

    let total = std::time::Instant::now();
    let (prove, verify) = guest::build_fib();
    println!("SRS Compute and Guest Program Compile Time: {:?}", total.elapsed());
    
    let proving_time = std::time::Instant::now();
    let (output, proof) = prove(n);
    println!("Prover Time {:?}", proving_time.elapsed());
    println!("Proof Size {:?}", proof.size().unwrap());
    println!();

    let verify_time = std::time::Instant::now();
    let is_valid = verify(proof);
    println!("Verify Time {:?}", verify_time.elapsed());
    println!();

    println!("isProofValid: {}", is_valid);
    println!("Total Time Elapsed: (build + prove + verify): {:?}", total.elapsed());
    Ok(())
}