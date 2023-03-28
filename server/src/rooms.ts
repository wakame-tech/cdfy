import { Exports, State } from './deps.ts'

export interface Room {
  id: string
  players: Set<string>
  instance: Exports
  state: State
}

export interface IRoomRepository {
  exist(id: string): Promise<boolean>
  get(id: string): Promise<Room | null>
  save(room: Room): Promise<void>
}

export class LocalRoomRepository implements IRoomRepository {
  rooms: Record<string, Room> = {}

  constructor(private updateCallBack: (room: Room) => void) {
    this.rooms = {}
  }

  exist(id: string): Promise<boolean> {
    return Promise.resolve(Object.keys(this.rooms).includes(id))
  }

  get(id: string): Promise<Room | null> {
    return Promise.resolve(this.rooms[id])
  }

  save(room: Room): Promise<void> {
    this.rooms[room.id] = room
    console.debug(`update callback: ${JSON.stringify(room.state.data)}`)
    this.updateCallBack(room)
    return Promise.resolve()
  }
}
