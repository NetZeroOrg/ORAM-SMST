# ORAM-Based Sparse Merkle Sum Tree

ORAM stands for Oblivious RA is a cryptographic primitive allowing updates to
data (stored in a tree) in a manner that does not allow for access patteren leakage

If we had a normal tree much like every implementation other a user might observe update history of it's sibling (included in the merkle proof) and link the transactions to onchain data for same amount.

This is a access pattern leakage attach. This implemenation is based on ORAM-based SMT introduced in the [DAPOL +](https://eprint.iacr.org/2021/1350.pdf) the implementation is adapted to our usecase of Proof of Solvency in case of a blockchain custodian
