import { Field, Group } from "o1js";
import { NodeContent } from "./types";
import { readFileSync } from 'fs';
interface PathData {
    commitment: string;
    hash: string
}


export class DeserialiseProofs {
    static getProofPath(file: string): NodeContent[] {
        const json = readFileSync(file, 'utf-8')
        const data: PathData[] = JSON.parse(json)
        let nodeContent = data.map(({ commitment, hash }) => {
            let groupCommitment = DeserialiseProofs.groupFromCommitment(commitment)
            let hashField = Field.from(hash)
            return new NodeContent({ commitment: groupCommitment, hash: hashField })
        })
        return nodeContent
    }

    static groupFromCommitment(_commitment: string): Group {
        // commitment is base 10 string 
        const commitment = BigInt(_commitment).toString(16)
        if (commitment.length != 130) {
            throw new Error("Invalid deserilaized commitment size")
        }
        // The modulus bit size is 256 therefore we need 256 / 4 hex characters to extract X
        const x = '0x' + commitment.slice(0, 64)
        // in ark serilize we add a 0 an empty flag at the end of x thus y will begin with the 67th character
        const y = '0x' + commitment.slice(64, 128)
        const yflags = Number(commitment.slice(128))

        if (yflags == 64) {
            return Group.zero
        }
        return Group({ x, y })
    }
}