import { usePlugin } from '../state'
import { Button } from '../component/Button'

interface State {
  count: number
}

type Action = 'Increment'

export const Counter = (props: { roomId: string }) => {
  const { state, rpc } = usePlugin<State, Action>(props.roomId)

  return (
    <div className='App'>
      <p>{JSON.stringify(state)}</p>
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
