import { Card, CardView } from './CardView'
import { useState } from 'react'

export interface Deck {
  style: 'Arrange' | 'Stack'
  cards: Card[]
}

export const defaultDeck = (): Deck => {
  return {
    style: 'Arrange',
    cards: [],
  }
}

export interface DeckViewProps<S> {
  state: S
  label: string
  disabled?: (state: S) => boolean
  deck: Deck
  selects?: number[]
  onClickCard?: (key: number) => void
}

export const useSelects = () => {
  const [selects, setSelects] = useState<number[]>([])

  const toggle = (i: number) => {
    if (selects.includes(i)) {
      setSelects(selects.filter((n) => n != i))
    } else {
      setSelects([...selects, i])
    }
  }

  const reset = () => {
    setSelects([])
  }

  return {
    selects,
    toggle,
    reset,
  }
}

export function DeckView<S>(props: DeckViewProps<S>) {
  return (
    <div style={{ minHeight: '8rem' }}>
      <h2
        style={{
          color: props.disabled?.(props.state) ? 'lightgray' : 'black',
        }}
      >
        {props.label}({props.deck.cards.length})
      </h2>
      <div
        style={{ display: 'flex', overflowX: 'scroll', paddingBottom: '2rem' }}
      >
        {props.deck.cards.map((card, i) => (
          <>
            <CardView
              key={i}
              onClick={() =>
                !props.disabled?.(props.state) && props.onClickCard?.(i)
              }
              selected={props.selects?.includes(i) ?? false}
              card={card}
            />
          </>
        ))}
      </div>
    </div>
  )
}
