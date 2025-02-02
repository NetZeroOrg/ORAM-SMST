import { assert, Poseidon, Provable, ZkProgram } from "o1js";
import { MerkleWitness, NodeContent } from "./types";
export const computeRoot = async (witness: MerkleWitness, userLeaf: NodeContent): Promise<{ publicOutput: NodeContent }> => {
    // the current x positon 
    assert(witness.lefts.length == witness.path.length, "The path length and left array do not match")
    let rootComm = userLeaf.commitment
    let rootHash = userLeaf.hash
    for (let index = 0; index < 32; index++) {
        const leftComm = Provable.if(witness.lefts[index]!, witness.path[index]!.commitment, rootComm);
        const rightComm = Provable.if(witness.lefts[index]!, rootComm, witness.path[index]!.commitment);
        const newComm = leftComm.add(rightComm)
        const leftHash = Provable.if(witness.lefts[index]!, witness.path[index]!.hash, rootHash)
        const rightHash = Provable.if(witness.lefts[index]!, rootHash, witness.path[index]!.hash)
        const newHash = Poseidon.hash([...leftComm.toFields(), ...rightComm.toFields(), leftHash, rightHash])
        rootHash = Provable.if(newComm.equals(rootComm), rootHash, newHash)
        rootComm = newComm
    }
    return { publicOutput: new NodeContent({ commitment: rootComm, hash: rootHash }) }
}

export const computeRootProgram = ZkProgram({
    name: 'Compute Root',
    publicInput: MerkleWitness,
    publicOutput: NodeContent,
    methods: {
        computeRoot: {
            privateInputs: [NodeContent],
            method: computeRoot
        }
    }
})

export class computeRootProof extends ZkProgram.Proof(computeRootProgram) { }