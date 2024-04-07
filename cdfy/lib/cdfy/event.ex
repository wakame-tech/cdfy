defmodule Cdfy.Event do
  @type t ::
          %{
            name: String.t(),
            value: map()
          }
          | %{name: String.t()}
end
