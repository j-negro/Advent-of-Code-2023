import Data.Char (digitToInt, isDigit)

main :: IO ()
main = do
  result <- solve
  print result

solve :: IO Int
solve = do
  content <- readFile "../input.txt"
  return (parseFile (lines content))

parseFile :: [String] -> Int
parseFile = foldr (\s sumAgg -> parseLine s + sumAgg) 0

parseLine :: [Char] -> Int
parseLine cs =
  let nums = numsPerLine cs
   in 10 * head nums + last nums

numsPerLine :: [Char] -> [Int]
numsPerLine = foldr (\x h -> if isDigit x then digitToInt x : h else h) []
