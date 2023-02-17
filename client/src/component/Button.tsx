export interface ButtonProps<S> {
  state: S
  label: string
  disabled: (state: S) => boolean
  onClick: (state: S) => void
}

export function Button<S>(props: ButtonProps<S>) {
  return (
    <>
      <button
        disabled={props.disabled(props.state)}
        style={{ margin: '0.5em' }}
        onClick={(e) => props.onClick(props.state)}
      >
        {props.label}
      </button>
    </>
  )
}
