import { Exports, State } from './deps.ts'

const rooms: Record<string, Room> = {}

export interface Room {
  id: string
  instance: Exports
  state: State
}

export const existRoom = (id: string): Promise<boolean> => {
  return Promise.resolve(Object.keys(rooms).includes(id))
}

export const getRoom = (id: string): Promise<Room | null> => {
  return Promise.resolve(rooms[id])
}

export const saveRoom = async (room: Room): Promise<void> => {
  rooms[room.id] = room
}
