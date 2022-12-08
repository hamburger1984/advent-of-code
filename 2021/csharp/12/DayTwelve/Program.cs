// See https://aka.ms/new-console-template for more information


//var input = "../sample_input";
var input = "../input";

var rawEdges = File.ReadAllLines(input)
  .Where(l => !l.StartsWith('#'))
  .SelectMany(l =>
  {
      var p = l.Split('-');
      return new []{(Start: p[0], End: p[1]), (Start: p[1], End: p[0])};
  }).ToArray();

var edges = rawEdges.GroupBy(e => e.Start)
  .ToDictionary(g => g.Key, g => g.Select(e => e.End).Where(n => n!="start").ToArray());
edges.Remove("end");

var nodes = edges.Keys.ToArray();
Console.WriteLine($"Nodes: {string.Join(',', nodes)}");



Console.WriteLine($"Edges:");
foreach(var e in edges)
  Console.WriteLine($" {e.Key} -> {string.Join(',', e.Value)}");

var paths = new List<string>();

IEnumerable<string> findPaths(List<string> aggregate, string node, bool smallVisited){
  foreach(var dest in edges[node]){
    if(dest == "end"){
      yield return string.Join(',', aggregate.Prepend("start").Append(dest));
      continue;
    }
    var visitingSmall = false;

    // can revisit only one small cave
    if(char.IsLower(dest[0]) && aggregate.Contains(dest)){
      if(smallVisited)
        continue;
      visitingSmall = true;
    }

    foreach(var next in findPaths(new List<string>(aggregate){dest}, dest, smallVisited || visitingSmall))
      yield return next;
  }
}

foreach(var n in edges["start"]){
  var init = new List<string>{n};

  foreach(var path in findPaths(init, n, false))
    paths.Add(path);
}

Console.WriteLine($"{paths.Count} Paths:");
foreach(var p in paths.Take(30))
  Console.WriteLine(p);
if(paths.Count > 30)
  Console.WriteLine("..");
