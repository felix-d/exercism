module IsbnVerifier (isbn) where

import Text.Regex.PCRE

isbn :: String -> Bool
isbn str
  | validateStr str = validateNum $ toNum str
  | otherwise = False

validateStr :: String -> Bool
validateStr str = length(str) == 13 && str =~ "^\\d+-\\d+-\\d+-(\\d+|X)$"

toNum :: String -> [Integer]
toNum str = map mapper $ filter (/= '-') str
    where mapper c
          | c == 'X' = 10
          | otherwise = digitToInt c

validateNum :: [Integer] -> Bool
validateNum nums = True

