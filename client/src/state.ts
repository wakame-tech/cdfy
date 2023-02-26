import { useEffect, useState } from 'react'
import { socket } from './socket'

export interface Room {
  players: Set<string>
  state: State
}

export interface State {
  data: string
}

export const usePlugin = <S, R>(roomId: string | null) => {
  // const pluginId = 'counter'
  const pluginId = 'career_poker'
  const [plugin, setPlugin] = useState(pluginId)
  const [state, setState] = useState<S | null>(null)

  useEffect(() => {
    console.debug(`emit join as ${socket.id}`)
    socket.emit('join', roomId, plugin)
    socket.on('update', (room: Room | null) => {
      if (!room) {
        throw 'room is null'
      }
      console.debug(room)
      const data: S = JSON.parse(room.state.data)
      setState(data)
    })
    return () => {
      socket.off('update')
    }
  }, [])

  const rpc = (value: R) => {
    console.debug({ roomId, value })
    socket.emit('rpc', roomId, value)
  }

  return {
    id: socket.id,
    plugin,
    state,
    roomId,
    rpc,
  }
}
