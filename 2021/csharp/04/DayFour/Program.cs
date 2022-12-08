// See https://aka.ms/new-console-template for more information

using System.IO;

//var input = "../sample_input";
var input = "../input";

var lines = File.ReadAllLines(input);

var drawnNumbers = lines[0].Split(',').Select(t => int.Parse(t)).ToArray();
var boardNumbers = lines.Skip(1).Where(l => !string.IsNullOrEmpty(l)).SelectMany(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries)).Select(t => (false, int.Parse(t))).ToArray();

var columns = 5;
var rows = 5;
var boardSize = columns*rows;
var boards = boardNumbers.Length/boardSize;

bool isRowChecked(int board, int row){
  var start = board*boardSize + row*columns;

  return boardNumbers.Skip(start).Take(columns).All(bn => bn.Item1);
}

bool isColumnChecked(int board, int col){
  var start = board*boardSize + col;
  for(var row = 0; row < rows; row++){
    if(!boardNumbers[start+row*columns].Item1) return false;
  }
  return true;
}

var boardWins = new bool[boards];
int? lastBoard = null;
int? lastNumber = null;

foreach(var number in drawnNumbers){
  for(var i = 0; i < boardNumbers.Length; i++)
    if(boardNumbers[i].Item2 == number) boardNumbers[i].Item1 = true;

  Console.WriteLine($"{number,3} => {boardNumbers.Count(bn => bn.Item1)} checked on all boards");

  for(var board = 0; board < boards; board++){
    if(boardWins[board]) continue;

    for(var row = 0; row < rows; row++){
      if(isRowChecked(board, row)){
        boardWins[board] = true;
        Console.WriteLine($" >> ROW {board},{row} checked");
      }
    }
    for(var col = 0; col < columns; col++){
      if(isColumnChecked(board, col)){
        boardWins[board] = true;
        Console.WriteLine($" >> COL {board},{col} checked");
      }
    }

    if(boardWins.All(bw => bw)){
      lastBoard = board;
      lastNumber = number;
      break;
    }
  }
  if(lastBoard.HasValue) break;
}

Console.Write($"Board {lastBoard} was last to be checked when drawing {lastNumber}");

if(lastBoard.HasValue){
  var uncheckedBoardNumbers = boardNumbers.Skip(lastBoard.Value * boardSize).Take(boardSize).Where(bn => !bn.Item1).Select(bn => bn.Item2).Sum();
  var score = uncheckedBoardNumbers * lastNumber.Value;
  Console.WriteLine($" having a total of {uncheckedBoardNumbers} still unchecked\n => Score {score}");
}

