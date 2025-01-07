defmodule Forte.Operations do
  @moduledoc false

  import Forte.Utilities

  @spec transpose(Forte.pitch_class_set(), integer()) :: Forte.pitch_class_set()
  def transpose(set, delta) do
    Enum.map(set, &mod(&1 + delta, 12))
  end

  @doc """
  """
  @spec transpose_to(Forte.pitch_class_set(), integer()) :: Forte.pitch_class_set()
  def transpose_to([s | _] = set, starting) do
    transpose(set, starting - s)
  end

  @doc """
  """
  @spec invert(Forte.pitch_class_set()) :: Forte.pitch_class_set()
  def invert(set) do
    Enum.map(set, &mod(12 - &1, 12))
  end
end
