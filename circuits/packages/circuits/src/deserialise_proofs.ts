import { Bool, Field, Group, UInt64 } from "o1js";
import { NodeContent, MerkleWitness as CircuitMerkleWitness } from "./types";
import { readFileSync } from 'fs';
interface NodeData {
    commitment: Array<number>;
    hash: Array<number>,
}

interface MerkleWitness {
    path: NodeData[],
    lefts: boolean[]
    user_leaf: NodeData
    root: NodeData
}


export class DeserialiseProofs {

    static readProof(file: string): [CircuitMerkleWitness, NodeContent, NodeContent] {
        const jsonData = readFileSync(file, "utf-8")
        const data: MerkleWitness = JSON.parse(jsonData)
        let path = data.path.map((sibling) => this.nodeContentFromNode(sibling))
        // Add zeroes to the front
        for (let index = path.length; index < 32; index++) {
            path.push(NodeContent.zero())
        }
        const userLeaf = this.nodeContentFromNode(data.user_leaf)
        const root = this.nodeContentFromNode(data.root)
        const lefts = data.lefts.map((left) => Bool.fromValue(left))
        for (let index = lefts.length; index < 32; index++) {
            lefts.push(Bool(false))
        }
        return [new CircuitMerkleWitness({ path, lefts }), userLeaf, root]
    }


    static nodeContentFromNode({ commitment, hash }: NodeData): NodeContent {
        const groupCommitment = groupFromCommitment(commitment)
        const hashField = Field.from(leBytesToBigint(hash))
        return new NodeContent({ commitment: groupCommitment, hash: hashField })
    }

}
/**
 * 
 * @param commitment The commitment serialized array
 * @returns Deserialise the group element
 */
function groupFromCommitment(commitment: Array<number>): Group {
    if (commitment.length != 65) {
        throw Error("Invalid commitment bytes there should be exactly 65 bytes")
    }
    /// @dev these arrays are in little endian format as seen in the ser buffer of in arkworks-serialize
    /// https://github.com/arkworks-rs/algebra/blob/9ce33e6ef1368a0f5b01b91e6df5bc5877129f30/ff/src/const_helpers.rs#L148
    const x = leBytesToBigint(commitment.slice(0, 32))
    const y = leBytesToBigint(commitment.slice(32, 64))
    const yflags = commitment[64]

    if (yflags == 64) {
        return Group.zero
    }
    return yflags == 128 ? Group({ x, y }).neg() : Group({ x, y })
}
/**
 * A wrapper function around the Big endian function to support little endian format
 * @param arr The array in little endian format
 * @returns 
 */
const leBytesToBigint = (arr: number[]): bigint => beBytesToBigint(new Uint8Array(arr.reverse()))

/**
 * @param buf The uint 8 buffer array in big endian format
 * @returns Bigint representing the array
 */
function beBytesToBigint(buf: Uint8Array): bigint {
    let bits = 8n
    if (ArrayBuffer.isView(buf)) {
        bits = BigInt(buf.BYTES_PER_ELEMENT * 8)
    } else {
        buf = new Uint8Array(buf)
    }

    let ret = 0n
    for (const i of buf.values()) {
        const bi = BigInt(i)
        ret = (ret << bits) + bi
    }
    return ret
}
