import { getPlugin } from './plugin.ts'
import { State } from './gen/types.ts'
import { delay } from 'https://deno.land/std@0.161.0/async/mod.ts'
import { io, redis } from './server.ts'

const promisify = <T>(f: () => T): Promise<T> => {
  return new Promise((resolve, reject) => {
    try {
      resolve(f())
    } catch (e) {
      reject(e)
    }
  })
}

export interface Room {
  plugin: string
  state: State
}
export const rooms: Record<string, Room> = {}

export interface IRoomService {
  existRoom(roomId: string): Promise<boolean>
  createRoom(playerId: string, roomId: string): Promise<Room>
  joinPlayer(playerId: string, roomId: string): Promise<Room>
  leavePlayer(playerId: string, roomId: string): Promise<Room>
  cancelTask(roomId: string, taskId: string): Promise<Room>
  reserveTask(playerId: string, roomId: string): Promise<void>
  onTask(roomId: string, taskId: string): Promise<Room>
  rpc(playerId: string, roomId: string, action: unknown): Promise<Room>
}

export class RoomService implements IRoomService {
  async existRoom(roomId: string): Promise<boolean> {
    return Object.keys(rooms).includes(roomId)
  }

  async createRoom(
    playerId: string,
    roomId: string,
    plugin: string
  ): Promise<Room> {
    const runtime = getPlugin(plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${plugin} not found`)
    }
    const state = runtime.onCreateRoom?.(playerId, roomId)
    if (!state) {
      return Promise.reject('[createRoom] state is null')
    }
    rooms[roomId] = {
      plugin,
      state,
    }
    return rooms[roomId]
  }

  async joinPlayer(playerId: string, roomId: string): Promise<Room> {
    const room = rooms[roomId]
    if (!room) {
      return Promise.reject(`room ${roomId} is null`)
    }
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }
    const state = runtime.onJoinPlayer?.(playerId, roomId, room.state)
    if (!state) {
      return Promise.reject('[joinPlayer] state is null')
    }
    room.state = state
    return Promise.resolve(room)
  }

  async leavePlayer(playerId: string, roomId: string): Promise<Room> {
    const room = rooms[roomId]
    console.log(`[leave] room ${roomId}, id=${playerId}`)
    if (!room) {
      return Promise.reject(`room ${roomId} is null`)
    }
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }
    const state = runtime.onLeavePlayer?.(playerId, roomId, room.state)
    if (!state) {
      return Promise.reject('[leavePlayer] state is null')
    }
    room.state = state
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

    const room = rooms[roomId]
    if (!room) {
      console.log(Object.keys(rooms))
      return Promise.reject(`[onTask] room "${roomId}" is null`)
    }
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }
    if (!runtime.onTask) {
      return room
    }
    const state = await promisify(() =>
      runtime.onCancelTask!(taskId, room.state)
    ).catch((e) => console.error(e))
    if (!state) {
      return Promise.reject('[onTask] state is null')
    }
    console.log(`state(${state.data.length} B)`)
    room.state = state
    return room
  }

  async onTask(roomId: string, taskId: string): Promise<Room> {
    const room = rooms[roomId]
    if (!room) {
      console.log(Object.keys(rooms))
      return Promise.reject(`[onTask] room "${roomId}" is null`)
    }
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }
    if (!runtime.onTask) {
      return room
    }
    const state = await promisify(() =>
      runtime.onTask!(taskId, room.state)
    ).catch((e) => console.error(e))
    if (!state) {
      return Promise.reject('[onTask] state is null')
    }
    console.log(`state(${state.data.length} B)`)
    room.state = state
    return room
  }

  async rpc(playerId: string, roomId: string, action: unknown): Promise<Room> {
    const room = rooms[roomId]
    if (!room) {
      return Promise.reject(`room ${roomId} is null`)
    }
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }
    if (!runtime.rpc) {
      return room
    }
    const state = await promisify(() =>
      runtime.rpc!(playerId, roomId, room.state, JSON.stringify(action))
    ).catch((e) => console.error(e))
    if (!state) {
      return Promise.reject('state is null')
    }
    console.log(`state(${state.data.length} B)`)
    room.state = state
    return room
  }
}
