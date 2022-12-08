// See https://aka.ms/new-console-template for more information
//

//using BenchmarkDotNet.Running;
//var summary = BenchmarkRunner.Run<DayEleven.MapReadingTests>();
//return;

//var input = "../sample_input";
var input = "../input";

var mapLines = File.ReadAllLines(input);

var width = mapLines.First().Length;
var height = mapLines.Length;

var map = mapLines.SelectMany(line => line.ToCharArray().Select(c => (byte)(c-'0'))).ToArray();

int step(){
  var stepFlashes = 0;

  for(var i = 0; i < map.Length; i++) map[i]++;

  var flashed = new List<int>();

  bool flashIndex(int i){
    map[i] = 0;
    if(flashed. Contains(i)) return false;
    flashed.Add(i);

    // increase neighbors
    // ------------------
    // top: not for first line
    if(i >= width) {
      // top left
      if(i%width > 0) map[i-width-1]++;
      // top
      map[i-width]++;
      // top right
      if(i%width < width-1) map[i-width+1]++;
    }
      // right
    if(i%width < width-1) map[i+1]++;

    // bottom: not for last line
    if(i < map.Length-width){
      // bottom-right
      if(i%width < width-1) map[i+width+1]++;
      // bottom
      map[i+width]++;
      // bottom-left
      if(i%width > 0) map[i+width-1]++;
    }

    // left
    if(i%width > 0) map[i-1]++;
    return true;
  }

  while(true){
    var newFlash = false;
    for(var i = 0; i < map.Length; i++){
      if(map[i] > 9){
        var isFlash = flashIndex(i);
        if(isFlash) stepFlashes++;

        newFlash = newFlash || isFlash;
      }
    }
    if(!newFlash) break;
  }
  flashed.ForEach(i => map[i] = 0);

  return stepFlashes;
}

bool allFlashed() => map.All(v => v == 0);

void printForStep(int step) {
  Console.Write($"\n--- step {step} ---");
  for(var i = 0; i < map.Length; i++){
    if(i%width==0) Console.WriteLine();
    Console.Write(map[i]);
  }
  Console.WriteLine();
}

var totalFlashes = 0;
var f = 0;

while(true){
  totalFlashes += step();
  //if(f < 9 || (f+1)%10==0) printForStep(f+1);

  if(allFlashed()){
    Console.WriteLine($"All flashed in step {f+1}");
    printForStep(f+1);
    break;
  }
  f++;
}

Console.WriteLine($"\n{totalFlashes} in {f+1} steps");
