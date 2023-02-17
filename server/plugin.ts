import { delay } from 'https://deno.land/std@0.161.0/async/mod.ts'
import { connect } from 'https://deno.land/x/redis/mod.ts'
import { createRuntime, Exports, Imports } from './gen/index.ts'
import { RoomService } from './room.ts'
import { io } from './server.ts'

const plugins: Record<string, Exports> = {}

const hostname = Deno.env.get('REDIS_HOST')!
const port = Deno.env.get('REDIS_PORT')!
const password = Deno.env.get('REDIS_PASSWORD')!

const redis = await connect({
  hostname,
  port,
  password,
  tls: true,
})

export const registerPluginFromLocal = async (path: string): Promise<void> => {
  const imports: Imports = {
    rand() {
      return Math.floor(Math.random() * Math.pow(2, 32))
    },
    cancel(taskId: string) {
      ;(async () => {
        const res = await redis.del(taskId)
        console.log(`task=${taskId} delete ${res}`)
      })()
    },
    reserve(playerId: string, roomId: string, action: string, timeout: number) {
      const taskId = crypto.randomUUID()
      ;(async () => {
        await redis.set(taskId, action)
        await delay(timeout)

        const res = await redis.get(taskId)
        console.log(`task=${taskId} res=${res}`)
        if (!res) {
          console.log(`task=${taskId} action not found`)
          return
        }
        const value: unknown = JSON.parse(res)
        console.log(
          `[reserve] timeout=${timeout} id=${playerId} action=${value}`
        )
        const usecase = new RoomService()
        await usecase
          .rpc(playerId, roomId.toString(), value)
          .catch((e) => console.error(e))
        const room = await usecase
          .onTask(roomId.toString(), taskId)
          .catch((e) => console.error(e))
        console.log(room)
        io.to(roomId).emit('update', room)
      })()
      return taskId
    },
  }
  const plugin = await Deno.readFile(path)
  const runtime = await createRuntime(plugin, imports)
  const meta = runtime.pluginMeta?.()!
  console.log(`installed ${meta.name} v${meta.version} @ ${path}`)
  plugins[meta.name] = runtime
}

export const getPlugin = (name: string): Exports | undefined => {
  return plugins[name]
}
