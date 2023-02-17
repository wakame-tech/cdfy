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
  // const plugin = 'career-poker'
  const pluginId = 'career-poker'
  const [plugin, setPlugin] = useState(pluginId)
  const [state, setState] = useState<S | null>(null)

  useEffect(() => {
    // const plugin = 'career-poker'
    socket.emit('join', roomId, plugin)
    socket.on('update', (room: Room) => {
      console.log(`id = ${socket.id}`)
      console.log(room)
      const data: S = JSON.parse(room.state.data)
      console.log(data)
      setState(data)
    })
    return () => {
      socket.off('join')
      socket.off('rpc')
    }
  }, [])

  const rpc = (value: R) => {
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
