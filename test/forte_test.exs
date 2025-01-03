defmodule ForteTest do
  use ExUnit.Case
  doctest Forte

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

  describe "normal_form/1" do
    test "reorders the set to its most left-compacted order" do
      assert Forte.normal_form([1, 2, 11]) == [11, 1, 2]
      assert Forte.normal_form([1, 3, 7, 10]) == [7, 10, 1, 3]
      assert Forte.normal_form([4, 2, 0, 8]) == [0, 2, 4, 8]
    end
  end

  describe "prime_form/1" do
    test "finds the canonical prime form of the set" do
      assert Forte.prime_form([1, 3, 7, 10]) == [0, 2, 5, 8]

      assert Forte.prime_form([0, 1, 3, 7, 8]) == [0, 1, 3, 7, 8]
      assert Forte.prime_form([0, 1, 5, 6, 8]) == [0, 1, 3, 7, 8]

      assert Forte.prime_form([0, 1, 3, 5, 8, 9]) == [0, 1, 3, 5, 8, 9]
      assert Forte.prime_form([0, 1, 4, 5, 7, 9]) == [0, 1, 3, 5, 8, 9]

      assert Forte.prime_form([0, 1, 3, 6, 8, 9]) == [0, 1, 3, 6, 8, 9]
      assert Forte.prime_form([0, 2, 3, 6, 7, 9]) == [0, 1, 3, 6, 8, 9]

      assert Forte.prime_form([0, 1, 2, 4, 7, 8, 9]) == [0, 1, 2, 4, 7, 8, 9]
      assert Forte.prime_form([0, 1, 2, 5, 6, 7, 9]) == [0, 1, 2, 4, 7, 8, 9]

      assert Forte.prime_form([0, 1, 2, 3, 5, 8, 9]) == [0, 1, 2, 3, 5, 8, 9]
      assert Forte.prime_form([0, 1, 4, 5, 6, 7, 9]) == [0, 1, 2, 3, 5, 8, 9]

      assert Forte.prime_form([0, 1, 2, 4, 5, 7, 9, 10]) == [0, 1, 2, 4, 5, 7, 9, 10]
      assert Forte.prime_form([0, 1, 3, 4, 5, 7, 8, 10]) == [0, 1, 2, 4, 5, 7, 9, 10]
    end
  end

  describe "prime_form/2" do
    test "can use Rahn's algorithm to find the prime form of a set" do
      assert Forte.prime_form([0, 1, 3, 7, 8], :rahn) == [0, 1, 5, 6, 8]
      assert Forte.prime_form([0, 1, 5, 6, 8], :rahn) == [0, 1, 5, 6, 8]

      assert Forte.prime_form([0, 1, 3, 5, 8, 9], :rahn) == [0, 1, 4, 5, 7, 9]
      assert Forte.prime_form([0, 1, 4, 5, 7, 9], :rahn) == [0, 1, 4, 5, 7, 9]

      assert Forte.prime_form([0, 1, 3, 6, 8, 9], :rahn) == [0, 2, 3, 6, 7, 9]
      assert Forte.prime_form([0, 2, 3, 6, 7, 9], :rahn) == [0, 2, 3, 6, 7, 9]

      assert Forte.prime_form([0, 1, 2, 4, 7, 8, 9], :rahn) == [0, 1, 2, 5, 6, 7, 9]
      assert Forte.prime_form([0, 1, 2, 5, 6, 7, 9], :rahn) == [0, 1, 2, 5, 6, 7, 9]

      assert Forte.prime_form([0, 1, 2, 3, 5, 8, 9], :rahn) == [0, 1, 4, 5, 6, 7, 9]
      assert Forte.prime_form([0, 1, 4, 5, 6, 7, 9], :rahn) == [0, 1, 4, 5, 6, 7, 9]

      assert Forte.prime_form([0, 1, 2, 4, 5, 7, 9, 10], :rahn) == [0, 1, 3, 4, 5, 7, 8, 10]
      assert Forte.prime_form([0, 1, 3, 4, 5, 7, 8, 10], :rahn) == [0, 1, 3, 4, 5, 7, 8, 10]
    end
  end

  describe "interval_class_vector/1" do
    test "returns the interval class vector for the set" do
      assert Forte.interval_class_vector([0, 1, 3, 7, 8]) == [2, 1, 1, 2, 3, 1]
      assert Forte.interval_class_vector([0, 1, 5, 6, 8]) == [2, 1, 1, 2, 3, 1]
    end
  end
end
