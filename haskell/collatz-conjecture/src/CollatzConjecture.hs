module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz num
    | num <= 0 = Nothing
    | otherwise = Just $ collatzCount num 0
        where
            collatzCount x counter
                | x == 1 = counter
                | x `rem` 2 == 0 = collatzCount (x `div` 2) (counter + 1)
                | otherwise = collatzCount (x * 3 + 1) (counter + 1)
