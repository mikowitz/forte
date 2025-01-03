defmodule Forte.Sets do
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

  def sets, do: @sets
end
