import { CarrerPoker } from './plugins/CareerPoker'
import { Counter } from './plugins/Counter'
import { pluginId } from './state'

function App() {
  const roomId = new URLSearchParams(location.search).get('room') ?? 'global'
  return (
    <div className='p-2'>
      {pluginId === 'counter' && <Counter roomId={roomId} />}
      {pluginId === 'career_poker' && <CarrerPoker roomId={roomId} />}
    </div>
  )
}

export default App
