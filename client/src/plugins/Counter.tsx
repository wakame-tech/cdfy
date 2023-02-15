import { useGameState } from '../state'

export interface Data {
  count: number
  actions: string[]
}

export const Counter = (props: { roomId: string }) => {
  const { state, action } = useGameState<Data>(props.roomId)

  const onClick = (id: string) => {
    action(id, {})
  }

  return (
    <div className='App'>
      <p>{state?.count}</p>
      {state?.actions.map((id) => (
        <button onClick={(e) => onClick(id)}>{id}</button>
      ))}
    </div>
  )
}
