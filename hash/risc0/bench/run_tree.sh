#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE_TREE} ]; then 
ZKBENCH_HASH_TYPE_TREE="poseidon2_babybear_native"    
fi

if [ -z ${ZKBENCH_TREE_DEPTH} ]; then 
ZKBENCH_TREE_DEPTH=2
fi

echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE_TREE"
echo "Tree Depth = $ZKBENCH_TREE_DEPTH"

# Run the benchmarks
RUST_LOG=info ./target/release/benchmark $ZKBENCH_HASH_TYPE_TREE $ZKBENCH_TREE_DEPTH