/**
 * async task in plugin
 */
import { delay } from '../deps.ts'
import { redis } from './redis.ts'
import { IRoomRepository } from '../rooms.ts'
import { Runtime } from './runtime.ts'

export interface Task {
  id: string
  roomId: string
  playerId: string
  // JSON
  action: string
  timeout: number
}

export interface TaskRunner {
  reserve(task: Task): void
  execute(task: Task): Promise<void>
}

export class RedisTaskRunner implements TaskRunner {
  constructor(private roomRepository: IRoomRepository) {}
  reserve(task: Task) {
    !(async () => {
      await redis.setex(task.id, task.timeout / 1000, JSON.stringify(task))
      await delay(task.timeout)
      const taskStr = await redis.get(task.id)
      if (!taskStr) {
        console.log(`task=${task.id} action not found`)
        return
      }
      const delayedTask: Task = JSON.parse(taskStr)
      this.execute(delayedTask)
    })()
  }

  async execute(task: Task): Promise<void> {
    const action: unknown = JSON.parse(task.action)
    console.log(
      `[reserved task] timeout=${task.timeout} id=${
        task.playerId
      } action=${JSON.stringify(action)}`
    )

    const room = await this.roomRepository.get(task.roomId)
    if (!room) {
      throw 'room not found'
    }
    const runtime = new Runtime(room.instance)
    let newState = await runtime.rpc(
      room.state,
      task.playerId,
      task.roomId,
      action
    )
    newState = await runtime.onDidTask(newState, task.id)
    const newRoom = {
      ...room,
      state: newState,
    }
    await this.roomRepository.save(newRoom)
  }
}
