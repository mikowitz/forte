defmodule Forte.SetsTest do
  use ExUnit.Case, async: true

  alias Forte.Sets

  @tag :regression
  test "prime form confirmation" do
    for {_name, set} <- Sets.sets() do
      assert set == Forte.prime_form(set)
    end
  end
end
