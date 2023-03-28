import { Server } from './deps.ts'
import { Room, LocalRoomRepository, IRoomRepository } from './rooms.ts'
import {
  PluginRegistrarService,
  registerLocalPackages,
} from './registrar/PluginRegistrarService.ts'
import { instantiate, Runtime } from './runtime/runtime.ts'
import { RedisTaskRunner } from './runtime/task.ts'
import { LocalPluginMetaRepository } from './registrar/PluginMetaRepository.ts'
import { LocalWasmStorage } from './registrar/wasmStorage.ts'

export const io = new Server({
  connectTimeout: 5000,
  cors: {
    origin: [Deno.env.get('ORIGIN')!],
    methods: ['GET', 'POST'],
  },
})

export interface IRoomEventListener {
  onJoinEvent(roomId: string, playerId: string, pluginId: string): Promise<void>
  onDisconnectEvent(roomId: string, playerId: string): Promise<void>
  onRpcEvent(roomId: string, playerId: string, action: unknown): Promise<void>
}

export class RoomEventListerner implements IRoomEventListener {
  runtime: Runtime | null

  constructor(
    private pluginRegistrarService: PluginRegistrarService,
    private roomRepository: IRoomRepository
  ) {
    this.runtime = null
  }

  async onJoinEvent(
    roomId: string,
    playerId: string,
    pluginId: string
  ): Promise<void> {
    if (await this.roomRepository.exist(roomId)) {
      console.log(`[join] player==${playerId} room=${roomId}`)
      const room = await this.roomRepository.get(roomId)
      if (!room || !this.runtime) {
        throw 'room not found'
      }
      if (room.players.has(playerId)) {
        return
      }
      const newRoom: Room = {
        ...room,
        players: room.players.add(playerId),
        state: await this.runtime.onJoinPlayer(room.state, playerId, room.id),
      }
      console.debug(room)
      await this.roomRepository.save(newRoom)
    } else {
      console.log(
        `[create] player=${playerId} room=${roomId} (plugin = ${pluginId})`
      )
      const pkg = await this.pluginRegistrarService.fetchPackage(pluginId)
      console.debug(`package ${pluginId} found`)
      const exports = await instantiate(
        pkg.wasm,
        new RedisTaskRunner(this.roomRepository)
      )
      this.runtime = new Runtime(exports)
      console.debug(`package ${pluginId} succesfully instantiated`)

      const state = await this.runtime.onCreateRoom(playerId, roomId)
      const room: Room = {
        id: roomId,
        players: new Set([playerId]),
        instance: exports,
        state,
      }
      console.debug(room)
      await this.roomRepository.save(room)
    }
  }

  async onDisconnectEvent(roomId: string, playerId: string): Promise<void> {
    const room = await this.roomRepository.get(roomId)
    if (!room || !this.runtime) {
      throw 'room not found'
    }
    room.players.delete(playerId)
    const newRoom: Room = {
      ...room,
      state: await this.runtime.onLeavePlayer(room.state, playerId, roomId),
    }
    console.debug(room)
    await this.roomRepository.save(newRoom)
  }

  async onRpcEvent(
    roomId: string,
    playerId: string,
    action: unknown
  ): Promise<void> {
    console.log(
      `[rpc] id=${playerId} room=${roomId}, value=${JSON.stringify(action)}`
    )
    const room = await this.roomRepository.get(roomId)
    if (!room || !this.runtime) {
      throw 'room not found'
    }
    const newRoom = {
      ...room,
      state: await this.runtime.rpc(room.state, playerId, room.id, action),
    }
    console.debug(newRoom)
    await this.roomRepository.save(newRoom)
  }
}

const pluginMetaRepository = new LocalPluginMetaRepository()
await registerLocalPackages(pluginMetaRepository)

const listener = new RoomEventListerner(
  new PluginRegistrarService(pluginMetaRepository, new LocalWasmStorage()),
  new LocalRoomRepository((room) => io.to(room.id).emit('update', room))
)

io.on('connection', (socket) => {
  let currentRoomId: string
  console.log(`[connect] id=${socket.id}`)

  socket.on('join', async (roomId: string, plugin: string) => {
    socket.join(roomId)
    currentRoomId = roomId
    await listener.onJoinEvent(roomId, socket.id, plugin).catch((e) => {
      console.error(e)
      socket.emit('error', e)
    })
  })

  socket.on(
    'rpc',
    async (roomId: string, playerId: string, action: unknown) => {
      await listener.onRpcEvent(roomId, playerId, action).catch((e) => {
        console.error(e)
        socket.emit('error', e)
      })
    }
  )

  socket.on('disconnecting', async () => {
    console.log(
      `[disconnect] id=${socket.id} rooms=[${Array.from(socket.rooms)}]`
    )

    for (const roomId of socket.rooms) {
      socket.leave(roomId)
      await listener.onDisconnectEvent(currentRoomId, socket.id).catch((e) => {
        console.error(e)
        socket.emit('error', e)
      })
    }
  })
})
