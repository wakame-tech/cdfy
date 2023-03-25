/**
 * async task in plugin
 */
import { delay } from './deps.ts'
import { onCancelTask, onDidTask, rpc } from './plugin/runtime.ts'
import { redis } from './redis.ts'
import { getRoom, saveRoom } from './rooms.ts'
import { io } from './socket.ts'

export interface Task {
  id: string
  roomId: string
  // JSON
  action: string
  timeout: number
}

const _schedule = async (playerId: string, task: Task) => {
  await redis.set(task.id, task.action)

  await delay(task.timeout)

  const _actionStr = await redis.get(task.id)
  if (!_actionStr) {
    console.log(`task=${task.id} action not found`)
    return
  }
  const action = JSON.parse(_actionStr)
  console.log(
    `[reserved task] timeout=${
      task.timeout
    } id=${playerId} action=${JSON.stringify(action)}`
  )

  const room = await getRoom(task.roomId)
  if (!room) {
    throw 'room not found'
  }
  let newState = await rpc(
    room.instance,
    room.state,
    playerId,
    task.roomId,
    action
  )
  newState = await onDidTask(room.instance, newState, task.id)
  const newRoom = {
    ...room,
    state: newState,
  }
  await saveRoom(newRoom)
  io.to(room.id).emit('update', newRoom)
}

export const reserveTask = (
  taskId: string,
  playerId: string,
  roomId: string,
  action: string,
  timeout: number
): Task => {
  const task = {
    id: taskId,
    roomId,
    action,
    timeout,
  }
  _schedule(playerId, task)
  return task
}

export const cancelTask = async (
  taskId: string,
  roomId: string
): Promise<void> => {
  const room = await getRoom(roomId)
  if (!room) {
    throw 'room not found'
  }
  await redis.del(taskId)
  const newState = await onCancelTask(room.instance, room.state, taskId)
  const newRoom = {
    ...room,
    state: newState,
  }
  await saveRoom(newRoom)
  io.to(room.id).emit('update', newRoom)
}
