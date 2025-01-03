defmodule Forte.Sets do
  def sets do
    File.read!("priv/data/forte.sets")
    |> String.split("\n", trim: true)
    |> Enum.map(fn set_str ->
      [name, prime] = String.split(set_str, ":")
      {name, String.split(prime, "", trim: true) |> Enum.map(&to_integer/1)}
    end)
    |> Enum.into(%{})
  end

  defp to_integer("T"), do: 10
  defp to_integer("E"), do: 11
  defp to_integer(x), do: String.to_integer(x)
end
