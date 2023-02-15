export type Card =
  | {
      Number: ['s' | 'h' | 'd' | 'c', number]
    }
  | 'Joker'

const card = (card: Card): string => {
  const playingCards = [
    ...'🂠🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂭🂮🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂽🂾🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃍🃎🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃝🃞🃟',
  ]

  if (card === 'Joker') {
    return playingCards[playingCards.length - 1]
  }
  const suit = 'shdc'.indexOf(card['Number'][0])
  return playingCards[1 + suit * 13 + (card['Number'][1] - 1)]
}

const isRed = (card: Card): boolean => {
  return card !== 'Joker' && /[hd]/.test(card['Number'][0])
}

export interface CardProps {
  card: Card
  onClick?: () => void
}

export const CardView = (props: CardProps) => {
  return (
    <span
      onClick={(e) => props.onClick?.()}
      style={{
        color: isRed(props.card) ? 'red' : 'black',
        fontSize: '100px',
        margin: '0px',
        padding: '0px',
        userSelect: 'none',
      }}
    >
      {card(props.card)}
    </span>
  )
}
