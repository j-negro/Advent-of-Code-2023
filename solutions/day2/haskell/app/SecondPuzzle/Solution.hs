import GameParser

main :: IO ()
main = do
  games <- getGames
  print (sumOfPowers games)

sumOfPowers :: [Game] -> Int
sumOfPowers = foldr (\(ID _ gameSets) sumAgg -> minimumPowers gameSets + sumAgg) 0

minimumPowers :: [GameSet] -> Int
minimumPowers gs =
  let (Red x, Green y, Blue z) = minimumColors gs
   in x * y * z

minimumColors :: [GameSet] -> (ColoredCube, ColoredCube, ColoredCube)
minimumColors [] = (Red 0, Green 0, Blue 0)
minimumColors (gs : gss) =
  let (Red a, Green b, Blue c) = arrangeColors gs
      (Red x, Green y, Blue z) = minimumColors gss
   in (Red (max a x), Green (max b y), Blue (max c z))

arrangeColors :: [ColoredCube] -> (ColoredCube, ColoredCube, ColoredCube)
arrangeColors [] = (Red 0, Green 0, Blue 0)
arrangeColors ((Red n) : colors) = let (Red x, Green y, Blue z) = arrangeColors colors in (Red (n + x), Green y, Blue z)
arrangeColors ((Green n) : colors) = let (Red x, Green y, Blue z) = arrangeColors colors in (Red x, Green (n + y), Blue z)
arrangeColors ((Blue n) : colors) = let (Red x, Green y, Blue z) = arrangeColors colors in (Red x, Green y, Blue (n + z))