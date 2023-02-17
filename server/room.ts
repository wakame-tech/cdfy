import { getPlugin } from './plugin.ts'
import { State } from './gen/types.ts'

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
      return Promise.reject('state is null')
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
      return Promise.reject('state is null')
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
      return Promise.reject('state is null')
    }
    room.state = state
    return room
  }

  async onTask(roomId: string, taskId: string): Promise<Room> {
    const room = rooms[roomId]
    if (!room) {
      return Promise.reject(`room ${roomId} is null`)
    }
    const runtime = getPlugin(room.plugin)
    if (!runtime) {
      return Promise.reject(`plugin ${room.plugin} not found`)
    }
    const state = await new Promise<State | undefined>((resolve, reject) => {
      try {
        const state = runtime.onTask?.(taskId, room.state)
        resolve(state)
      } catch (e) {
        reject(e)
      }
    }).catch((e) => console.error(e))
    if (!state) {
      return Promise.reject('state is null')
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
    const state = await new Promise<State | undefined>((resolve, reject) => {
      try {
        const state = runtime.rpc?.(
          playerId,
          roomId,
          room.state,
          JSON.stringify(action)
        )
        resolve(state)
      } catch (e) {
        reject(e)
      }
    }).catch((e) => console.error(e))
    if (!state) {
      return Promise.reject('state is null')
    }
    console.log(`state(${state.data.length} B)`)
    room.state = state
    return room
  }
}
