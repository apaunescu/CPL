hailLen 1 = 0
hailLen n
    |n `mod` 2 == 0 = hailLen(n`div` 2) + 1
    |n `mod` 2 /= 0 = hailLen(3*n + 1) + 1 

divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n`div`2)],  n`mod`i  == 0]
primes :: Int -> [Int]
primes n = [i | i <- [2..(n)], divisors i == []]

join n [] = []
join n (a:[]) = a
join n (x:xs) = x ++ n ++ (join n xs)

pythagorean :: Int -> [(Int, Int, Int)]
pythagorean c = [(a,b,c) | a <- [1..(c)], b <- [1..(c)], c<-[1..(c)], (a^2)+(b^2) == (c^2), a<=b, b<=c]
