import io from 'socket.io-client'

const endpoint = import.meta.env.VITE_ENDPOINT
export const socket = io(endpoint, {
  timeout: 5000,
  // transports: ['websocket'],
  // upgrade: false,
})
