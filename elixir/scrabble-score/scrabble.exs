defmodule Scrabble do
  @doc """
  Calculate the scrabble score for the word.
  """
  @spec score(String.t) :: non_neg_integer
  def score(word) do
    word
    |> String.upcase
    |> String.codepoints
    |> Enum.map(&(score_by_letter(&1)))
    |> Enum.sum
  end

  def score_by_letter(letter) do
    case letter do
      n when n in ["A", "E", "I", "O", "U", "L", "N", "R", "S", "T"] -> 1
      n when n in ["D", "G"] -> 2
      n when n in ["B", "C", "M", "P" ] -> 3
      n when n in ["F", "H", "V", "W", "Y"] -> 4
      n when n in ["K"] -> 5
      n when n in ["J", "X"] -> 8
      n when n in ["Q", "Z"] -> 10
      _ -> 0
    end
  end
end
