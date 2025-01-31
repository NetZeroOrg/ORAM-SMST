import { assert } from "console"
import { DeserialiseProofs } from "../src/deserialise_proofs"
import * as fs from "fs"
import path from "path"
//TODO: do better testing
const BASE_DIR = "/Users/utkarshdagoat/dev/NetZero/ORAM-SMST/"
describe("Deserialization test", () => {
    it("Should deserialize", async () => {
        const PROOF_DIR = BASE_DIR + 'proofs'
        if (!fs.existsSync(PROOF_DIR)) {
            assert(false, `Proof directory ${PROOF_DIR} does not exists`)
        }
        const anyFile = fs.readdirSync(PROOF_DIR).filter((file) => fs.statSync(path.join(PROOF_DIR, file)).isFile())[0];
        if (anyFile == undefined) {
            assert(false, "No file in the Proof directory");
        } else {
            const file = path.join(PROOF_DIR, anyFile);
            const merkleWitness = DeserialiseProofs.getProofPath(file)
        }
    })
})

