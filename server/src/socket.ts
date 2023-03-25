import { Server } from './deps.ts'
import { Room, existRoom, getRoom, saveRoom } from './rooms.ts'
import { getPluginPackageFromLocal } from './plugin/registry.ts'
import {
  instantiate,
  onJoinPlayer,
  onLeavePlayer,
  rpc,
} from './plugin/runtime.ts'
import { onCreateRoom } from './plugin/runtime.ts'

const origin = Deno.env.get('ORIGIN')!
console.log(origin)

export const io = new Server({
  connectTimeout: 5000,
  cors: {
    origin: [Deno.env.get('ORIGIN')!],
    methods: ['GET', 'POST'],
  },
})

const onJoinEvent = async (
  roomId: string,
  playerId: string,
  plugin: string
): Promise<Room> => {
  if (await existRoom(roomId)) {
    console.log(`[join] player==${playerId} room=${roomId}`)
    const room = await getRoom(roomId)
    if (!room) {
      throw 'room not found'
    }
    const newRoom = {
      ...room,
      state: await onJoinPlayer(room.instance, room.state, playerId, room.id),
    }
    await saveRoom(newRoom)
    return newRoom
  } else {
    console.log(
      `[create] player=${playerId} room=${roomId} (plugin = ${plugin})`
    )
    const pkg = await getPluginPackageFromLocal('./counter.wasm')
    const instance = await instantiate(pkg.wasm)
    const state = await onCreateRoom(instance, playerId, roomId)
    const room: Room = {
      id: roomId,
      instance,
      state,
    }
    await saveRoom(room)
    return room
  }
}

const onDisconnectEvent = async (
  roomId: string,
  playerId: string
): Promise<Room> => {
  const room = await getRoom(roomId)
  if (!room) {
    throw 'room not found'
  }
  const newRoom = {
    ...room,
    state: await onLeavePlayer(room.instance, room.state, playerId, roomId),
  }
  await saveRoom(newRoom)
  return newRoom
}

const onRpcEvent = async (
  roomId: string,
  playerId: string,
  action: unknown
): Promise<Room> => {
  console.log(
    `[rpc] id=${playerId} room=${roomId}, value=${JSON.stringify(action)}`
  )
  const room = await getRoom(roomId)
  if (!room) {
    throw 'room not found'
  }
  const newRoom = {
    ...room,
    state: await rpc(room.instance, room.state, playerId, room.id, action),
  }
  await saveRoom(newRoom)
  return newRoom
}

io.on('connection', (socket) => {
  let currentRoomId: string
  console.log(`[connect] id=${socket.id}`)

  socket.on('join', async (roomId: string, plugin: string) => {
    socket.join(roomId)
    currentRoomId = roomId
    const newRoom = await onJoinEvent(roomId, socket.id, plugin).catch((e) => {
      console.error(e)
      socket.emit('error', e)
    })
    io.to(roomId).emit('update', newRoom)
  })

  socket.on(
    'rpc',
    async (roomId: string, playerId: string, action: unknown) => {
      const newRoom = await onRpcEvent(roomId, playerId, action).catch((e) => {
        console.error(e)
        socket.emit('error', e)
      })
      io.to(roomId).emit('update', newRoom)
    }
  )

  socket.on('disconnecting', async () => {
    console.log(
      `[disconnect] id=${socket.id} rooms=[${Array.from(socket.rooms)}]`
    )

    for (const roomId of socket.rooms) {
      socket.leave(roomId)
      const newRoom = await onDisconnectEvent(currentRoomId, socket.id).catch(
        (e) => {
          console.error(e)
          socket.emit('error', e)
        }
      )
      io.to(roomId).emit('update', newRoom)
    }
  })
})
