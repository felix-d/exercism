defmodule Raindrops do
  @doc """
  Returns a string based on raindrop factors.

  - If the number contains 3 as a prime factor, output 'Pling'.
  - If the number contains 5 as a prime factor, output 'Plang'.
  - If the number contains 7 as a prime factor, output 'Plong'.
  - If the number does not contain 3, 5, or 7 as a prime factor,
    just pass the number's digits straight through.
  """
  @pairs [{3, "Pling"}, {5, "Plang"}, {7, "Plong"}]

  @spec convert(pos_integer) :: String.t
  def convert(number) do
    message = @pairs
    |> Enum.filter_map(fn {x, y} -> rem(number, x) == 0 end, fn {x, y} -> y end)
    |> Enum.join

    if message == "", do: to_string(number), else: message
  end
end
