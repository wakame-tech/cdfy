import { serve } from 'https://deno.land/std@0.150.0/http/server.ts'
import { io } from './socket.ts'

await serve(io.handler(), {
  port: 8080,
  onListen({ port, hostname }) {
    console.log(`Server started at http://${hostname}:${port}`)
  },
})
