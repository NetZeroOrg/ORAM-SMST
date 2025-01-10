type MST_ERROR = "MST_NOT_INITIALISED" | `NODE_NOT_FOUND_AT_HEIGHT_`

export class ErrorBase<T extends string> extends Error {
    name: T
    message: string
    cause: any

    constructor({
        name,
        message,
        cause
    }: {
        name: T,
        message: string,
        cause: any
    }) {
        super(message)
        this.name = name
        this.message = message
        this.cause = cause
    }
}

export class MSTError extends ErrorBase<MST_ERROR> { }