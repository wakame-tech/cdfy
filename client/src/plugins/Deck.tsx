import { Card, CardView } from './Card'

export interface DeckProps {
  name: string
  readonly?: boolean
  cards: Card[]
  selects?: number[]
  onClickCard?: (key: number) => void
}

export const Deck = (props: DeckProps) => {
  return (
    <div>
      <h2>
        {props.name}({props.cards.length})
      </h2>
      {props.cards.map((card, i) => (
        <>
          <CardView
            onClick={() => props.onClickCard?.(i)}
            selected={props.selects?.includes(i) ?? false}
            card={card}
          />
        </>
      ))}
    </div>
  )
}
