merge :: Ord a => [a] -> [a] -> [a]
merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys)
	| x >= y = (y : merge ys (x:xs))
	| y > x = (x : merge xs (y:ys))


hailLen 1 = 0
hailLen n = hailTail 1 n
	where 
		hailTail a _ = _
		hailTail a _ = hailTail _ _


hailLen 1 = 0
hailLen n
    |n `mod` 2 == 0 = hailLen(n`div` 2) + 1
    |n `mod` 2 /= 0 = hailLen(3*n + 1) + 1 




join n [] = []
join n (a:[]) = a
join n (x:xs) = x ++ n ++ (join n xs)
