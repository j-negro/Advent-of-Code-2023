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
numsPerLine [] = []
numsPerLine (c : cs) =
  if isDigit c
    then digitToInt c : numsPerLine cs
    else case parseStrToInt (c : cs) of
      Just x -> x : numsPerLine cs
      Nothing -> numsPerLine cs

parseStrToInt :: [Char] -> Maybe Int
parseStrToInt ('o' : 'n' : 'e' : _) = Just 1
parseStrToInt ('t' : 'w' : 'o' : _) = Just 2
parseStrToInt ('t' : 'h' : 'r' : 'e' : 'e' : _) = Just 3
parseStrToInt ('f' : 'o' : 'u' : 'r' : _) = Just 4
parseStrToInt ('f' : 'i' : 'v' : 'e' : _) = Just 5
parseStrToInt ('s' : 'i' : 'x' : _) = Just 6
parseStrToInt ('s' : 'e' : 'v' : 'e' : 'n' : _) = Just 7
parseStrToInt ('e' : 'i' : 'g' : 'h' : 't' : _) = Just 8
parseStrToInt ('n' : 'i' : 'n' : 'e' : _) = Just 9
parseStrToInt _ = Nothing