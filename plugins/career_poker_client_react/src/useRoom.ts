import { useCallback, useState } from 'react'
import { Room, createRoom, fetchRoom, joinRoom, loadPlugin, sendMessage } from './api'

export const useRoom = <T>(roomId: string, userId: string) => {
  const [room, setRoom] = useState<Room>({
    room_id: roomId,
    users: [],
    states: {}
  })

  const create = useCallback(async () => {
    const room = await createRoom(roomId)
    setRoom(room)
  }, [])

  const fetch = useCallback(async () => {
    const room = await fetchRoom(roomId, userId)
    setRoom(room)
  }, [])

  const join = useCallback(async () => {
    const room = await joinRoom(roomId, userId);
    setRoom(room)
  }, [])

  const load = useCallback(async () => {
    const room = await loadPlugin(roomId);
    setRoom(room)
  }, [])

  const message = useCallback(async (message: T) => {
    const room = await sendMessage(roomId, userId, message)
    setRoom(room)
  }, [])

  return {
    room, create, fetch, join, load, message
  }
}
