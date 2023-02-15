import { useState } from 'react'
import { useGameState } from '../state'
import { Card } from './Card'
import { Deck, useSelects } from './Deck'

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
  effect: {}
  prompt_4_player_id: string | null
  prompt_7_player_id: string | null
  prompt_13_player_id: string | null
  prompt_1_player_ids: string[]
}

export const CarrerPoker = (props: { roomId: string }) => {
  const [isDebug, setDebug] = useState(true)
  const { id, state, action } = useGameState<Data>(props.roomId)
  const hands = state?.fields[id] ?? []
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

  const selectTrushes = () => {
    const cards = selectedTrushes.map((i) => state.trushes[i])
    action('select_trushes', cards)
    resetTrushes()
  }

  const selectExcludes = () => {
    const cards = selectedExcludes.map((i) => state.excluded[i])
    action('select_excluded', cards)
    resetExcludes()
  }

  const selectPasses = () => {
    const cards = selectedHands.map((i) => hands[i])
    action('select_passes', cards)
    resetHands()
  }

  const serve = () => {
    const cards = selectedHands.map((i) => hands[i])
    action('serve', cards)
    resetHands()
  }

  const oneChance = () => {
    const card = hands.find((card) => card['Number'] && card['Number'][1] == 1)!
    action('one_chance', [card])
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
          <p>last={JSON.stringify(state.last_served_player_id)}</p>
        </>
      )}

      <button
        style={{ margin: '0.5em' }}
        onClick={(e) => action('distribute', {})}
      >
        配る
      </button>

      <Deck
        readonly={state.prompt_13_player_id !== id}
        name='除外'
        cards={state.excluded}
        selects={selectedExcludes}
        onClickCard={toggleExclude}
      />
      <Deck
        readonly={state.prompt_4_player_id !== id}
        name='墓地'
        cards={state.trushes}
        selects={selectedTrushes}
        onClickCard={toggleTrush}
      />
      <Deck readonly name='場' cards={state.river.at(-1) ?? []} />
      <Deck
        readonly={id !== state.current}
        name='手札'
        cards={hands}
        selects={selectedHands}
        onClickCard={toggleHand}
      />

      {id === state.current &&
        state.prompt_13_player_id !== id &&
        state.prompt_7_player_id !== id &&
        state.prompt_4_player_id !== id && (
          <>
            <button
              style={{ margin: '0.5em' }}
              onClick={(e) => action('pass', {})}
            >
              パス
            </button>
            <button style={{ margin: '0.5em' }} onClick={serve}>
              出す
            </button>
          </>
        )}
      {state.prompt_13_player_id === id && (
        <button style={{ margin: '0.5em' }} onClick={selectExcludes}>
          除外から手札に加える
        </button>
      )}
      {state.prompt_4_player_id === id && (
        <button style={{ margin: '0.5em' }} onClick={selectTrushes}>
          墓地から手札に加える
        </button>
      )}
      {state.prompt_7_player_id === id && (
        <button style={{ margin: '0.5em' }} onClick={selectPasses}>
          左隣の人に渡す
        </button>
      )}
      {id !== state.current &&
        hands.some((card) => card['Number'] && card['Number'][1] == 1) && (
          <button style={{ margin: '0.5em' }} onClick={oneChance}>
            ワンチャンス
          </button>
        )}
    </div>
  )
}
