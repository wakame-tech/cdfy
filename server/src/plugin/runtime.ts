import { createRuntime, ResultState, State } from '../deps.ts'
import type { Exports, Imports } from '../deps.ts'
import { cancelTask, reserveTask } from '../task.ts'

export const unwrap = (f: () => ResultState): Promise<State> => {
  return new Promise((resolve, reject) => {
    const t = f()
    if (!t['Ok']) {
      reject(t['Err'])
    } else {
      resolve(t['Ok'])
    }
  })
}

export const instantiate = (wasm: ArrayBuffer): Promise<Exports> => {
  const imports: Imports = {
    rand() {
      return Math.floor(Math.random() * Math.pow(2, 32))
    },
    debug(message: string) {
      console.log(`@plugin ${message}`)
    },
    cancel(roomId: string, taskId: string) {
      cancelTask(taskId, roomId)
    },
    reserve(
      playerId: string,
      roomId: string,
      action: string,
      timeout: number
    ): string {
      const taskId = crypto.randomUUID()
      const task = reserveTask(taskId, playerId, roomId, action, timeout)
      return task.id
    },
  }
  return createRuntime(wasm, imports)
}

export const onCreateRoom = (
  instance: Exports,
  playerId: string,
  roomId: string
): Promise<State> => {
  if (!instance.onCreateRoom) {
    throw 'onCreateRoom not found'
  }
  return unwrap(() => instance.onCreateRoom!(playerId, roomId))
}

export const onJoinPlayer = (
  instance: Exports,
  state: State,
  playerId: string,
  roomId: string
): Promise<State> => {
  if (!instance.onJoinPlayer) {
    return Promise.resolve(state)
  }
  return unwrap(() => instance.onJoinPlayer!(playerId, roomId, state))
}

export const onLeavePlayer = (
  instance: Exports,
  state: State,
  playerId: string,
  roomId: string
): Promise<State> => {
  if (!instance.onLeavePlayer) {
    return Promise.resolve(state)
  }
  return unwrap(() => instance.onLeavePlayer!(playerId, roomId, state))
}

export const onCancelTask = (
  instance: Exports,
  state: State,
  taskId: string
): Promise<State> => {
  if (!instance.onCancelTask) {
    return Promise.resolve(state)
  }
  return unwrap(() => instance.onCancelTask!(taskId, state))
}

export const onDidTask = (
  instance: Exports,
  state: State,
  taskId: string
): Promise<State> => {
  if (!instance.onTask) {
    return Promise.resolve(state)
  }
  return unwrap(() => instance.onTask!(taskId, state))
}

export const rpc = (
  instance: Exports,
  state: State,
  playerId: string,
  roomId: string,
  action: unknown
): Promise<State> => {
  if (!instance.rpc) {
    return Promise.resolve(state)
  }
  return unwrap(() =>
    instance.rpc!(playerId, roomId, state, JSON.stringify(action))
  )
}
