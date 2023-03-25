import { connect } from './deps.ts'

const hostname = Deno.env.get('REDIS_HOST')!
const port = Deno.env.get('REDIS_PORT')!
const password = Deno.env.get('REDIS_PASSWORD')!

console.log({ hostname, port })
export const redis = await connect({
  hostname,
  port,
  password,
})
console.log('connected to redis')
