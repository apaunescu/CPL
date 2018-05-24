hailLen :: (Ord n) => [n] -> n
hailLen [1] = 1
hailLen (1:n)
    |n `mod` 2 == 0 = n `div` 2
    |n `mod` 2 /= 0 = 3*n + 1
    where answer = hailLen n
