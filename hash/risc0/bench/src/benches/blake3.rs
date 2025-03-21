use benchmark_methods::{
    BLAKE3_ELF, BLAKE3_ID
};
use risc0_zkvm::{ExecutorEnv, sha, ExecutorImpl};
use std::time::Instant;

pub fn blake3_bench(input: Vec<u8>) {

  let env = ExecutorEnv::builder()
    .write_slice(&input)
    .build()
    .unwrap();

  let mut exec = ExecutorImpl::from_elf(env, &BLAKE3_ELF).unwrap();
  let session = exec.run().unwrap();

  // Produce a receipt by proving the specified ELF binary.
  let (receipt, proving_time) = {

    let start = Instant::now();
    let receipt = session.prove().unwrap();
    let elapsed = start.elapsed();

    (receipt, elapsed)
  };
  
  //proof size
  let proof_bytes = receipt
    .inner
    .composite()
    .unwrap()
    .segments
    .iter()
    .fold(0, |acc, segment| acc + segment.get_seal_bytes().len())
    as u32;
  
  //number of cycles
  let cycles = session.total_cycles;

  // verify your receipt
  let verification_time = {

    let start = Instant::now(); 
    receipt.verify(BLAKE3_ID).unwrap();
    let elapsed = start.elapsed();

    elapsed
  };

  let _output: sha::Digest = receipt.journal.decode().unwrap();

  eprintln!("Proving Time: {:?}", proving_time);
  eprintln!("Verification Time: {:?}", verification_time);
  eprintln!("Proof Bytes: {:?}", proof_bytes);
  eprintln!("Total Cycles: {:?}", cycles);
  eprintln!("Hash: {:?}", _output);

}