import { useGameState } from '../state'
import { Card, CardView } from './Card'

export interface Data {
  actions: string[]
  fields: Record<string, Card[]>
}

export const CarrerPoker = (props: { roomId: string }) => {
  const { id, state, action } = useGameState<Data>(props.roomId)

  const onClick = (id: string) => {
    action(id)
  }

  return (
    <div className='App'>
      <div>
        {state?.actions.map((id) => (
          <button onClick={(e) => onClick(id)}>{id}</button>
        ))}
      </div>
      <div>
        {state?.fields[id].map((card) => (
          <CardView card={card} />
        ))}
      </div>
    </div>
  )
}
