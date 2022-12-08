// See https://aka.ms/new-console-template for more information
//
using System.IO;


var input = "../input";
var aim = 0;
var horizontal = 0;
var depth = 0;

foreach(var line in File.ReadLines(input)){
  if(line.StartsWith("forward ")){
    var step = int.Parse(line["forward ".Length..]);
    horizontal += step;
    depth += aim*step;
    Console.WriteLine($"-> {horizontal} ({line})");
  } else if (line.StartsWith("up ")){
    var step = int.Parse(line["up ".Length..]);
    aim -= step;
    Console.WriteLine($"{aim} (-{step})");
  } else if (line.StartsWith("down ")){
    var step = int.Parse(line["down ".Length..]);
    aim += step;
    Console.WriteLine($"{aim} (+{step})");
  }
}

Console.WriteLine($"vv {depth} -> {horizontal} => {depth*horizontal}");
