import Data.Time.Calendar
import Data.Time.Calendar.OrdinalDate


merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys)
    | x >= y = (y : merge ys (x:xs))
    | y > x = (x : merge xs (y:ys))

hailLen n = hailTail 0 n
    where 
        hailTail a 1 = a
        hailTail a n
            |n `mod` 2 == 0 = hailTail (a + 1) (n `div` 2)
            |n `mod` 2 /= 0 = hailTail (a + 1) (3*n + 1)

fact 0 = 1
fact 1 = 1
fact n = n * fact(n-1)

fact' n = foldl (*) n [1..n-1]

daysInYear :: Integer -> [Day]
daysInYear y = [fromGregorian y 1 1.. fromGregorian y 12 31]
    where 
        jan1 = fromGregorian y 1 1
        dec31 = fromGregorian y 12 31

isFriday :: Day -> Bool
isFriday d
    |5 == snd (mondayStartWeek d) = True
    |otherwise = False

divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n`div`2)],  n`mod`i  == 0]
primes :: Int -> [Int]
primes n = [i | i <- [2..(n)], divisors i == []]

getDay (y,m,d) = d
        
isPrimeDay :: Day -> Bool
isPrimeDay d 
    |divisors(getDay(toGregorian d)) == [] = True
    |otherwise = False


primeFridays :: Integer -> [Day]
primeFridays y = [i | i <- daysInYear y, isFriday i  == True && isPrimeDay i == True]