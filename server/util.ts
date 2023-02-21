import { ResultState, State } from './gen/types.ts'

export const expect = <T>(
  f: () => T | undefined,
  message: string
): Promise<T> => {
  return new Promise((resolve, reject) => {
    const t = f()
    if (!t) {
      reject(message)
    } else {
      resolve(t)
    }
  })
}

export const unwrap = (f: () => ResultState): Promise<State> => {
  return new Promise((resolve, reject) => {
    const t = f()
    if (t['Err']) {
      reject(t['Err'])
    } else {
      resolve(t['Ok'])
    }
  })
}
