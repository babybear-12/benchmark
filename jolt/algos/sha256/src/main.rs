use jolt::Serializable;

pub fn main() {
    let total = std::time::Instant::now();
    let (prove, verify) = guest::build_sha2();
    println!("SRS Compute and Guest Program Compile Time: {:?}", total.elapsed());
    
    let proving_time = std::time::Instant::now();
    let input = &[5u8; 10000];
    let (output, proof) = prove(input);
    println!("Prover Time {:?}", proving_time.elapsed());
    println!("Proof Size {:?}", proof.size().unwrap());
    println!();

    let verify_time = std::time::Instant::now();
    let is_valid = verify(proof);
    println!("Verify Time {:?}", verify_time.elapsed());
    println!();

    println!("isProofValid: {}", is_valid);
    println!("Total Time Elapsed: (build + prove + verify): {:?}", total.elapsed());
}