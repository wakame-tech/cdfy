export interface ButtonProps<S> {
  color?: string
  state: S
  label: string
  disabled: (state: S) => boolean
  onClick: (state: S) => void
}

export function Button<S>(props: ButtonProps<S>) {
  const disabled = props.disabled(props.state)
  return (
    <button
      type='button'
      className={`text-white font-medium rounded text-sm px-3 py-1.5 mr-2 mb-1 focus:outline-none ${
        disabled
          ? 'cursor-not-allowed bg-gray-200'
          : props.color || 'bg-blue-700'
      }`}
      disabled={disabled}
      onClick={(e) => props.onClick(props.state)}
    >
      {props.label}
    </button>
  )
}
