import { computeRootProgram } from "../src/circuit"
import { DeserialiseProofs } from "../src/deserialise_proofs"
describe("computes root correctly", () => {
    it("should compile ,generate and verify proof", async () => {
        const { } = await computeRootProgram.compile()
        const [merkleWitness, userLeaf, root] = DeserialiseProofs.readProof('./sample/test_proof.json')
        const { proof } = await computeRootProgram.computeRoot(merkleWitness, userLeaf)
        proof.verify()
        expect(proof.publicOutput.commitment.equals(root.commitment).toBoolean()).toBe(true)
        expect(proof.publicOutput.hash.equals(root.hash).toBoolean()).toBe(true)
    })
})