// See https://aka.ms/new-console-template for more information

using System.Text.RegularExpressions;

//target area: x=20..30, y=-10..-5
var input = "../sample_input";
//var input = "../input";

var regex = new Regex(@"x=(-?\d+)\.\.(-?\d+),\s+y=(-?\d+)\.\.(-?\d+)");
var m = regex.Match(File.ReadAllText(input));
var (x0, x1, y0, y1) = (long.Parse(m.Groups[1].Value),
                        long.Parse(m.Groups[2].Value),
                        long.Parse(m.Groups[3].Value),
                        long.Parse(m.Groups[4].Value));

var target = (X:Math.Min(x0, x1),
              Y:Math.Min(y0, y1),
              Width:Math.Abs(x0-x1),
              Height:Math.Abs(y0-y1));


var velocity = (dX:0L, dY:0L);

Console.WriteLine($"Target = {target}");
// Part I
maxHeightShot();

// Part II
allHittingShots();

void maxHeightShot(){
  // necessary X speed?
  // 2 ->             2 + 1 =  3
  // 3 ->         3 + 2 + 1 =  7
  // 4 ->     4 + 3 + 2 + 1 = 11
  // 5 -> 5 + 4 + 3 + 2 + 1 = 16
  // ... until horizontally in target
  var XX = 0L;
  while(true){
    velocity.dX++;
    XX += velocity.dX;
    if(target.X <= XX && XX <= target.X + target.Width)
      break;
  }

  // Y speed = vertical distance to target?
  // S......
  // .......
  // ....TTT
  // ....TTT
  // ....TTT
  //
  // 0 -> +0,0 _-1,0_ -2,-1  -3,-3  -4,-6 ..
  // 1 -> +1,0  +0,1  -1,1 _-2,0_ -3,-2
  // 2 -> +2,0  +1,2  +0,3  -1,3  -2,2 _-3,0_ -4,-3 ..
  // 3 -> +3,0  +2,3  +1,5  +0,6  -1,6  -2,5  -3,3 _-4,0_..
  velocity.dY = Math.Max(Math.Abs(y0), Math.Abs(y1))-1;

  Console.WriteLine($"Initial velocity = {velocity}");

  var step = 0;
  var position = (X:0L, Y:0L);
  var positions = new List<(long X, long Y)>{(position.X, position.Y)};

  var bottom = Math.Min(y0, y1);

  while(step < 500){
    position.X += velocity.dX;
    position.Y += velocity.dY;

    if(position.Y < Math.Min(y0, y1)){
      Console.WriteLine(" ... too low - stop");
      break;
    }

    //Console.WriteLine($"{position} moving at {velocity}");
    step++;
    positions.Add((position.X, position.Y));

    if(target.X <= position.X && position.X <= target.X + target.Width &&
        target.Y <= position.Y && position.Y <= target.Y + target.Height){
      Console.WriteLine("HIT!!");
      break;
    }

    velocity.dY -= 1;
    if(velocity.dX != 0) {
      velocity.dX += velocity.dX < 0 ? 1 : -1;
    }
  }

  printResult(positions, target);
}

void allHittingShots(){

}

void printResult(List<(long X, long Y)> positions,
    (long X, long Y, long Width, long Height) target){

  var minY = positions.Select(p=>p.Y).Union(new []{y0, y1}).Min();
  var minX = positions.Select(p=>p.X).Union(new []{x0, x1}).Min();
  var maxY = positions.Select(p=>p.Y).Union(new []{y0, y1}).Max();
  var maxX = positions.Select(p=>p.X).Union(new []{x0, x1}).Max();

  for(var y = maxY; y >= minY; y--){
  for(var x = minX; x <= maxX; x++){
    if(positions.Contains((x,y))){
      Console.Write('#');
    } else if(target.Y <= y && y <= target.Y + target.Height &&
        target.X <= x && x <= target.X + target.Width){
      Console.Write('T');
    } else {
      Console.Write('.');
    }
  }
  Console.WriteLine();
  }

  Console.WriteLine($"Reached max height {maxY}");
}

