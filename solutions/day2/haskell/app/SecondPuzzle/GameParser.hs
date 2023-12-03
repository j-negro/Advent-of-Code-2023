module GameParser (getGames, Game (..), GameSet, ColoredCube (..)) where

data ColoredCube = Blue Int | Red Int | Green Int
  deriving (Show)

type GameSet = [ColoredCube]

data Game = ID Int [GameSet] deriving (Show)

getGames :: IO [Game]
getGames = do
  content <- readFile "../input.txt"
  return (parseFile (lines content))

parseFile :: [String] -> [Game]
parseFile = map parseLine

parseLine :: String -> Game
parseLine line =
  let (gameId, setString) = getIDAndSets line
   in ID (parseGameID gameId) (parseSetString setString)

getIDAndSets :: String -> (String, String)
getIDAndSets str = case split ':' str of
  [gameId, setString] -> (gameId, setString)
  _ -> error "Parsing Error"

parseGameID :: String -> Int
parseGameID string = read (last (split ' ' string))

parseSetString :: String -> [GameSet]
parseSetString str = map parseGameSet (split ';' str)

parseGameSet :: String -> GameSet
parseGameSet gameSet =
  let colorsList = map words (split ',' gameSet)
   in map getColoredCube colorsList

getColoredCube :: [String] -> ColoredCube
getColoredCube [num, color] = buildColoredCube color (read num)
getColoredCube _ = error "Parsing Error"

buildColoredCube :: String -> Int -> ColoredCube
buildColoredCube "blue" = Blue
buildColoredCube "red" = Red
buildColoredCube "green" = Green
buildColoredCube _ = error "Unkwown color"

--

split :: Char -> String -> [String]
split _ "" = []
split c s = firstWord : split c rest
  where
    firstWord = takeWhile (/= c) s
    rest = drop (length firstWord + 1) s