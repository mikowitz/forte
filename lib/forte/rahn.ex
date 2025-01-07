defmodule Forte.Rahn do
  @moduledoc false

  import Bitwise
  import Forte.{Operations, Utilities}

  def prime_form(set) do
    zeroed =
      set
      |> normal_form()
      |> zero()

    i =
      zeroed
      |> invert()
      |> normal_form()
      |> zero()

    [zeroed, i]
    |> Enum.min_by(&bitsize/1)
  end

  def normal_form(set) do
    set
    |> Enum.sort()
    |> rotations()
    |> Enum.min_by(&bitsize/1)
    |> Enum.map(&rem(&1, 12))
  end

  defp rotations(set) do
    (set ++ Enum.map(set, &(&1 + 12)))
    |> Enum.chunk_every(length(set), 1)
    |> Enum.take(length(set))
  end

  defp zero([h | _] = set) do
    Enum.map(set, &mod(&1 - h, 12))
  end

  defp bitsize([h | _] = set) do
    Enum.map(set, &(1 <<< (&1 - h)))
    |> Enum.reduce(0, &|||/2)
  end
end
