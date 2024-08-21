use std::time::Instant;
use ark_serialize::CanonicalSerialize;

pub fn blake2_bench(input: Vec<u8>) {

    let (prove_blake2, verify_blake2, guest_build_time) = {

        let start = Instant::now();
        let (prove, verify) = guest::build_blake2();
        let elapsed = start.elapsed();

        (prove, verify, elapsed)
    };

    let (output, proof, proving_time) = {

        let start = Instant::now();
        let (output, proof) = prove_blake2(input.as_slice());
        let elapsed = start.elapsed();

        (output, proof, elapsed)
    };

    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();

    let (is_valid, verification_time) = {

        let start = Instant::now();
        let is_valid = verify_blake2(proof);
        let elapsed = start.elapsed();

        (is_valid, elapsed)
    };

    assert!(is_valid);
    println!("output: {:?}", hex::encode(output));
    println!("guest build time: {:?}", guest_build_time);
    println!("proving time: {:?}", proving_time);
    println!("verification time: {:?}", verification_time);
    println!("proof size: {:?} bytes", proof_bytes.len());
}