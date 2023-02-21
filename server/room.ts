import { getPlugin } from './plugin.ts'
import { State } from './gen/types.ts'
import { delay } from 'https://deno.land/std@0.161.0/async/mod.ts'
import { Lock } from 'https://deno.land/x/async_lock/mod.ts'
import { io, redis } from './server.ts'
import { expect, unwrap } from './util.ts'

export interface Room {
  plugin: string
  state: State | null
}

export const rooms: Record<string, Room> = {}

const lock = new Lock()

export interface IRoomService {
  existRoom(roomId: string): boolean
  createRoom(playerId: string, roomId: string, plugin: string): Promise<Room>
  joinPlayer(playerId: string, roomId: string): Promise<Room>
  leavePlayer(playerId: string, roomId: string): Promise<Room>
  cancelTask(roomId: string, taskId: string): Promise<Room>
  reserveTask(
    playerId: string,
    roomId: string,
    taskId: string,
    action: string,
    timeout: number
  ): Promise<void>
  onTask(roomId: string, taskId: string): Promise<Room>
  rpc(playerId: string, roomId: string, action: unknown): Promise<Room>
}

export class RoomService implements IRoomService {
  existRoom(roomId: string): boolean {
    return Object.keys(rooms).includes(roomId)
  }

  async createRoom(
    playerId: string,
    roomId: string,
    plugin: string
  ): Promise<Room> {
    const runtime = await expect(
      () => getPlugin(plugin),
      `plugin ${plugin} not found`
    )
    const onCreateRoom = runtime.onCreateRoom

    await lock.run(async () => {
      if (onCreateRoom) {
        const state = await unwrap(() => onCreateRoom(playerId, roomId))
        rooms[roomId] = {
          plugin,
          state,
        }
      } else {
        rooms[roomId] = {
          plugin,
          state: null,
        }
      }
    })
    return rooms[roomId]
  }

  async joinPlayer(playerId: string, roomId: string): Promise<Room> {
    const room = await expect(() => rooms[roomId], `room ${roomId} is null`)
    const runtime = await expect(
      () => getPlugin(room.plugin),
      `plugin ${room.plugin} not found`
    )
    const onJoinPlayer = runtime.onJoinPlayer
    if (!room.state) {
      throw 'room state is null'
    }
    if (onJoinPlayer) {
      await lock.run(async () => {
        const state = await unwrap(() =>
          onJoinPlayer(playerId, roomId, room.state!)
        )
        room.state = state
      })
    }
    return room
  }

  async leavePlayer(playerId: string, roomId: string): Promise<Room> {
    const room = await expect(() => rooms[roomId], `room ${roomId} is null`)
    const runtime = await expect(
      () => getPlugin(room.plugin),
      `plugin ${room.plugin} not found`
    )
    const onLeavePlayer = runtime.onLeavePlayer
    if (!room.state) {
      throw 'room state is null'
    }
    if (onLeavePlayer) {
      await lock.run(async () => {
        const state = await unwrap(() =>
          onLeavePlayer(playerId, roomId, room.state!)
        )
        room.state = state
      })
    }
    return room
  }

  async reserveTask(
    playerId: string,
    roomId: string,
    taskId: string,
    action: string,
    timeout: number
  ): Promise<void> {
    const usecase = new RoomService()
    await redis.set(taskId, action)
    await delay(timeout)

    const res = await redis.get(taskId)
    console.log(`task=${taskId} res=${res}`)
    if (!res) {
      console.log(`task=${taskId} action not found`)
      return
    }
    const value: unknown = JSON.parse(res)
    console.log(`[reserve] timeout=${timeout} id=${playerId} action=${value}`)
    await usecase
      .rpc(playerId, roomId.toString(), value)
      .catch((e) => console.error(e))
    const room = await usecase
      .onTask(roomId.toString(), taskId)
      .catch((e) => console.error(e))
    io.to(roomId).emit('update', room)
  }

  async cancelTask(roomId: string, taskId: string): Promise<Room> {
    const res = await redis.del(taskId)
    console.log(`task=${taskId} delete ${res}`)

    const room = await expect(() => rooms[roomId], `room ${roomId} is null`)
    const runtime = await expect(
      () => getPlugin(room.plugin),
      `plugin ${room.plugin} not found`
    )
    const onCancelTask = runtime.onCancelTask
    if (!room.state) {
      throw 'room state is null'
    }
    if (onCancelTask) {
      const state = await unwrap(() => onCancelTask(taskId, room.state!))
      room.state = state
    }
    return room
  }

  async onTask(roomId: string, taskId: string): Promise<Room> {
    const room = await expect(() => rooms[roomId], `room ${roomId} is null`)
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }

    const onTask = runtime.onTask
    if (!room.state) {
      throw 'room state is null'
    }
    if (onTask) {
      const state = await unwrap(() => onTask(taskId, room.state!))
      room.state = state
    }
    return room
  }

  async rpc(playerId: string, roomId: string, action: unknown): Promise<Room> {
    const room = await expect(() => rooms[roomId], `room ${roomId} is null`)
    const runtime = await expect(
      () => getPlugin(room.plugin),
      `plugin ${room.plugin} not found`
    )

    const rpc = runtime.rpc
    if (!room.state) {
      throw 'room state is null'
    }
    if (rpc) {
      await lock.run(async () => {
        const state = await unwrap(() =>
          rpc(playerId, roomId, room.state!, JSON.stringify(action))
        )
        room.state = state
      })
    }
    return room
  }
}
