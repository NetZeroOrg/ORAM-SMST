import { MSTError } from "@/error";
import { MSTNode } from "@/node";
import { Curreny } from "@/types";

/**
 * In memory database for Merkle Sum Tree
 * All operations on merkle sum tree are atomic i.e first the operations are performed on the in memory database and then the database is updated
 * the database can be chosen from the following options
 * 1. PostgesSQL
 * 2. MongoDB
 * 3. Redis
 */
export class InMemoryDb {

    nodes: MSTNode[][]
    root: MSTNode
    height: number
    currencies: Curreny[]

    constructor(nodes: MSTNode[][], root: MSTNode, height: number, currencies: Curreny[]) {
        this.nodes = nodes
        this.root = root
        this.height = height
        this.currencies = currencies
    }

    public leaves = async (): Promise<MSTNode[] | Error> => {
        let leaves = this.nodes[this.height]
        if (leaves == undefined) {
            return new MSTError({
                name: "MST_NOT_INITIALISED",
                message: "MST is not initialised leaves are undefined",
                cause: "Leaves comes out undefined"
            })
        }
        return leaves
    }

}
