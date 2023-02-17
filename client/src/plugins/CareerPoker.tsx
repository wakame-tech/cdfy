import { useState } from 'react'
import { usePlugin } from '../state'
import { Suit, Card } from '../component/CardView'
import { Button } from '../component/Button'
import { DeckView, Deck, defaultDeck, useSelects } from '../component/DeckView'

type Rpc =
  | 'Distribute'
  | 'Pass'
  | {
      OneChance: { serves: Card[] }
    }
  | {
      SelectTrushes: {
        serves: Card[]
      }
    }
  | {
      SelectPasses: {
        serves: Card[]
      }
    }
  | {
      SelectExcluded: {
        serves: Card[]
      }
    }
  | {
      Serve: {
        serves: Card[]
      }
    }

interface Effect {
  river_size: number | null
  suit_limits: Suit[]
  effect_limits: number[]
  turn_revoluted: boolean
  is_step: boolean
  revoluted: boolean
}

interface State {
  current: string | null
  players: string[]
  excluded: Deck
  trushes: Deck
  river: Deck[]
  will_flush_task_id: string | null
  last_served_player_id: string | null
  fields: Record<string, Deck>
  river_size: number | null
  effect: Effect
  prompts: Record<string, string>
}

export const CarrerPoker = (props: { roomId: string }) => {
  const [isDebug, setDebug] = useState(true)
  const { id, state, rpc } = usePlugin<State, Rpc>(props.roomId)

  const hands = state?.fields[id] ?? defaultDeck()
  const river = state?.river[state?.river.length - 1] ?? defaultDeck()

  const {
    selects: selectedHands,
    toggle: toggleHand,
    reset: resetHands,
  } = useSelects()
  const {
    selects: selectedExcludes,
    toggle: toggleExclude,
    reset: resetExcludes,
  } = useSelects()
  const {
    selects: selectedTrushes,
    toggle: toggleTrush,
    reset: resetTrushes,
  } = useSelects()

  if (!state) {
    return <div></div>
  }

  return (
    <div className='App'>
      {isDebug && (
        <>
          <p>
            id={id}, players={state?.players.join(',')}
          </p>
          <p>{JSON.stringify(state.effect)}</p>
          <p>{state.players.join(' -> ')}</p>
          <p>current={JSON.stringify(state.current)}</p>
          <p>selecting={state.prompts[id]}</p>
          <p>task={state.will_flush_task_id}</p>
        </>
      )}

      <Button
        state={state}
        label='配る'
        disabled={(state) => false}
        onClick={() => rpc('Distribute')}
      />

      <details>
        <summary>除外・墓地</summary>
        <DeckView
          state={state}
          label='除外'
          disabled={(state) => state.prompts[id] !== 'excluded'}
          deck={state.excluded}
          selects={selectedExcludes}
          onClickCard={toggleExclude}
        />
        <DeckView
          state={state}
          label='墓地'
          disabled={(state) => state.prompts[id] !== 'trushes'}
          deck={state.trushes}
          selects={selectedTrushes}
          onClickCard={toggleTrush}
        />
      </details>
      <DeckView
        state={state}
        label={state.will_flush_task_id ? '場(5秒後に流れます)' : '場'}
        disabled={(state) => true}
        deck={river}
      />
      <DeckView
        state={state}
        label='手札'
        disabled={(state) => id !== state.current}
        deck={hands}
        selects={selectedHands}
        onClickCard={toggleHand}
      />
      <Button
        state={state}
        label='パス'
        disabled={(state) =>
          id !== state.current ||
          !!state.prompts[id] ||
          !!state.will_flush_task_id
        }
        onClick={() => rpc('Pass')}
      />

      <Button
        state={state}
        label='出す'
        disabled={(state) =>
          id !== state.current ||
          !!state.prompts[id] ||
          !!state.will_flush_task_id
        }
        onClick={() => {
          const serves = selectedHands.map((i) => hands.cards[i])
          rpc({
            Serve: {
              serves,
            },
          })
          resetHands()
        }}
      />

      <Button
        state={state}
        label='除外から手札に加える'
        disabled={(state) => state.prompts[id] !== 'excluded'}
        onClick={() => {
          const serves = selectedExcludes.map((i) => state.excluded.cards[i])
          rpc({
            SelectExcluded: {
              serves,
            },
          })
          resetExcludes()
        }}
      />

      <Button
        state={state}
        label='墓地から手札に加える'
        disabled={(state) => state.prompts[id] !== 'trushes'}
        onClick={() => {
          const serves = selectedTrushes.map((i) => state.trushes.cards[i])
          rpc({
            SelectTrushes: {
              serves,
            },
          })
          resetTrushes()
        }}
      />

      <Button
        state={state}
        label='左隣の人に渡す'
        disabled={(state) => state.prompts[id] !== id}
        onClick={() => {
          const serves = selectedHands.map((i) => hands.cards[i])
          rpc({
            SelectPasses: {
              serves,
            },
          })
          resetHands()
        }}
      />

      <Button
        state={state}
        label='ワンチャンス'
        disabled={(state) => {
          const hands = state?.fields[id] ?? defaultDeck()
          hands.cards.some((card) => 'Number' in card && card['Number'][1] == 1)
        }}
        onClick={() => {
          const card = hands.cards.find(
            (card) => 'Number' in card && card['Number'][1] == 1
          )!
          if (!card) {
            return
          }
          rpc({
            OneChance: {
              serves: [card],
            },
          })
        }}
      />
    </div>
  )
}
