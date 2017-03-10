defmodule Bob do
  def screaming(input) do
    String.match?(input, ~r/^[^a-z]*\p{Lu}+[^a-z]*!?$/) ->
  end

  def hey(input) do
    cond do
      String.ends_with?(input, "?") ->
        "Sure."
      String.trim(input) == "" ->
        "Fine. Be that way!"
      screaming(input) ->
        "Whoa, chill out!"
      true -> "Whatever."
    end
  end
end
