defmodule Forte.OperationsTest do
  use ExUnit.Case, async: true

  describe "transpose/2" do
    test "transposes up the given step, modulo 12" do
      assert Forte.transpose([0, 1, 4, 6, 5, 10], 3) == [3, 4, 7, 9, 8, 1]
    end

    test "transposes down the given step, modulo 12" do
      assert Forte.transpose([0, 1, 4, 6, 5, 9], -3) == [9, 10, 1, 3, 2, 6]
    end
  end

  describe "transpose_to/2" do
    test "transposes to start on the given index, modulo 12" do
      assert Forte.transpose_to([1, 3, 7, 6, 4, 0], 0) == [0, 2, 6, 5, 3, 11]
      assert Forte.transpose_to([1, 3, 7, 6, 4, 0], 7) == [7, 9, 1, 0, 10, 6]
    end
  end

  describe "invert/1" do
    test "inverts the set, modulo 12" do
      assert Forte.invert([1, 5, 0, 2, 4, 8]) == [11, 7, 0, 10, 8, 4]
    end
  end
end
