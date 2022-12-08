// See https://aka.ms/new-console-template for more information

//var input = "../sample_input";
var input = "../input";

var points = 0;
var completionScores = new List<long>();

foreach(var line in File.ReadLines(input)){
  var opened = new Stack<char>();
  var discard = false;
  foreach(var c in line){
    if(c is '<' or '(' or '[' or '{')
    {
      opened.Push(c);
      continue;
    }
    var opener = opened.Pop();
    if(c == '>' && opener != '<'){
      points += 25137;
      discard = true;
      Console.WriteLine($"Got '{c}' to close '{opener}'");
      break;
    }
    if(c == ')' && opener != '('){
      points += 3;
      discard = true;
      Console.WriteLine($"Got '{c}' to close '{opener}'");
      break;
    }
    if(c == ']' && opener != '['){
      points += 57;
      discard = true;
      Console.WriteLine($"Got '{c}' to close '{opener}'");
      break;
    }
    if(c == '}' && opener != '{'){
      points += 1197;
      discard = true;
      Console.WriteLine($"Got '{c}' to close '{opener}'");
      break;
    }
  }

  if(discard) continue;

  var completionScore = 0L;

  while(opened.Count > 0){
    var c = opened.Pop();
    completionScore = completionScore * 5 + c switch{
      '<' => 4,
      '(' => 1,
      '[' => 2,
      '{' => 3,
      _ => throw new ArgumentOutOfRangeException($"unexpected value '{c}'")
    };
  }

  Console.WriteLine(completionScore);
  completionScores.Add(completionScore);
}

completionScores.Sort();

Console.WriteLine($">> Error score {points}");
Console.WriteLine($">> Completion scores mid {completionScores[completionScores.Count/2]}");
