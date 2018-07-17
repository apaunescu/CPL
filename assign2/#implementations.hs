#implementations


--e2
divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n`div`2)],  n`mod`i  == 0]
primes :: Int -> [Int]
primes n = [i | i <- [2..(n)], divisors i == []]

join n [] = []
join n (a:[]) = a
join n (x:xs) = x ++ n ++ (join n xs)

pythagorean :: Int -> [(Int, Int, Int)]
pythagorean c = [(a,b,c) | a <- [1..(c)], b <- [1..(c)], c<-[1..(c)], (a^2)+(b^2) == (c^2), a<=b, b<=c]


--e4
hailSeq :: Integer -> [Integer]
hailSeq 1 = [1]
hailSeq n = n : hailSeq(hailstone n)

merge :: Ord(a) => [a] -> [a] -> [a]
merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys)
    | x >= y = (y : merge ys (x:xs))
    | y > x = (x : merge xs (y:ys))

--this is the one we need
mergeSort :: Ord(a) => [a] -> [a]
mergeSort[] = []
mergeSort[x] = [x]
mergeSort xs = merge (mergeSort (fst(splitAt(length xs `div` 2) xs))) (mergeSort (snd(splitAt(length xs `div` 2) xs)))


--e5
fib :: Integer -> Integer
fib 0 = 0
fib 1 = 1
fib n = fib(n-1) + fib(n-2)