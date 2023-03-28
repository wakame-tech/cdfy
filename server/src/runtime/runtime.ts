import { createRuntime, PluginMeta, ResultState, State } from '../deps.ts'
import type { Exports, Imports } from '../deps.ts'
import { RedisTaskRunner } from './task.ts'

const unwrap = (f: () => ResultState): Promise<State> => {
  return new Promise((resolve, reject) => {
    const t = f() as {
      ['Ok']: State
      ['Err']: string
    }
    if (!t['Ok']) {
      reject(t['Err'])
    } else {
      resolve(t['Ok'])
    }
  })
}

export const instantiate = (
  wasm: ArrayBuffer,
  taskRunner: RedisTaskRunner
): Promise<Exports> => {
  const imports: Imports = {
    rand() {
      return Math.floor(Math.random() * Math.pow(2, 32))
    },
    debug(message: string) {
      console.log(`@plugin ${message}`)
    },
    reserve(
      playerId: string,
      roomId: string,
      action: string,
      timeout: number
    ): string {
      const task = {
        id: crypto.randomUUID(),
        roomId,
        playerId,
        action,
        timeout,
      }
      taskRunner.reserve(task)
      return task.id
    },
  }
  return createRuntime(wasm, imports)
}

export const getMeta = async (
  wasm: ArrayBuffer
): Promise<PluginMeta | null> => {
  const instance = await createRuntime(wasm, {
    debug: function (_message: string): void {
      throw new Error('Function not implemented.')
    },
    rand: function (): number {
      throw new Error('Function not implemented.')
    },
    reserve: function (
      _playerId: string,
      _roomId: string,
      _action: string,
      _timeout: number
    ): string {
      throw new Error('Function not implemented.')
    },
  })
  return instance.pluginMeta?.()!
}

export class Runtime {
  constructor(private exports: Exports) {}

  onCreateRoom(playerId: string, roomId: string): Promise<State> {
    if (!this.exports.onCreateRoom) {
      throw 'onCreateRoom not found'
    }
    return unwrap(() => this.exports.onCreateRoom!(playerId, roomId))
  }

  onJoinPlayer(state: State, playerId: string, roomId: string): Promise<State> {
    if (!this.exports.onJoinPlayer) {
      return Promise.resolve(state)
    }
    return unwrap(() => this.exports.onJoinPlayer!(playerId, roomId, state))
  }

  onLeavePlayer(
    state: State,
    playerId: string,
    roomId: string
  ): Promise<State> {
    if (!this.exports.onLeavePlayer) {
      return Promise.resolve(state)
    }
    return unwrap(() => this.exports.onLeavePlayer!(playerId, roomId, state))
  }

  onDidTask(state: State, taskId: string): Promise<State> {
    if (!this.exports.onTask) {
      return Promise.resolve(state)
    }
    return unwrap(() => this.exports.onTask!(taskId, state))
  }

  rpc(
    state: State,
    playerId: string,
    roomId: string,
    action: unknown
  ): Promise<State> {
    if (!this.exports.rpc) {
      return Promise.resolve(state)
    }
    return unwrap(() =>
      this.exports.rpc!(playerId, roomId, state, JSON.stringify(action))
    )
  }
}
