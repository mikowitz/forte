# Forte

`Forte` provides functions for understanding and comparing pitch class sets
based on the work of Allen Forte, particularly his book _The Structure of
Atonal Music_.

This library includes a catalog of the prime forms of pitch class sets of size 0-11,
as defined by Forte. These can be looked up by name

    iex> Forte.sets["6-31"]
    [0,1,3,5,8,9]

or by the set's contents
  
    iex> Forte.name([0,1,3,5,8,9])
    {"6-31", [0,1,3,5,8,9]}

When looking up by contents, the provided set does _not_ have to be in prime
form, as the prime form will be calculated and returned along with the pitch
class set's name

    iex> Forte.name([7,8,4,5,11])
    {"5-16", [0,1,3,4,7]}

## Set operations

`Forte` also provides functions for the basic set operations of transposition
and inversion

    iex> Forte.transpose([1,2,3,4,5], 2)
    [3,4,5,6,7]

    iex> Forte.transpose_to([1,2,3,4,5], 0)
    [0,1,2,3,4]

    iex> Forte.invert([1,2,3,4,5])
    [11,10,9,8,7]

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `forte` to your list of dependencies in `mix.exs`:

    def deps do
      [
        {:forte, "~> 0.1.0"}
      ]
    end

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/forte>.
