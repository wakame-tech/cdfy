import { createRuntime, Exports, Imports } from './gen/index.ts'
import { RoomService } from './room.ts'
import { io } from './server.ts'

const plugins: Record<string, Exports> = {}

export const registerPluginFromLocal = async (path: string): Promise<void> => {
  const imports: Imports = {
    rand() {
      return Math.floor(Math.random() * Math.pow(2, 32))
    },
    debug(message: string) {
      console.log(message)
    },
    cancel(roomId: string, taskId: string) {
      const usecase = new RoomService()
      const room = usecase.cancelTask(roomId, taskId)
      io.to(roomId).emit('update', room)
    },
    reserve(playerId: string, roomId: string, action: string, timeout: number) {
      const taskId = crypto.randomUUID()
      const usecase = new RoomService()
      usecase.reserveTask(playerId, roomId, taskId, action, timeout)
      return taskId
    },
  }
  const plugin = await Deno.readFile(path)
  const runtime = await createRuntime(plugin, imports)
  const meta = runtime.pluginMeta?.()!
  console.log(
    `installed ${meta.name} v${meta.version} @ ${path} (${
      plugin.byteLength / 1000
    } KB)`
  )
  plugins[meta.name] = runtime
}

export const getPlugin = (name: string): Exports | undefined => {
  return plugins[name]
}
