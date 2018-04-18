module Pangram (isPangram) where

import Data.Char
import Data.List

isPangram :: String -> Bool

isPangram text =
    length set == 26
        where alpha = filter isAlpha text
              lower = map toLower alpha
              set = nub lower
