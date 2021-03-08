module Main where

import System.Environment
import System.IO
import Data.List
import Data.Time

type StampedDiff = (UTCTime, Int)

-- per leggere ogni riga dell'input
parseLine :: String -> UTCTime
parseLine = parseTimeOrError True defaultTimeLocale "%Y-%m-%d %H:%M:%S.000"


-- calcola le differenze temporali, tiene solo quelle standard
calcolaDifferenze :: [UTCTime] -> [StampedDiff]
calcolaDifferenze listaDate =
  filter 
    ((`elem` [60, 300, 600, 900, 1800, 3600, 7200, 10800, 14400, 21600, 43200, 86400]) . snd) $ -- il secondo elemento deve essere un intervallo previsto
    zipWith 
      (\data1 data2 -> (data1, round . nominalDiffTimeToSeconds $ diffUTCTime data2 data1))
      listaDate 
      (tail listaDate)

sameFreq x y = snd x == snd y
sameDay x y = (utctDay . fst) x  == (utctDay . fst) y

raggruppa :: [StampedDiff] -> [[StampedDiff]]
raggruppa = 
  filter ((/= []) . tail) .  -- commentare per togliere il filtro dei giorni singoli
  groupBy sameDay


minimoGiornaliero :: [StampedDiff] -> StampedDiff
minimoGiornaliero listaCoppie = (dataInizio , timeDiff)
  where
    dataInizio = (head . map fst) listaCoppie
    timeDiff = (minimum . map snd) listaCoppie


periodi :: [StampedDiff] -> [StampedDiff]
periodi = map head . groupBy sameFreq


format :: StampedDiff -> String
format (t, td) = show t ++ " " ++ show td


main :: IO ()
main = do
  args <- getArgs
  dates <- readFile $ head args

  let groupedList = 
        map minimoGiornaliero . 
        raggruppa . 
        calcolaDifferenze . 
        map parseLine . 
        lines $ dates 

  writeFile (head args ++ "-plottable") $ 
    unlines . map format $ 
    groupedList

  writeFile (head args ++ "-changes") $ 
    unlines . map format . periodi $ 
    groupedList



