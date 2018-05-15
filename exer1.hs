det a b c = b^2 - 4*a*c
quadsol1 a b c = (-b - sqrt (det a b c))/2*a
quadsol2 a b c = (-b + sqrt (det a b c))/2*a
third_a a = a !! 2
third_b (_:_:b:_) = b
hailstone n
    |n `mod` 2 == 0 = n `div` 2
    |n `mod` 2 /= 0 = 3*n + 1