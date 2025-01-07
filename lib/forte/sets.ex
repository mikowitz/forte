defmodule Forte.Sets do
  @moduledoc false

  @sets File.read!("priv/data/forte.sets")
        |> String.split("\n", trim: true)
        |> Enum.map(fn set_str ->
          [name, prime] = String.split(set_str, ":")

          {name,
           String.split(prime, "", trim: true)
           |> Enum.map(fn
             "T" -> 10
             "E" -> 11
             x -> String.to_integer(x)
           end)}
        end)
        |> Enum.into(%{})

  @spec sets() :: %{String.t() => Forte.pitch_class_set()}
  def sets, do: @sets

  @spec name(list(integer())) :: {String.t(), Forte.pitch_class_set()} | nil
  def name(set) do
    prime = Forte.prime_form(set)

    Enum.find(@sets, fn {_k, v} -> v == prime end)
  end
end
