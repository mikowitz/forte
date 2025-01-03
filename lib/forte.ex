defmodule Forte do
  alias Forte.Operations
  @type pitch_class_set() :: list(non_neg_integer())

  import Forte.Utilities

  defdelegate sets(), to: Forte.Sets
  defdelegate transpose(set, t), to: Operations
  defdelegate transpose_to(set, t), to: Operations
  defdelegate invert(set), to: Operations

  @spec prime_form(pitch_class_set(), atom()) :: pitch_class_set()
  def prime_form(set, _algorithm \\ :carter)

  def prime_form([], _), do: []

  def prime_form(set, :carter) do
    inversion =
      invert(set)

    [set, inversion]
    |> Enum.map(&normal_form_t0/1)
    |> Enum.min()
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

  @spec interval_class_vector(pitch_class_set()) :: list(non_neg_integer())
  def interval_class_vector(set) do
    starting = %{1 => 0, 2 => 0, 3 => 0, 4 => 0, 5 => 0, 6 => 0}

    set
    |> prime_form()
    |> subsets()
    |> Enum.filter(&(length(&1) == 2))
    |> Enum.reduce(starting, fn [pc1, pc2], acc ->
      Map.update(acc, interval_class(pc2 - pc1), 1, &(&1 + 1))
    end)
    |> Map.values()
  end

  @spec subsets(pitch_class_set()) :: list(list(non_neg_integer()))
  def subsets(set) do
    set = Enum.sort(set)

    Enum.reduce(0..length(set), [], fn i, acc ->
      subsets(i, set) ++ acc
    end)
    |> Enum.sort_by(&{length(&1), &1})
  end

  defp subsets(0, _), do: [[]]
  defp subsets(_, []), do: []

  defp subsets(n, [h | t]) do
    subsets(n, t) ++
      for l <- subsets(n - 1, t) do
        [h | l]
      end
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

  defp normal_form_t0(set) do
    set
    |> normal_form()
    |> transpose_to(0)
  end

  defp interval_class(interval) when interval > 6, do: 12 - interval
  defp interval_class(interval), do: interval
end
