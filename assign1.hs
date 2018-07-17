import RainbowAssign
import qualified Data.Map as Map
import Data.Maybe as Maybe

pwLength, nLetters, width, height :: Int
filename :: FilePath
pwLength = 8            -- length of each password
nLetters = 5            -- number of letters to use in passwords: 5 -> a-e
width = 40              -- length of each chain in the table
height = 1000           -- number of "rows" in the table
filename = "table.txt"  -- filename to store the table

convert :: Int -> [Int]
convert n = (n `mod` nLetters) : (convert(n `div` nLetters))

passConvert :: [Int] -> Passwd
passConvert [] = []
passConvert (x:xs) = toLetter x : (passConvert xs)

pwReduce :: Hash -> Passwd
pwReduce n = passConvert values
	where values = reverse (take pwLength (convert (fromEnum n)))


rainbowTable:: Int -> [Passwd] -> Map.Map Hash Passwd
rainbowTable n xs = Map.fromList (zip hash xs)
    where hash = valueHasher n xs

valueHasher:: Int -> [Passwd] -> [Hash]
valueHasher 0 xs = hashes xs
valueHasher n xs = valueHasher (n-1) (retryHash xs)

retryHash :: [Passwd] -> [Passwd]
retryHash xs = [pwReduce i | i <- myHash]
	where myHash = [pwHash i | i <- xs]

hashes :: [Passwd] -> [Hash]
hashes xs = [pwHash i | i <- xs]

findPassword :: Map.Map Hash Passwd -> Int -> Hash -> Maybe Passwd
findPassword table n hash = case (findHashCol n hash table) of
	Nothing -> Nothing
	Just x -> searhValue n x hash


findHashCol :: Int -> Hash -> Map.Map Hash Passwd -> Maybe Passwd
findHashCol (-1) xs table = Nothing
findHashCol n xs table = case (Map.lookup xs table) of
	Nothing -> findHashCol (n-1) (pwHash(pwReduce(xs))) table
	Just x -> Just x

searhValue :: Int -> Passwd -> Hash -> Maybe Passwd
searhValue (-1) xs hash = Nothing
searhValue n xs hash
	|pwHash(xs) == hash = Just xs
	|otherwise = searhValue(n-1) (pwReduce(pwHash(xs))) hash

generateTable :: IO ()
generateTable = do
  table <- buildTable rainbowTable nLetters pwLength width height
  writeTable table filename

test2 :: Int -> IO ([Passwd], Int)
test2 n = do
  table <- readTable filename
  pws <- randomPasswords nLetters pwLength n
  let hs = map pwHash pws
  let result = Maybe.mapMaybe (findPassword table width) hs
  return (result, length result)

main :: IO ()
main = do
  generateTable
  res <- test2 10000
  print res