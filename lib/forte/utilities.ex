defmodule Forte.Utilities do
  @moduledoc """
  Utility functions for `Forte`
  """

  @doc """
  Modulo operation ensuring a non-negative return value

  ## Examples

      iex> mod(-3, 12)
      9

  """
  def mod(a, b) do
    rem(rem(a, b) + b, b)
  end
end
