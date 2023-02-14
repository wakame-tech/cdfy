import { Card } from './plugins/Card'
import { Counter } from './plugins/Counter'

function App() {
  const roomId = new URLSearchParams(location.search).get('room') ?? 'global'

  return (
    <div className='App'>
      <p>roomId={roomId}</p>
      <Counter roomId={roomId} />

      <Card onClick={() => alert('a')} expr='Ah' />
      <Card onClick={() => alert('a')} expr='As' />
    </div>
  )
}

export default App
