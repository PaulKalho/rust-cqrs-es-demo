import { z } from "zod"

const schema = z.object({
    NODE_ENV: z.literal("production").or(z.literal("development")).default("production"),
    NEXT_PUBLIC_API_URL: z.string().url().default("http://localhost:3030/"),
}).transform(obj => ({
    env: obj.NODE_ENV,
    apiUrl: obj.NEXT_PUBLIC_API_URL,
}))

function getConfig() {
    const conf = schema.safeParse({
        NODE_ENV: process.env.NODE_ENV,
        NEXT_PUBLIC_API_URL: process.env.NEXT_PUBLIC_API_URL,
    })

    if (!conf.success) {
        throw new Error(`Invalid environment variables:${conf.error}`)
    } else {
        return Object.assign(conf.data)
    }
}

export {
    getConfig
}
