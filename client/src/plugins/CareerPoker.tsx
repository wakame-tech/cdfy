import { useEffect, useState } from 'react'
import { usePlugin } from '../state'
import { Suit, Card } from '../component/CardView'
import { Button } from '../component/Button'
import { DebugView } from '../component/Debug'
import { DeckView, Deck, defaultDeck, useSelects } from '../component/DeckView'
import { Players } from '../component/Players'

type Rpc =
  | 'Distribute'
  | {
      Pass: { player_id: string }
    }
  | {
      OneChance: { player_id: string; serves: Card[] }
    }
  | {
      Select: {
        from: string
        player_id: string
        serves: Card[]
      }
    }
  | {
      ServeAnother: {
        player_id: string
        serves: Card[]
      }
    }
  | {
      Serve: {
        player_id: string
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

export interface State {
  room_id: string
  current: string | null
  players: string[]
  river: Deck[]
  will_flush_task_id: string | null
  last_served_player_id: string | null
  // players deck + trushes + excluded
  fields: Record<string, Deck>
  river_size: number | null
  effect: Effect
  prompts: Record<string, string>
}

export const CarrerPoker = (props: { roomId: string }) => {
  const [isDebug, setDebug] = useState(false)
  const { id, state, rpc } = usePlugin<State, Rpc>(props.roomId)

  const hands = state?.fields[id] ?? defaultDeck()
  const river = state?.river[state?.river.length - 1] ?? defaultDeck()

  const [sec, setSec] = useState(0)

  useEffect(() => {
    if (state?.will_flush_task_id) {
      let s = 5
      setSec(s)
      let id = setInterval(() => setSec(--s), 1000)
      return () => {
        clearInterval(id)
      }
    }
  }, [state?.will_flush_task_id])

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
      <Players id={id} state={state} />

      <div className='flex'>
        <div className='flex items-center mb-4'>
          <input
            id='default-checkbox'
            type='checkbox'
            checked={isDebug}
            className='w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded'
            onClick={(e) => setDebug(!isDebug)}
          />
          <label
            htmlFor='default-checkbox'
            className='ml-2 text-sm font-medium text-gray-900 dark:text-gray-300'
          >
            デバッグ
          </label>
        </div>
      </div>

      {isDebug && <DebugView state={state} />}

      <details>
        <summary>除外・墓地</summary>
        <DeckView
          state={state}
          label='除外'
          disabled={(state) => state.prompts[id] !== 'excluded'}
          deck={state.fields['excluded']}
          selects={selectedExcludes}
          onClickCard={toggleExclude}
        />
        <DeckView
          state={state}
          label='墓地'
          disabled={(state) => state.prompts[id] !== 'trushes'}
          deck={state.fields['trushes']}
          selects={selectedTrushes}
          onClickCard={toggleTrush}
        />
      </details>
      <DeckView
        state={state}
        label='場'
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

      {state.effect.effect_limits.length > 0 && (
        <p>
          効果制限: [
          {[...state.effect.effect_limits.sort((a, b) => a - b)].join(',')}]
        </p>
      )}
      {state.effect.suit_limits.length > 0 && (
        <p>縛り: [{[...state.effect.suit_limits].join(',')}]</p>
      )}
      {state.effect.is_step && <p>階段状態</p>}
      {state.effect.revoluted !== state.effect.turn_revoluted && (
        <p>革命状態</p>
      )}

      <div>
        <Button
          color='bg-red-700'
          state={state}
          label='開始'
          disabled={(state) => id !== state.players[0]}
          onClick={() => rpc('Distribute')}
        />
        <Button
          state={state}
          label='パス'
          disabled={(state) =>
            state.river.length === 0 ||
            id !== state.current ||
            !!state.prompts[id] ||
            !!state.will_flush_task_id
          }
          onClick={() =>
            rpc({
              Pass: {
                player_id: id,
              },
            })
          }
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
                player_id: id,
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
            const serves = selectedExcludes.map(
              (i) => state.fields['excluded'].cards[i]
            )
            rpc({
              Select: {
                from: 'excluded',
                player_id: id,
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
            const serves = selectedTrushes.map(
              (i) => state.fields['trushes'].cards[i]
            )
            rpc({
              Select: {
                from: 'trushes',
                player_id: id,
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
              ServeAnother: {
                player_id: id,
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
            const disabled_1 = state.effect.effect_limits.includes(1)
            return (
              disabled_1 ||
              !hands.cards.some(
                (card) => 'Number' in card && card['Number'][1] == 1
              )
            )
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
                player_id: id,
                serves: [card],
              },
            })
          }}
        />
      </div>

      {state.will_flush_task_id && <p>{sec} 秒後に流れます</p>}
    </div>
  )
}
