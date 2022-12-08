// See https://aka.ms/new-console-template for more information


//var input = "../sample_input";
var input = "../input";

var dots = new HashSet<(int X, int Y)>();
var folds = new List<(char Axis, int Offset)>();

foreach(var line in File.ReadLines(input)){
  if(string.IsNullOrEmpty(line)) continue;

  if(line.StartsWith("fold along ")){
    var fold = line.Substring("fold along ".Length);
    folds.Add((Axis: fold[0], Offset: int.Parse(fold.Substring("x=".Length))));
  } else {
    var parts = line.Split(',').Select(coord => int.Parse(coord));
    dots.Add((X:parts.First(), Y: parts.Skip(1).First()));
  }
}

void dumpDots(){
  var height = dots.Select(d => d.Y).Max();
  var width = dots.Select(d => d.X).Max();
  
  for(var y = 0; y <= height; y++){
    for(var x = 0; x <= width; x++){
      if(dots.Contains((x,y))) Console.Write('#');
      else Console.Write('.');
    }
    Console.WriteLine();
  }
  dumpStats();
}

void dumpStats(){
  Console.WriteLine($">> {dots.Count} Dots");
}

Console.WriteLine("Dots:");
foreach(var dot in dots)
  Console.WriteLine(dot);
dumpDots();

Console.WriteLine("Folds:");
foreach(var fold in folds)
  Console.WriteLine(fold);


foreach(var fold in folds){
  if(fold.Axis == 'x'){
    var toMirror = dots.Where(d => d.X > fold.Offset).ToArray();
    dots.RemoveWhere(d => d.X > fold.Offset);

    foreach(var tm in toMirror)
      dots.Add((fold.Offset- (tm.X-fold.Offset), tm.Y));
  }
  if(fold.Axis == 'y'){
    var toMirror = dots.Where(d => d.Y > fold.Offset).ToArray();
    dots.RemoveWhere(d => d.Y > fold.Offset);

    foreach(var tm in toMirror)
      dots.Add((tm.X, fold.Offset- (tm.Y-fold.Offset)));
  }
  //dumpDots();
  dumpStats();
}

dumpDots();
