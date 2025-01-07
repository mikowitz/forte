defmodule Forte.SetsTest do
  use ExUnit.Case, async: true

  alias Forte.Sets

  @tag :regression
  test "prime form confirmation" do
    for {_name, set} <- Sets.sets() do
      assert set == Forte.prime_form(set)
    end
  end

  describe "name/1" do
    test "returns the name and prime form of the given set" do
      assert Forte.name([2, 1]) == {"2-1", [0, 1]}
    end

    test "always returns the Forte prime form of the set" do
      assert Forte.name([0, 1, 5, 6, 8]) == {"5-20", [0, 1, 3, 7, 8]}
    end
  end
end
