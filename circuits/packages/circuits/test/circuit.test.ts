import { computeRoot, computeRootProof, computeRootProgram } from "../src/circuit"
import { DeserialiseProofs } from "../src/deserialise_proofs"
describe("computes root correctly", () => {
    it("should compile ,generate and verify proof", async () => {
        const { } = await computeRootProgram.compile()
        const [merkleWitness, userLeaf, root] = DeserialiseProofs.readProof('./sample/test_proof.json')
        const { proof } = await computeRootProgram.computeRoot(merkleWitness, userLeaf)
        console.log("Proof Generated!")
        proof.verify()
        console.log("Proof Verified")
        expect(proof.publicOutput).toBe(root)
    })
})