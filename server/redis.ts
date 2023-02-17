import { connect } from 'https://deno.land/x/redis/mod.ts'

const hostname = Deno.env.get('REDIS_HOST')!
const port = Deno.env.get('REDIS_PORT')!
const password = Deno.env.get('REDIS_PASSWORD')!

console.log({ hostname, port })

const redis = await connect({
  hostname,
  port,
  password,
  tls: true,
}).catch((e) => console.error(e))
if (!redis) {
  Deno.exit()
}

await redis.set('hoge', 1)
const res = await redis.get('hoge')
console.log(res)
