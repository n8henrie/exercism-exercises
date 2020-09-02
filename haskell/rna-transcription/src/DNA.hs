module DNA (toRNA) where

translate :: Char -> Either Char Char
translate 'G' = Right 'C'
translate 'C' = Right 'G'
translate 'T' = Right 'A'
translate 'A' = Right 'U'
translate c   = Left c

toRNA :: String -> Either Char String
toRNA = traverse translate
