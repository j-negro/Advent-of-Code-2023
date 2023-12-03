import GameParser

main :: IO ()
main = do
  games <- getGames
  print (sumOfPossibles games)

sumOfPossibles :: [Game] -> Int
sumOfPossibles =
  foldr
    ( \(ID gameId gameSets) sumAgg ->
        ( if isGamePossible gameSets
            then gameId
            else 0
        )
          + sumAgg
    )
    0

isGamePossible :: [GameSet] -> Bool
isGamePossible = all isSetPossible

isSetPossible :: GameSet -> Bool
isSetPossible = all isColorPossible

-- 12 red cubes, 13 green cubes, and 14 blue cubes
isColorPossible :: ColoredCube -> Bool
isColorPossible (Red n) = n <= 12
isColorPossible (Green n) = n <= 13
isColorPossible (Blue n) = n <= 14
