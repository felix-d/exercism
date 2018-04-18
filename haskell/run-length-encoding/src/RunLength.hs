module RunLength (decode, encode) where

import Data.Char

decode :: String -> String
decode "" = ""
decode encodedText = replicate n c ++ decode rest
    where
        i = takeWhile isDigit encodedText
        n = if null i then 1 else read i
        c = head $ filter (not . isDigit) encodedText
        rest = drop (1 + length i) encodedText

encode :: String -> String
encode "" = ""
encode decodedText = n ++ c : encode rest
    where
        c = head decodedText
        l = length $ takeWhile (==c) decodedText
        n = if l == 1 then "" else show l
        rest = drop l decodedText
