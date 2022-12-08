// See https://aka.ms/new-console-template for more information


var pwd = Environment.CurrentDirectory;

Console.WriteLine(pwd);

var input = "../input";

var increaseCounter = 0;
var lastValue = -1;
var increasedSum = 0;
var lastSum = -1;
var window = new [] { 0, 0, 0 };
var i = 0;

foreach (var line in File.ReadLines(input))
{
    if (!int.TryParse(line, out var value))
    {
        Console.Error.WriteLine($"Failed to parse '{line}'");
        break;
    }

    Console.Write(value);

    window[i%3] = value;
    i++;

    if (lastValue == -1)
    {
        lastValue = value;
        continue;
    }

    if (lastValue < value)
    {
        increaseCounter++;
        Console.ForegroundColor = ConsoleColor.Green;
    }
    else
    {
        Console.ForegroundColor = ConsoleColor.Red;
    }

    Console.Write("\t" + increaseCounter);

    if(i<3){
      Console.WriteLine();
    } else{
      var sum = window.Sum();
      Console.ForegroundColor = ConsoleColor.White;
      Console.Write("\t" + sum);
      if(lastSum > 0 && lastSum < sum){
        increasedSum++;
        Console.ForegroundColor = ConsoleColor.Green;
      }
      else
      {
         Console.ForegroundColor = ConsoleColor.Red;
      }

      Console.WriteLine("\t" + increasedSum);
      lastSum = sum;
    }

    Console.ForegroundColor = ConsoleColor.White;
    lastValue = value;
}

Console.WriteLine(increaseCounter);
Console.WriteLine(increasedSum);
