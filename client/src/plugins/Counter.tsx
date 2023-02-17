import { usePlugin } from '../state'
import { Button } from '../component/Button'

interface State {
  count: number
}

type Action = 'Increment' | 'WillIncrement' | 'Cancel'

export const Counter = (props: { roomId: string }) => {
  const { state, rpc } = usePlugin<State, Action>(props.roomId)

  return (
    <div className='App'>
      <p>{JSON.stringify(state)}</p>
      <Button
        state={state}
        label='cancel'
        disabled={(state) => false}
        onClick={() => {
          rpc('Cancel')
        }}
      />
      <Button
        state={state}
        label='+1 after 3s'
        disabled={(state) => false}
        onClick={() => {
          rpc('WillIncrement')
        }}
      />
      <Button
        state={state}
        label='+1'
        disabled={(state) => false}
        onClick={() => {
          rpc('Increment')
        }}
      />
    </div>
  )
}
