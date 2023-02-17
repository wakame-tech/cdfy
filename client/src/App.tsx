import { CarrerPoker } from './plugins/CareerPoker'
import { Counter } from './plugins/Counter'
import { usePlugin } from './state.ts'

function App() {
  const roomId = new URLSearchParams(location.search).get('room') ?? 'global'
  const { plugin } = usePlugin(roomId)
  return (
    <div className='App'>
      {plugin === 'counter' && <Counter roomId={roomId} />}
      {plugin === 'career-poker' && <CarrerPoker roomId={roomId} />}
    </div>
  )
}

export default App
