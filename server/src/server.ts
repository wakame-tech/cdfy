import { serve } from './deps.ts'
import { io } from './socket.ts'

await serve(io.handler(), {
  port: 8080,
  onListen({ port, hostname }) {
    console.log(`Server started at http://${hostname}:${port}`)
  },
})
