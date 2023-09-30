// import { CarrerPoker } from './plugins/CareerPoker'
import { Counter } from './plugins/Counter'
import { useRoom } from './useRoom'

function App() {
  const { room, fetch, create, join, load, message } = useRoom("a", "u")

  return (
    <>
      <button className="p-2 border" onClick={e => fetch()}>fetch</button>
      <button className="p-2 border" onClick={e => create()}>create</button>
      <button className="p-2 border" onClick={e => join()}>join</button>
      <button className="p-2 border" onClick={e => load()}>load</button>

      <p>{JSON.stringify(room)}</p>

      <div className='p-2'>
        {"counter" in room.states && <Counter id="u" state={JSON.parse(room.states["counter"])} onMessage={message} />}
      </div>
    </>
  )
}

export default App
