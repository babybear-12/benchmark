use jolt::Serializable;

pub fn main() {
    let start = std::time::Instant::now();

    let srs_compute_and_guest_program_compile_time = std::time::Instant::now();
    let (prove, verify) = guest::build_sub();
    println!("SRS Compute and Guest Program Compile Time {:?}", srs_compute_and_guest_program_compile_time.elapsed());
    println!();

    let proving_time = std::time::Instant::now();
    let (output, proof) = prove(100, 10);
    println!("Prover Time {:?}", proving_time.elapsed());

    // proof
    //     .save_to_file("proof.bin")
    //     .expect("Failed to save proof to file");
    println!("Proof Size {:?}", proof.size().unwrap());
    println!();

    let verify_time = std::time::Instant::now();
    let is_valid = verify(proof);
    println!("Verify Time {:?}", verify_time.elapsed());
    println!();

    // println!("result: {:?}", output);
    println!("isProofValid: {}", is_valid);

    println!("Total Time Elapsed: (build + prove + verify) {:?}", start.elapsed());
}
