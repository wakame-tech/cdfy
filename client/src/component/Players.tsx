import { State } from '../plugins/CareerPoker'

export const Players = (props: { state: State; id: string }) => {
  return (
    <div>
      {props.state.players.map((player, i, arr) => {
        return (
          <>
            <span
              className={`${
                props.state.current !== player ? 'text-gray-300' : ''
              } ${props.id === player ? 'underline' : ''}`}
            >
              {player.substring(0, 4)} (
              {props.state.fields[player]?.cards.length ?? 0})
            </span>
            {i !== arr.length - 1 && <span className='px-2'>→</span>}
          </>
        )
      })}
    </div>
  )
}
