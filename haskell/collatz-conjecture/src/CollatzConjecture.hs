module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz n
  | n <= 0 = Nothing
  | otherwise = Just $ collatz' n

collatz' :: Integer -> Integer
collatz' 1 = 0
collatz' n = 1 + collatz' (if odd n then 3 * n + 1 else n `div` 2)
