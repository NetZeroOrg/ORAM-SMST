# ORAM-Based Sparse Merkle Sum Tree

ORAM stands for Oblivious RAM is a cryptographic primitive allowing updates to
data (stored in a tree) in a manner that does not allow for access patteren leakage

If we had a normal tree much like most of the implementations a user might observe update history of it's sibling (included in the merkle proof) and link the transactions to onchain data for same amount.

This is a access pattern leakage attach. This implemenation is based on ORAM-based SMT introduced in the [DAPOL +](https://eprint.iacr.org/2021/1350.pdf) the implementation is adapted to our usecase of Proof of Solvency in case of a blockchain custodian

## Running Test for Tree and circuits

To run the test tree there is an `e2e` test in the `src/tree.rs`

```bash
cargo test tree_tree
```

This creates the tree from random records, finds a path for a random leaf , stores this merkle proof in a json file which is then used for the circuit test
To run the circuit tests run this command

```bash
cd circuits && pnpm i && pnpm test
```

This runs the test on a sample proof in `circuits/sample/test_proof.json` esentially reconstructing the root from the given parameters
