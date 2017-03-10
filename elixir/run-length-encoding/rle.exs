defmodule RunLengthEncoder do
  @doc """
  Generates a string where consecutive elements are represented as a data value and count.
  "HORSE" => "1H1O1R1S1E"
  For this example, assume all input are strings, that are all uppercase letters.
  It should also be able to reconstruct the data into its original form.
  "1H1O1R1S1E" => "HORSE"
  """
  @spec encode(String.t) :: String.t
  def encode(string) do
    string
    |> String.codepoints
    |> parse([])
    |> Enum.reduce("", fn({char, count}, res) -> "#{count}#{char}#{res}" end)
  end

  def parse([], result), do: result
  def parse([h1|t1], []), do: parse(t1, [{h1, 1}])
  def parse([h1|t1], [{h1, freq}|t2]), do: parse(t1, [{h1, freq + 1}|t2])
  def parse([h1|t1], result), do: parse(t1, [{h1, 1}|result])

  @spec decode(String.t) :: String.t
  def decode(string) do
    Regex.scan(~r/\d+|\D+/, string)
    |> List.flatten
    |> Enum.map_every(2, &(String.to_integer(&1)))
    |> build_string("")
  end

  defp build_string([], result), do: result
  defp build_string([num, char|tail], result) do
    build_string(tail, result <> String.duplicate(char, num))
  end
end
