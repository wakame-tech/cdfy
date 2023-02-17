import { Server } from 'https://deno.land/x/socket_io@0.1.1/mod.ts'
import { serve } from 'https://deno.land/std/http/server.ts'
import { connect } from 'https://deno.land/x/redis/mod.ts'
import { RoomService } from './room.ts'
import { registerPluginFromLocal } from './plugin.ts'

await registerPluginFromLocal('./counter.wasm')
await registerPluginFromLocal('./career_poker.wasm')

const hostname = Deno.env.get('REDIS_HOST')!
const port = Deno.env.get('REDIS_PORT')!
const password = Deno.env.get('REDIS_PASSWORD')!

console.log({ hostname, port })
export const redis = await connect({
  hostname,
  port,
  password,
})

console.log('connected to redis')

export const io = new Server({
  connectTimeout: 5000,
  cors: {
    origin: [Deno.env.get('ORIGIN')!],
    methods: ['GET', 'POST'],
  },
})

io.on('connection', (socket) => {
  const usecase = new RoomService()
  console.log(`[connect] id=${socket.id}`)

  socket.on('join', async (roomId: string, plugin: string) => {
    if (await usecase.existRoom(roomId)) {
      socket.join(roomId)
      const room = await usecase
        .joinPlayer(socket.id, roomId)
        .catch((e) => console.error(e))
      if (!room) {
        return
      }
      console.log(`[join] id==${socket.id} room=${roomId}`)
      io.to(roomId).emit('update', room)
    } else {
      socket.join(roomId)
      const room = await usecase
        .createRoom(socket.id, roomId, plugin)
        .catch((e) => console.error(e))
      if (!room) {
        return
      }
      console.log(
        `[create] id=${socket.id} room=${roomId} (plugin = ${room.plugin})`
      )
      io.to(roomId).emit('update', room)
    }
  })

  socket.on('rpc', async (roomId: string, value: unknown) => {
    console.log(
      `[rpc] id=${socket.id} room=${roomId}, value=${JSON.stringify(value)}`
    )
    const room = await usecase
      .rpc(socket.id, roomId.toString(), value)
      .catch((e) => console.error(e))
    io.to(roomId).emit('update', room)
  })

  socket.on('disconnecting', async () => {
    console.log(
      `[disconnect] id=${socket.id} rooms=[${Array.from(socket.rooms)}]`
    )
    for (const roomId of socket.rooms) {
      if (await usecase.existRoom(roomId.toString())) {
        const room = await usecase
          .leavePlayer(socket.id, roomId.toString())
          .catch((e) => console.error(e))
        if (room) {
          io.to(roomId).emit('update', room)
          socket.leave(roomId)
        }
      }
    }
  })
})

await serve(io.handler(), {
  port: 8080,
  onListen({ port, hostname }) {
    console.log(`Server started at http://${hostname}:${port}`)
  },
})
