// See https://aka.ms/new-console-template for more information
//

using System.IO;


var input = "../input";
//var input = "../sample_input";
//var lines = 0;
//int[] oneOnPosition = null;

var oxy = new List<string>();
var co2 = new List<string>();
var length = 0;

foreach(var line in File.ReadLines(input)){
  length = Math.Max(length, line.Length);

  /*
   * Part I
   *

  lines++;
  if(oneOnPosition == null) oneOnPosition = new int[line.Length];
  for(var i = 0; i < line.Length; i++){
    oneOnPosition[i] += line[i] - '0';
  }
  */

  oxy.Add(line);
  co2.Add(line);
}

/*
 * Part I
 *

// most common is 1 -> 1
int gamma = 0;

// most common is 0 -> 1
int epsilon = 0;

var half = lines / 2;
foreach (var one in oneOnPosition){
  gamma = (one > half?1:0) + 2*gamma;
  epsilon = (one > half?0:1) + 2*epsilon;
}
Console.WriteLine($"Gamma = {gamma} ({Convert.ToString(gamma, 2)}), Epsilon = {epsilon} ({Convert.ToString(epsilon, 2)})\n => Power Consumption = {gamma*epsilon}");
*/

Console.WriteLine("ALL\n" + string.Join("\n", oxy));
for(var i = 0; i < length; i++){
  if(oxy.Count > 1){
    var ones = oxy.Count(c => c[i] == '1');
    var zeros = oxy.Count - ones;

    if(ones >= zeros)
      oxy.RemoveAll(c => c[i] == '0');
    else
      oxy.RemoveAll(c => c[i] == '1');

    Console.WriteLine("Oxy " + i + " .. " + string.Join(", ", oxy));
  }

  if(co2.Count > 1){
    var ones = co2.Count(c => c[i] == '1');
    var zeros = co2.Count - ones;

    if(ones < zeros)
      co2.RemoveAll(c => c[i] == '0');
    else
      co2.RemoveAll(c => c[i] == '1');

    Console.WriteLine("CO2 " + i + " .. " + string.Join(", ", co2));
  }
}


var oxygenGen = Convert.ToInt32(oxy.First(), 2);
var co2Scrubb = Convert.ToInt32(co2.First(), 2);
Console.WriteLine($"Oxygen Generator = {oxygenGen}, CO2 Scrubber = {co2Scrubb}\n => Life Supply Rating = {oxygenGen*co2Scrubb}");
