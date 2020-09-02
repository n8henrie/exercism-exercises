module Pangram (isPangram) where

import Data.Char

isPangram :: String -> Bool
isPangram text = all (flip elem (map toLower text)) ['a'..'z']
