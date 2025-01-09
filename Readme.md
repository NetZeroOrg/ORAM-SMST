# Merkle Sum Tree

Merkle Sum Tree is the heart of proof of solvency protocols. Proposed in this research paper [Privacy-preserving proofs of solvency for Bitcoin exchanges](https://eprint.iacr.org/2015/1008.pdf)
This is typescript implementation of a Merkle Sum Tree with o1js.

## Directory structure

```bash
├── packages
│   ├── circuits # contians circuit for proof of membership and root computation
│   ├── mst # contains the core mst implmentation + serializers to save and fetch the data from db
```
