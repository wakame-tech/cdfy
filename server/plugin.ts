import { createRuntime, Exports, Imports } from './gen/index.ts'
import { RoomService } from './room.ts'
import { io } from './socket.ts'

export const instantiate = (wasm: ArrayBuffer): Promise<Exports> => {
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
  return createRuntime(wasm, imports)
}
