use methods::POSEIDON_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, SessionStats};

pub fn poseidon(n: u32) -> impl FnMut() -> (Receipt, SessionStats) {
    let env = ExecutorEnv::builder()
        .write(&[n])
        .expect("Failed to write numbers")
        .build()
        .expect("Failed to build env");

    let prover = default_prover();
    let prove_info = prover.prove(env, POSEIDON_ELF).expect("Proving failed");

    move || {
        let session = &prove_info.stats;
        let receipt = &prove_info.receipt;
        let session_stats = SessionStats {
            segments: session.segments,
            total_cycles: session.total_cycles,
            user_cycles: session.user_cycles,
        };
        (receipt.clone(), session_stats)
    }
}
