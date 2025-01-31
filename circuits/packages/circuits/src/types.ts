import { Field, Group, Provable, Struct } from "o1js";

export class NodeContent extends Struct({
    commitment: Group,
    hash: Field
}) { }

/**
 * @param path: We assume that the max height of tree is 32 we can increase this if we want
 * @param root: The root for the tree
 */
export class MerkleWitness extends Struct({
    path: Provable.Array(NodeContent, 32),
    root: NodeContent
}) { }
