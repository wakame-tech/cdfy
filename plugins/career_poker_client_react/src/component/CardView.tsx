export type Suit = 's' | 'h' | 'd' | 'c'

export type Card =
  | {
      Number: [Suit, number]
    }
  | {
      Joker: [Suit, number] | null
    }

const card = (card: Card): string => {
  const playingCards = [
    ...'🂠🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂭🂮🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂽🂾🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃍🃎🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃝🃞🃟',
  ]

  if ('Joker' in card) {
    return playingCards[playingCards.length - 1]
  }
  const suit = 'shdc'.indexOf(card['Number'][0])
  return playingCards[1 + suit * 13 + (card['Number'][1] - 1)]
}

const isRed = (card: Card): boolean => {
  if ('Number' in card) {
    return /[hd]/.test(card['Number'][0])
  } else {
    return false
  }
}

export interface CardProps {
  card: Card
  selected: boolean
  onClick?: () => void
}

export const CardView = (props: CardProps) => {
  return (
    <>
      <span
        onClick={(e) => props.onClick?.()}
        style={{
          display: 'inline-block',
          lineHeight: '1em',
          color: isRed(props.card) ? 'red' : 'black',
          fontSize: '100px',
          backgroundColor: props.selected ? '#ecc' : '',
          borderRadius: '10px',
          userSelect: 'none',
        }}
      >
        {card(props.card)}
      </span>
    </>
  )
}
