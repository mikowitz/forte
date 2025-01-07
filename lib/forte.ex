defmodule Forte do
  @moduledoc """
  `Forte` provides functions for understanding and comparing pitch class sets 
  based on the work of Allen Forte, particularly his book _The Structure of Atonal Music_.

  This library includes a catalog of the prime forms of pitch class sets of size 0-11, 
  as defined by Forte. These can be looked up by name 

      iex> Forte.sets["6-31"]
      [0,1,3,5,8,9]

  or by the set's contents 
    
      iex> Forte.name([0,1,3,5,8,9])
      {"6-31", [0,1,3,5,8,9]}

  When looking up by contents, the provided set does *not* have to be in prime form, as the 
  prime form will be calculated and returned along with the pitch class set's name

      iex> Forte.name([7,8,4,5,11])
      {"5-16", [0,1,3,4,7]}


  ## Set operations 

  `Forte` also provides functions for the basic set operations of [transposition](`Forte.transpose/2`) 
  and [inversion](`Forte.invert/1`).

      iex> Forte.transpose([1,2,3,4,5], 2)
      [3,4,5,6,7]

      iex> Forte.transpose_to([1,2,3,4,5], 0)
      [0,1,2,3,4]

      iex> Forte.invert([1,2,3,4,5])
      [11,10,9,8,7]

  """
  alias Forte.Operations
  @type pitch_class_index :: 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11
  @type pitch_class_set :: list(pitch_class_index())
  @type prime_form_algorithm :: :forte | :rahn

  import Forte.Utilities

  @doc """
  Returns all Forte sets in a map in the form `%{"Forte name" => prime_form}`

      iex> Forte.sets["3-4"]
      [0,1,5]
    
  """
  @spec sets() :: %{String.t() => pitch_class_set()}
  defdelegate sets(), to: Forte.Sets

  @doc """
  Finds the name and prime form of the Forte set whose prime form matches the given set

      iex> Forte.name([0,2,4,6,8,11])
      {"6-34", [0,1,3,5,7,9]}

  """
  @spec name(list(integer())) :: {String.t(), pitch_class_set()} | nil
  defdelegate name(set), to: Forte.Sets

  @doc """
  Transpose a pitch class set by the given number of chromatic steps

  ## Examples 

      iex> Forte.transpose([0,1,2], 3)
      [3,4,5]

      iex> Forte.transpose([0,1,2], -5)
      [7,8,9]

  """
  @spec transpose(pitch_class_set(), integer()) :: pitch_class_set()
  defdelegate transpose(set, t), to: Operations

  @doc """
  Transpose the given pitch class set to begin on the given pitch class index, modulo 12

  ## Examples

      iex> Forte.transpose_to([4,8,2,3,11], 0)
      [0,4,10,11,7]

  """
  @spec transpose_to(pitch_class_set(), integer()) :: pitch_class_set()
  defdelegate transpose_to(set, t), to: Operations

  @doc """
  Invert the given pitch class set, modulo 12

  ## Examples 

      iex> Forte.invert([1,9,5,7,6,0,3])
      [11,3,7,5,6,0,9]

  """
  @spec invert(pitch_class_set()) :: pitch_class_set()
  defdelegate invert(set), to: Operations

  @doc """
  Returns the canonical prime form of the given set. By default, this function 
  uses Forte's algorithm to calculate the prime form. An optional secondary 
  argument `:rahn` can be included to to use Rahn's algorithm instead. `:forte` 
  can be provided as an explicit second argument, which yields the default
  behaviour.

  Both algorithms begin by finding an ordering of the set with the smallest 
  total span, but in cases of ties, Forte's algorithm prefers orderings with 
  the smallest initial distances, while Rahn's minimizes the outer distances. 
  This is a fairly minor issue since these algorithms provide different 
  prime forms for only 6 (out of 222) pitch class sets.

  ## Examples 

      iex> set = [0,1,3,5,8,9] # Forte 6-31
      iex> Forte.prime_form(set)
      [0,1,3,5,8,9]
      iex> Forte.prime_form(set, :rahn)
      [0,1,4,5,7,9]

  """
  @spec prime_form(list(integer()), prime_form_algorithm()) :: pitch_class_set()
  def prime_form(set, algorithm \\ :forte)

  def prime_form([], _algorithm), do: []

  def prime_form(set, :forte) do
    inversion =
      invert(set)

    [set, inversion]
    |> Enum.map(&normal_form_t0/1)
    |> Enum.min()
  end

  def prime_form(set, :rahn), do: Forte.Rahn.prime_form(set)

  @doc false
  @spec normal_form(list(integer())) :: pitch_class_set()
  def normal_form(set) do
    Enum.sort(set)
    |> rotations()
    |> with_smallest_span()
    |> most_left_packed()
    |> Enum.map(&mod(&1, 12))
  end

  @doc """
  Returns a vector describing the interval content of the set's prime form.

  ## Examples 

      iex> Forte.interval_class_vector([1,2])
      [1,0,0,0,0,0]

      iex> Forte.interval_class_vector([0,2,4,5,7,9,11])
      [2,5,4,3,6,1]

  """
  @spec interval_class_vector(list(integer())) :: list(non_neg_integer())
  def interval_class_vector(set) do
    starting = %{1 => 0, 2 => 0, 3 => 0, 4 => 0, 5 => 0, 6 => 0}

    set
    |> prime_form()
    |> then(&subsets(2, &1))
    |> Enum.reduce(starting, fn [pc1, pc2], acc ->
      Map.update(acc, interval_class(pc2 - pc1), 1, &(&1 + 1))
    end)
    |> Map.values()
  end

  @doc """
  Returns all unique subsets of the given set

  ## Examples 

      iex> Forte.subsets([0,1,2])
      [[], [0], [1], [2], [0, 1], [0, 2], [1, 2], [0, 1, 2]]
  """
  @spec subsets(list(integer())) :: list(list(integer()))
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
