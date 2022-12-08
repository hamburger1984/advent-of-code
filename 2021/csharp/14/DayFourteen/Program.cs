// See https://aka.ms/new-console-template for more information

//var input = "../sample_input";
var input = "../input";

var rules = new Dictionary<(char, char), char>();
var existingPairs = new Dictionary<(char, char), long>();
var template = string.Empty;
foreach(var line in File.ReadLines(input)){
  if(string.IsNullOrEmpty(line)) continue;

  if(line.Contains(" -> ")){
    var rule = line.Split(" -> ");
    rules.Add((rule[0][0], rule[0][1]), rule[1][0]);
  } else {
    template = line;
  }
}

var symbols = template.ToCharArray().Union(rules.SelectMany(p => new []{p.Key.Item1, p.Key.Item2, p.Value})).Distinct().ToArray();
foreach(var s in symbols)
foreach(var t in symbols)
  existingPairs[(s,t)] = 0;

Console.WriteLine($"Template ({template}):");
for(var i = 0; i < template.Length-1; i++)
  existingPairs[(template[i], template[i+1])]++;
foreach(var p in existingPairs.Where(p => p.Value > 0))
  Console.WriteLine($" {p.Value,3} {p.Key}");

Console.WriteLine("Rules:");
foreach(var rule in rules){
  Console.WriteLine($"     {rule.Key} -> {rule.Value}");
}


var step = 0;
var steps = 40;
var nextPairs = new Dictionary<(char, char), long>();

while(true){
  step++;
  nextPairs.Clear();

  foreach(var p in existingPairs){
    if(rules.TryGetValue(p.Key, out var insert)){
      var k1 = (p.Key.Item1, insert);
      var k2 = (insert, p.Key.Item2);
      if(nextPairs.ContainsKey(k1)) nextPairs[k1] += p.Value;
      else nextPairs[k1] = p.Value;
      if(nextPairs.ContainsKey(k2)) nextPairs[k2] += p.Value;
      else nextPairs[k2] = p.Value;
    } else {
      if(nextPairs.ContainsKey(p.Key)) nextPairs[p.Key] += p.Value;
      else nextPairs[p.Key] = p.Value;
    }
  }
  existingPairs = new Dictionary<(char, char), long>(nextPairs);

  if(step >= steps) break;
}

var charCounts = existingPairs.Select(p => (Char:p.Key.Item1, Count:(long)p.Value)).Append((Char:template[template.Length-1], Count:1L));

var hist = charCounts.GroupBy(p => p.Char).Select(g => (Char:g.Key, Count:(long)g.Sum(p => (long)p.Count))).OrderBy(h => h.Count).ToArray();

Console.WriteLine("\nLeast to most common element:");
foreach(var h in hist){
  Console.WriteLine($" {h.Count,20:#,###} {h.Char}");
}

Console.WriteLine($" >> {(hist.Last().Count - hist.First().Count):#,###}");
