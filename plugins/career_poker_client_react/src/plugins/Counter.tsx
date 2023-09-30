
export interface State {
    count: number
}

export type Message =
    | 'Increment'

export interface CounterProps {
    id: string,
    state: State,
    onMessage: (message: Message) => Promise<void>,
}

export const Counter = ({ state, id, onMessage }: CounterProps) => {
    return (
        <>
            <button className="p-2 border" onClick={e => onMessage('Increment')}>+1</button>
            <div>count: {state.count}</div>
        </>
    )
}