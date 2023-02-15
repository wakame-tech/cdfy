import { useState } from 'react'
import { useGameState } from '../state'
import { Card } from './Card'
import { Deck } from './Deck'

export interface Data {
  current: string | null
  actions: string[]
  players: string[]
  excluded: Card[]
  river: Card[][]
  trushes: Card[]
  fields: Record<string, Card[]>
  last_served_player_id: string | null
  river_size: number | null
}

export const CarrerPoker = (props: { roomId: string }) => {
  const { id, state, action } = useGameState<Data>(props.roomId)
  const hands = state?.fields[id] ?? []
  const [selects, setSelects] = useState<number[]>([])

  const onClickCard = (i: number) => {
    if (selects.includes(i)) {
      setSelects(selects.filter((n) => n != i))
    } else {
      setSelects([...selects, i])
    }
  }

  if (!state) {
    return <div></div>
  }

  return (
    <div className='App'>
      <p>
        id={id}, players={state?.players.join(',')}
      </p>
      <p>{JSON.stringify(state)}</p>
      <button
        style={{ margin: '0.5em' }}
        onClick={(e) => action('distribute', {})}
      >
        配る
      </button>

      <button style={{ margin: '0.5em' }} onClick={(e) => action('pass', {})}>
        パス
      </button>
      <button
        style={{ margin: '0.5em' }}
        onClick={(e) => {
          const cards = selects.map((i) => hands[i])
          action('serve', cards)
          setSelects([])
        }}
      >
        出す
      </button>
      <Deck readonly name='山札' cards={state.river.at(-1) ?? []} />
      <Deck
        name='手札'
        cards={hands}
        selects={selects}
        onClickCard={onClickCard}
      />
    </div>
  )
}
