module Bob (responseFor) where

import Data.Char
import Data.List

trim :: String -> String
trim = dropWhileEnd isSpace . dropWhile isSpace

isQuestion :: String -> Bool
isQuestion xs = last xs == '?'

isYelling :: String -> Bool
isYelling xs = (not $ any isLower xs) && (any isUpper xs)

isEmpty :: String -> Bool
isEmpty xs = all isSpace xs

responseFor :: String -> String
responseFor "" = "Fine. Be that way!"
responseFor xs
    | isEmpty xst = "Fine. Be that way!"
    | isYelling xst && isQuestion xst = "Calm down, I know what I'm doing!"
    | isQuestion xst = "Sure."
    | isYelling xst =  "Whoa, chill out!"
    | otherwise = "Whatever."
    where xst = trim xs
