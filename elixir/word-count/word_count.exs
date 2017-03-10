defmodule Words do
  @doc """
  Count the number of words in the sentence.

  Words are compared case-insensitively.
  """
  @spec count(String.t) :: map
  def count(sentence) do
    String.downcase(sentence)
    |> String.split(~r/[^[:alnum:]\-]/u, trim: true)
    |> Enum.group_by(&(&1))
    |> Enum.map(fn({k, v}) -> {k, length(v)} end)
    |> Map.new
  end
end
