import { Card, CardView } from './Card'
import { useState } from 'react'
export interface DeckProps {
  name: string
  readonly?: boolean
  cards: Card[]
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

export const Deck = (props: DeckProps) => {
  return (
    <div>
      <h2
        style={{
          color: props.readonly ? 'gray' : 'black',
        }}
      >
        {props.name}({props.cards.length})
      </h2>
      {props.cards.map((card, i) => (
        <>
          <CardView
            onClick={() => !props.readonly && props.onClickCard?.(i)}
            selected={props.selects?.includes(i) ?? false}
            card={card}
          />
        </>
      ))}
    </div>
  )
}
