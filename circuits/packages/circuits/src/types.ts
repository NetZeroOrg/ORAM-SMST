import { Bool, Field, Group, Provable, Struct, UInt64 } from "o1js";

export class NodeContent extends Struct({
    commitment: Group,
    hash: Field
}) {
    static zero = () => new NodeContent({ commitment: Group.zero, hash: Field.from(0) })
    isZero = () => this.commitment.isZero() && this.hash.equals(0)
}

/**
 * @param path: We assume that the max height of tree is 32 we can increase this if we want
 * @param root: The root for the tree
 */
export class MerkleWitness extends Struct({
    path: Provable.Array(NodeContent, 32),
    lefts: Provable.Array(Bool, 32)
}) { }
