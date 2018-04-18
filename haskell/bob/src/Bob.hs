module Bob (responseFor) where

import Data.Char

responseFor :: String -> String
responseFor xs
  | null trimmed = "Fine. Be that way!"
  | not (null alpha) && all isUpper alpha = "Whoa, chill out!"
  | last trimmed == '?' = "Sure."
  | otherwise = "Whatever."
  where trimmed = filter (not . isSpace) xs
        alpha = filter isAlpha xs
