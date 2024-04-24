#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE} ]; then 
ZKBENCH_HASH_TYPE="poseidon"    
fi

if [ -z ${ZKBENCH_INPUT_SIZE_BYTES} ]; then 
ZKBENCH_INPUT_SIZE_BYTES=4
fi

echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE"
echo "Input Size (Bytes) = $ZKBENCH_INPUT_SIZE_BYTES"

# Run the benchmarks
./target/debug/plonky2_hash_benchmarks $ZKBENCH_HASH_TYPE $ZKBENCH_INPUT_SIZE_BYTES