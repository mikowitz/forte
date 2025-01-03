defmodule Forte do
  @type pitch_class_set() :: list(non_neg_integer())

  defdelegate sets(), to: Forte.Sets

  @spec transpose(pitch_class_set(), integer()) :: pitch_class_set()
  def transpose(set, delta) do
    Enum.map(set, &mod(&1 + delta, 12))
  end

  @spec transpose_to(pitch_class_set(), integer()) :: pitch_class_set()
  def transpose_to([s | _] = set, starting) do
    transpose(set, starting - s)
  end

  @spec invert(pitch_class_set()) :: pitch_class_set()
  def invert(set) do
    Enum.map(set, &mod(12 - &1, 12))
  end

  @spec interval_class_vector(pitch_class_set()) :: list(non_neg_integer())
  def interval_class_vector(set) do
    prime = prime_form(set)

    starting = %{1 => 0, 2 => 0, 3 => 0, 4 => 0, 5 => 0, 6 => 0}

    ic_counts =
      Enum.reduce(0..(length(set) - 2), starting, fn i1, acc ->
        pc1 = Enum.at(prime, i1)

        Enum.reduce((i1 + 1)..(length(set) - 1), acc, fn i2, acc ->
          pc2 = Enum.at(prime, i2)
          key = pc2 - pc1
          key = if key > 6, do: 12 - key, else: key
          Map.update(acc, key, 1, &(&1 + 1))
        end)
      end)

    Map.values(ic_counts)
  end

  @spec prime_form(pitch_class_set(), atom()) :: pitch_class_set()
  def prime_form(set, _algorithm \\ :carter)

  def prime_form(set, :carter) do
    set =
      set
      |> normal_form()
      |> transpose_to(0)

    inversion =
      invert(set)
      |> Enum.sort()
      |> normal_form()
      |> transpose_to(0)

    min(set, inversion)
  end

  def prime_form(set, :rahn), do: Forte.Rahn.prime_form(set)

  @spec normal_form(pitch_class_set()) :: pitch_class_set()
  def normal_form(set) do
    Enum.sort(set)
    |> rotations()
    |> with_smallest_span()
    |> most_left_packed()
    |> Enum.map(&mod(&1, 12))
  end

  defp mod(a, b) do
    rem(rem(a, b) + b, b)
  end

  defp rotations(set) do
    (set ++ Enum.map(set, &(&1 + 12)))
    |> Enum.chunk_every(length(set), 1, :discard)
    |> Enum.take(length(set))
  end

  defp with_smallest_span(rotations) do
    Enum.group_by(rotations, &span/1)
    |> then(& &1[Enum.min(Map.keys(&1))])
  end

  defp most_left_packed(sets) do
    Enum.sort(sets)
    |> Enum.min_by(fn [h | rest] ->
      Enum.map(rest, &(&1 - h))
    end)
  end

  defp span([s | _] = set), do: List.last(set) - s
end
