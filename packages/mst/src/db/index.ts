export enum PersistentDatabase {
    PostgreSQL,
    MongoDB,
    Redis
}

export * from "./inMemory"
export * from "./postgres"
export * from "./mongo"
export * from "./redis"