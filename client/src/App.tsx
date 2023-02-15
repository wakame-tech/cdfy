import { CarrerPoker } from './plugins/CareerPoker'
import { Counter } from './plugins/Counter'

function App() {
  const roomId = new URLSearchParams(location.search).get('room') ?? 'global'

  return (
    <div className='App'>
      <p>roomId={roomId}</p>
      {/* <Counter roomId={roomId} /> */}
      <CarrerPoker roomId={roomId} />
    </div>
  )
}

export default App
