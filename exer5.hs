pascal :: Integer -> [Integer]
pascal 0 = [1]
pascal 1 = [1,1]
pascal n = [1] ++ numbers ++ [1]
	where numbers = [(a+b) | (a,b)<-zipfunc]
		where zipfunc = zip prev (tail prev)
			where prev = pascal(n-1)

addPair :: (Integer,Integer) -> Integer
addPair = (uncurry (+))

withoutZeros :: (Eq a, Num a) => [a] -> [a]
withoutZeros = filter (/=0)

fib :: Integer -> Integer
fib 0 = 0
fib 1 = 1
fib n = fib(n-1) + fib(n-2)

fibs = map fib[0..]

things :: [Integer]
things = 0 : 1 : zipWith (+) things (tail things)