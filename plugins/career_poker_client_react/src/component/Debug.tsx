import ReactJson from 'react-json-view'

export const DebugView = (props: { state: object }) => {
  return <ReactJson src={props.state} />
}
