// See https://aka.ms/new-console-template for more information


(int Width, int Height, byte[] Data) readMap(){
  //var input = "../sample_input";
  var input = "../input";

  var width = 0;
  var height = 0;
 
  var dataList = new List<byte>();
  foreach(var line in File.ReadLines(input)){
    height++;
    if(width == 0)
      width = line.Length;
 
    dataList.AddRange(line.ToCharArray().Select(c => (byte)(c-'0')));
  }

  return (width, height, dataList.ToArray());
}

(int Width, int Height, byte[] Data) expandMap(
    (int Width, int Height, byte[] Data) map, int expansions){
  var newWidth = map.Width*(expansions+1);
  var newHeight = map.Height*(expansions+1);
  var newData = new byte[newWidth*newHeight];

  for(var y = 0; y < map.Height; y++){
    Array.ConstrainedCopy(map.Data, y*map.Width, newData, y*newWidth, map.Width);

    if(expansions == 0) continue;

    var line = new Span<byte>(map.Data, y*map.Width, map.Width);
    // copy + increment by ex to the right
    for(var ex = 1; ex <= expansions; ex++){
      var rightStart = (y*newWidth)+(ex*map.Width);
      for(var x = 0; x < line.Length; x++){
        var val = line[x]+ex;
        newData[rightStart+x] = (byte)(val > 9? (val % 10)+1:val);
      }
    }
    // copy + increment on the right edge downwards
    for(var ex = expansions+1; ex <= expansions*2; ex++){
      var rightStart = (y+(ex-expansions)*map.Height)*newWidth + newWidth - map.Width;

      for(var x = 0; x < line.Length; x++){
        var val = line[x]+ex;
        newData[rightStart+x] = (byte)(val > 9? (val % 10)+1:val);
      }
    }
    for(var ex = 0; ex < expansions; ex++){
      Array.ConstrainedCopy(newData, map.Width + (y+ex*map.Width)*newWidth, newData, (y+(ex+1)*map.Width)*newWidth, newWidth-map.Width);
    }
  }
  return (newWidth, newHeight, newData);
}


(int X, int Y) toPoint(int index, int width){
  return (index%width, (int)Math.Floor(index/(double)width));
}

int toIndex((int X, int Y) p, int width) {
  return p.Y * width + p.X;
}

void printMap((int Width, int Height, byte[] Data) map){
  for(var y = 0; y < map.Height; y++){
    for(var x = 0; x < map.Width; x++){
      Console.Write(map.Data[toIndex((x,y), map.Width)]);
    }
    Console.WriteLine();
  }
}

float manhattan((int X, int Y) p1, (int X, int Y) p2){
  return Math.Abs(p1.X - p2.X) + Math.Abs(p1.Y - p2.Y);
}

void aStar((int Width, int Height, byte[] Data) map){
  var startIndex = 0;
  var startPoint = toPoint(startIndex, map.Width);
  var goalIndex = map.Data.Length-1;
  var goalPoint = toPoint(goalIndex, map.Width);

  var temp = Enumerable.Range(0, map.Data.Length).Select(
      i => new
      {
        G = 0f,
        H = 0f,
        Visited = false,
        Closed = false,
        Parent = -1
      }).ToArray();

  var open = new PriorityQueue<int, double>();

  IEnumerable<int> neighbors(int index){
    // top
    if(index > map.Width) yield return index - map.Width;
    // right
    if(index % map.Width < map.Width-1) yield return index + 1;
    // bottom
    if(index < map.Data.Length - map.Width) yield return index + map.Width;
    // left
    if(index % map.Width > 0) yield return index - 1;
  }

  void printPath(){
    var i = goalIndex;
    var path = new Stack<(int X, int Y)>();

    while(i != -1){
      path.Push(toPoint(i, map.Width));
      i = temp[i].Parent;
    }

    while(path.Count > 0)
      Console.Write($" -> {path.Pop()}");
    Console.WriteLine();
  }

  temp[startIndex] = temp[startIndex] with {
    H = manhattan(startPoint, goalPoint),
    Visited = true
  };
  open.Enqueue(startIndex, temp[startIndex].H);

  while(open.Count > 0){
    var current = open.Dequeue();

    if(current == goalIndex){
      // TODO: found a path.
      printPath();
      Console.WriteLine($"\nDONE {temp[current]}");
      break;
    }

    temp[current] = temp[current] with { Closed = true };
    Console.Write($"\rprocessing neighbors of {current} .. {temp[current]}");

    foreach(var next in neighbors(current)){
      var nextT = temp[next];
      if(nextT.Closed) continue;

      var newG = temp[current].G + map.Data[next];
      var newH = nextT.H > 0 ? nextT.H : manhattan(toPoint(next, map.Width), goalPoint);
      var wasVisited = temp[next].Visited;

      if(!wasVisited || newG < nextT.G){
        temp[next] = nextT with { Visited = true, Parent = current, G = newG, H = newH };
        Console.Write($"\r > {next} .. {temp[next]}");

        if(!wasVisited) open.Enqueue(next, newG + nextT.H);
        // can we just change the priority instead?
        else open.Enqueue(next, newG + nextT.H);
      }
    }
  }
}

var map = readMap();

//printMap(map);

map = expandMap(map, 4);

//printMap(map);

aStar(map);

