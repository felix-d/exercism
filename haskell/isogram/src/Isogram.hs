module Isogram (isIsogram) where

import Data.Char
import Data.List

isIsogram :: String -> Bool
isIsogram text =
    (length $ nub alpha) == length alpha
        where alpha = map toLower . filter isAlpha $ text
