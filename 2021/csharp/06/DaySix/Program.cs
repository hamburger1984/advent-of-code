// See https://aka.ms/new-console-template for more information
//

using System.IO;

//var input = "../sample_input";
var input = "../input";

var defaultTimeout = 6;
var newTimeout = 8;

var timerCounts = new long[newTimeout+1];

foreach(var value in File.ReadAllText(input).Split(',').Select(t => int.Parse(t)))
  timerCounts[value]++;

var fishes = timerCounts.Sum();

var days = 256;
var daysLeft = days;

while(daysLeft-- > 0){
  var newFishes = timerCounts[0];
  for(var i = 0; i < timerCounts.Length-1; i++)
    timerCounts[i] = timerCounts[i+1];

  timerCounts[defaultTimeout] += newFishes;
  timerCounts[newTimeout] = newFishes;

  fishes += newFishes;
  Console.WriteLine($"{days-daysLeft,4} {fishes,15}");
}

