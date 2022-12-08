// See https://aka.ms/new-console-template for more information
//

//var input = "../sample_input";
var input = "../input";

var coordinates = File.ReadAllLines(input)
  .Select(l => l.Split(" -> "))
  .SelectMany(t => t.SelectMany(u => u.Split(',')))
  .Select(c => int.Parse(c)).ToArray();

var lines = coordinates.Chunk(4).Select(line => (X1:line[0], Y1:line[1], X2:line[2], Y2:line[3])).ToArray();

var maxX = lines.Max(l => Math.Max(l.X1, l.X2));
var maxY = lines.Max(l => Math.Max(l.Y1, l.Y2));

var map = new int[maxX+1,maxY+1];

foreach(var line in lines){
  Console.WriteLine(line);
  if(line.X1==line.X2){
    Console.WriteLine(" vertical");
    var x = line.X1;
    for(var y = Math.Min(line.Y1, line.Y2); y <= Math.Max(line.Y1,line.Y2); y++){
      map[x,y]++;
    }
  }
  if(line.Y1==line.Y2){
    Console.WriteLine(" horizontal");
    var y = line.Y1;
    for(var x = Math.Min(line.X1, line.X2); x <= Math.Max(line.X1,line.X2); x++){
      map[x,y]++;
    }
  }
  if(line.X1!=line.X2 && line.Y1!=line.Y2){
    Console.WriteLine(" diagonal");
    var stepX = line.X1 < line.X2?1:-1;
    var stepY = line.Y1 < line.Y2?1:-1;

    for(var (x,y) = (line.X1,line.Y1); x != line.X2+stepX; x += stepX, y += stepY)
      map[x,y]++;
  }
}

var overlappingPoints = 0;

for(var y = 0; y < maxY+1; y++){
for(var x = 0; x < maxX+1; x++){
  var cell = map[x,y];
  Console.Write(cell==0?"_":cell);

  if(cell > 1) overlappingPoints++;
}
Console.WriteLine();
}


Console.WriteLine($"Got {overlappingPoints} overlapping points.");
