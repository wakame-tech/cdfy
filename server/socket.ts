import { Server } from 'https://deno.land/x/socket_io@0.1.1/mod.ts'
import { RoomService } from './room.ts'

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
    socket.join(roomId)

    if (usecase.existRoom(roomId)) {
      const room = await usecase
        .joinPlayer(socket.id, roomId)
        .catch((e) => console.error(e))
      if (!room) {
        return
      }
      console.log(`[join] id==${socket.id} room=${roomId}`)
      io.to(roomId).emit('update', room)
    } else {
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
      .catch((e) => {
        console.error(e)
        socket.emit('error', e)
      })
    if (room) {
      io.to(roomId).emit('update', room)
    }
  })

  socket.on('disconnecting', async () => {
    console.log(
      `[disconnect] id=${socket.id} rooms=[${Array.from(socket.rooms)}]`
    )
    for (const roomId of socket.rooms) {
      if (usecase.existRoom(roomId.toString())) {
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
