import { Server } from 'https://deno.land/x/socket_io@0.1.1/mod.ts'
import { serve } from 'https://deno.land/std/http/server.ts'
import { RoomService } from './room.ts'
import { registerPluginFromLocal } from './plugin.ts'

await registerPluginFromLocal('./counter.wasm')
await registerPluginFromLocal('./career_poker.wasm')

const io = new Server({
  connectTimeout: 5000,
  cors: {
    origin: [Deno.env.get('DEV_URL')!, Deno.env.get('PROD_URL')!],
    methods: ['GET', 'POST'],
  },
})

io.on('connection', (socket) => {
  const usecase = new RoomService()
  console.log(`socket ${socket.id} connected`)

  socket.on('join', async (roomId) => {
    if (await usecase.existRoom(roomId)) {
      socket.join(roomId)
      const room = await usecase
        .joinPlayer(socket.id, roomId)
        .catch((e) => console.error(e))
      if (!room) {
        return
      }
      console.log(`[join] socket=${socket.id} joined room=${roomId}`)
      io.to(roomId).emit('update', room)
    } else {
      socket.join(roomId)
      // const plugin = 'counter'
      const plugin = 'career-poker'
      const room = await usecase
        .createRoom(socket.id, roomId, plugin)
        .catch((e) => console.error(e))
      if (!room) {
        return
      }
      console.log(`[create] room ${roomId} (plugin = ${room.plugin}) created`)
      io.to(roomId).emit('update', room)
    }
  })

  socket.on('action', async (roomId: string, id: string, value: unknown) => {
    console.log(
      `[action] socket=${
        socket.id
      } room=${roomId}, id=${id}, value=${JSON.stringify(value)}`
    )
    const room = await usecase
      .onClick(socket.id, roomId.toString(), id, value)
      .catch((e) => console.error(e))
    io.to(roomId).emit('update', room)
  })

  socket.on('disconnecting', async (reason) => {
    console.log(
      `[disconnect] socket=${socket.id} rooms=[${Array.from(socket.rooms)}]`
    )
    for (const roomId of socket.rooms) {
      if (await usecase.existRoom(roomId.toString())) {
        const room = await usecase.leavePlayer(socket.id, roomId.toString())
        console.log(`socket ${socket.id} disconnected due to ${reason}`)
        io.to(roomId).emit('update', room)
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
