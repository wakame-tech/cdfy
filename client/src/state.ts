import { useEffect, useState } from 'react'
import { socket } from './socket'

export interface Room {
  players: Set<string>
  state: State
}

export interface State {
  data: string
}

export const useGameState = <T>(roomId: string | null) => {
  const [state, setState] = useState<T | null>(null)

  useEffect(() => {
    socket.emit('join', roomId)

    socket.on('update', (room: Room) => {
      console.log(`id = ${socket.id}`)
      console.log(room)
      const data: T = JSON.parse(room.state.data)
      console.log(data)
      setState(data)
    })
    return () => {
      socket.off('join')
      socket.off('action')
    }
  }, [])

  const action = (id: string) => {
    console.log(id)
    socket.emit('action', roomId, id)
  }

  return {
    state,
    roomId,
    action,
  }
}
