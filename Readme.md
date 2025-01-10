# Merkle Sum Tree

Merkle Sum Tree is the heart of proof of solvency protocols. The implmentation is much in line with `DAPOL+`
introduced in the paper : [Generalised Proof of Liabilities](https://eprint.iacr.org/2021/1350.pdf)

## Directory structure

```bash
├── packages
│   ├── circuits # contians circuit for proof of membership and root computation
│   ├── smst # contains the core mst implmentation + serializers to save and fetch the data from db
```
