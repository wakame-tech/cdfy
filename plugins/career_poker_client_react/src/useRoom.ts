import { useCallback, useEffect, useState } from 'react'
import { Room, createRoom, fetchRoom, joinRoom, loadPlugin, sendMessage } from './api'

const ORIGIN = `ws://localhost:1234`

export const useRoom = <T>(roomId: string, userId: string) => {
  const [room, setRoom] = useState<Room>({
    room_id: roomId,
    users: [],
    states: {}
  })

  useEffect(() => {
    const ws = new WebSocket(`${ORIGIN}/rooms/${roomId}/listen/${userId}`);
    ws.onmessage = e => {
      const newRoom = JSON.parse(e.data)
      console.log(newRoom)
      setRoom(newRoom)
    }
    ws.onclose = e => {
      console.log(e)
    }
    return () => {
      ws.close()
    }
  }, [])

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
