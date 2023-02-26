import { CarrerPoker } from './plugins/CareerPoker'
import { Counter } from './plugins/Counter'
import { usePlugin } from './state'

function App() {
  const roomId = new URLSearchParams(location.search).get('room') ?? 'global'
  const { plugin } = usePlugin(roomId)
  return (
    <div className='p-2'>
      {plugin === 'counter' && <Counter roomId={roomId} />}
      {plugin === 'career_poker' && <CarrerPoker roomId={roomId} />}
    </div>
  )
}

export default App
