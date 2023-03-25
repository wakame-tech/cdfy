import { useEffect, useState } from 'react'
import { socket } from './socket'

export interface Room {
  players: Set<string>
  state: State
}

export interface State {
  data: string
}

// const pluginId = 'counter'
// export const pluginId: string = 'career_poker'
export const pluginId: string = 'counter'

export const usePlugin = <S, R>(roomId: string | null) => {
  const [state, setState] = useState<S | null>(null)

  useEffect(() => {
    if (!state) {
      socket.emit('join', roomId, pluginId)
    }
    console.debug(`emit join as ${socket.id}`)

    socket.on('update', (room: Room | null) => {
      if (!room) {
        throw 'room is null'
      }
      const data: S = JSON.parse(room.state.data)
      setState(data)
    })
    socket.on('error', (e: string) => {
      console.error(e)
      alert(e)
    })

    return () => {
      socket.off('update')
      socket.off('error')
    }
  }, [])

  const rpc = (value: R) => {
    socket.emit('rpc', roomId, socket.id, value)
  }

  return {
    id: socket.id,
    state,
    setState,
    roomId,
    rpc,
  }
}
