// See https://aka.ms/new-console-template for more information

//var input = "../sample_input";
var input = "../input";

var mapLines = File.ReadAllLines(input);
var width = mapLines.First().Length;
var height = mapLines.Length;

var map = mapLines.SelectMany(l => l.ToCharArray().Select(c => c - '0')).ToArray();

IEnumerable<(int X, int Y, int V)> findNeighbors(int x, int y){
  if(x > 0) yield return (x-1,y,map[y*width + x - 1]);
  if(y > 0) yield return (x,y-1,map[(y-1)*width + x]);
  if(x < width-1) yield return (x+1,y,map[y*width + x + 1]);
  if(y < height-1) yield return (x,y+1,map[(y+1)*width + x]);
}

// continue while larger (but <9) neighbors are available
int floodBasin(int x, int y){
  var visited = new List<(int X, int Y, int V)>();
  var queue = new Queue<(int X, int Y, int V)>();

  queue.Enqueue((x,y, map[y*width+x]));

  do{

    var next = queue.Dequeue();
    if(visited.Contains(next)) continue;
    visited.Add(next);

    foreach(var n in findNeighbors(next.X, next.Y)){
      if(visited.Contains(n)) continue;
      if(n.V == 9) continue;
      if(n.V < next.V) continue;
      queue.Enqueue(n);
    }
  }while(queue.Count > 0);

  return visited.Count;
}

var lows = new List<(int X, int Y)>();
var totalRisk = 0;
for(var y = 0; y < height; y++)
for(var x = 0; x < width; x++){
  var c = map[y*width +x];

  var neighbors = findNeighbors(x,y);
  if(neighbors.All(n => c < n.V)){
    Console.WriteLine($"Found low spot at {x},{y} with depth {c}");
    lows.Add((x,y));
    totalRisk += 1 + c;
  }
}

var largestBasins = new []{0,0,0};

foreach(var low in lows){
  var basin = floodBasin(low.X, low.Y);

  var min = largestBasins.Min();
  if(min < basin){
    var minIdx = Array.IndexOf(largestBasins, min);
    largestBasins[minIdx] = basin;
  }
}

Console.WriteLine($"Total risk is {totalRisk}");
Console.WriteLine($"Three largest basins multiplied is {string.Join('*', largestBasins)} = {largestBasins.Aggregate(1, (agg, v) => agg*v)}");

