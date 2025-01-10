import { Field, Poseidon } from "o1js";

/**
 * The MSTNode class represents a node in the Merkle Sum Tree. The node can be a leaf node or an internal node.
 * @member hash : the hash for the node
 */
export class MSTNode {
    public hash: Field
    public balances: Field[]

    constructor(hash: Field, balances: Field[]) {
        this.hash = hash;
        this.balances = balances;
    }

    /**
     * create a leaf node for the Merkle Sum Tree given the balances and username. The hash is calculated as
     * `hash = Poseidon(username, balances[0], balances[1], ..., balances[n])`
     * 
     * @remarks The balances and username are fetched from the database see the `buildTree` function
     * 
     * @param username username of the user in database
     * @param balances balances of user for the number of tokens
     * @returns a leaf node
     */
    static leaf(username: string, balances: bigint[]): MSTNode {
        let balancesField = balances.map((balance) => Field(balance));
        let hash = Poseidon.hash([Field(username), ...balancesField]);
        return new MSTNode(hash, balancesField);
    }

    /**
     * creates a internal node for the Merkle Sum Tree given the left and right child. The hash is calculated as
     * `hash = Poseidon(left_child.hash, right_child.hash, left_child.balances[0], right_child.balances[0], ..., left_child.balances[n] , right_child.balances[n])`
     * @param left_child the left child for this internal node
     * @param right_child the right child foir the internal node
     * @returns an internal node with the hash and balances 
     */
    static internal_node(left_child: MSTNode, right_child: MSTNode): MSTNode {
        let netBalances = left_child.balances.map((balance, index) => balance.add(right_child.balances[index]));
        let hash = Poseidon.hash([left_child.hash, right_child.hash, ...netBalances]);
        return new MSTNode(hash, netBalances);
    }
}