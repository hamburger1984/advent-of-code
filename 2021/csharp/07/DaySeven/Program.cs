// See https://aka.ms/new-console-template for more information
//

using System.Collections.Concurrent;
using System.Diagnostics;
using System.IO;

//var input = "../sample_input";
var input = "../input";

var positions = File.ReadAllText(input).Split(',').Select(t => int.Parse(t)).ToArray();

var maxX = positions.Max();

int costFromDistance(int distance){

  var stepCost = 1;
  var cost = 0;
  while(distance > 0){
    distance--;
    cost+=stepCost;
    stepCost++;
  }
  return cost;
}

int moveAllTo(int target){
  var cost = 0;
  foreach(var pos in positions){
    if(pos < target)
      cost += costFromDistance(target-pos);
    if(pos > target)
      cost += costFromDistance(pos-target);
  }
  return cost;
}

var swP = Stopwatch.StartNew();
ConcurrentBag<(int Cost, int Position)> results = new();
await Task.WhenAll(Enumerable.Range(0, maxX).Select(i =>
{
  results.Add((Cost:moveAllTo(i), Position:i));
  return Task.CompletedTask;
}));
var bestP = results.ToArray().OrderBy(p => p.Cost).First();
swP.Stop();
Console.WriteLine($"Best position: {bestP.Position}, costing {bestP.Cost}, took {swP.Elapsed}");


var sw = Stopwatch.StartNew();
var best = (Cost:int.MaxValue, Position:-1);
for(var i = 0; i <= maxX; i++){
  var cost = moveAllTo(i);
  if(best.Cost > cost){
    best = (Cost:cost, Position:i);
  }
}
sw.Stop();
Console.WriteLine($"Best position: {best.Position}, costing {best.Cost}, took {sw.Elapsed}");
