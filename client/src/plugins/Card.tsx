const card = (expr: string): string => {
  const playingCards = [
    ...'🂠🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂭🂮🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂽🂾🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃍🃎🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃝🃞🃟',
  ]
  if (!expr) {
    return playingCards[0]
  }
  if (expr === 'joker') {
    return playingCards[playingCards.length - 1]
  }
  const suit = 'shdc'.indexOf(expr[1])
  const number = 'A23456789TJQK'.indexOf(expr[0])
  return playingCards[1 + suit * 13 + number]
}

export interface CardProps {
  expr: string
  onClick: () => void
}

export const Card = (props: CardProps) => {
  return (
    <span
      onClick={(e) => props.onClick()}
      style={{
        color: props.expr[1].match(/[hd]/) ? 'red' : 'black',
        fontSize: '100px',
        margin: '0px',
        padding: '0px',
        userSelect: 'none',
      }}
    >
      {card(props.expr)}
    </span>
  )
}
