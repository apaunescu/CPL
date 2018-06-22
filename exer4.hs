
hailstone :: Integer -> Integer
hailstone n
	|n `mod` 2 == 0 = n `div` 2
	|n `mod` 2 /= 0 = 3*n + 1

hailSeq :: Integer -> [Integer]
hailSeq 1 = [1]
hailSeq n = n : hailSeq(hailstone n)

hailLen :: Integer -> Int
hailLen 1 = 0
hailLen n
    |n `mod` 2 == 0 = hailLen(n`div` 2) + 1
    |n `mod` 2 /= 0 = hailLen(3*n + 1) + 1 

hailSeq' :: Integer -> [Integer]
hailSeq' n = take ((hailLen n)+1) (iterate(hailstone) n)

join :: String -> [String] -> String
join n [] = []
join n (a:[]) = a
join n (x:xs) = x ++ n ++ foldl1(delim) xs
	where delim a b = a ++ n ++ b

merge :: Ord(a) => [a] -> [a] -> [a]
merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys)
    | x >= y = (y : merge ys (x:xs))
    | y > x = (x : merge xs (y:ys))


mergeSort :: Ord(a) => [a] -> [a]
mergeSort[] = []
mergeSort[x] = [x]
mergeSort xs = merge (mergeSort (fst(splitAt(length xs `div` 2) xs))) (mergeSort (snd(splitAt(length xs `div` 2) xs)))

--first_b (b:_) = b

--findElt :: Ord(a) => a -> [a] -> Maybe a
findElt n (x:xs) = maybe "" show (Just n)

--lastButOne :: Ord(a) => a -> [a] -> Maybe a
--lastButOne n [] = Nothing
--lastButOne n [_] = Just n
--lastButOne n [x,_] = Just x
--lastButOne n (_:xs) = lastButOne n xs


--	maybe "" show (Just n)



