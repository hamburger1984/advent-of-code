// See https://aka.ms/new-console-template for more information
//
using DaySixteen;

//var input = "../sample_input";
var input = "../input";

foreach(var hex in File.ReadAllLines(input)){
  if(hex.StartsWith('#')) continue;

  var bits = new Hex2Bits(hex);

  bool[] Take(Hex2Bits bits, int num){
    var result = new bool[num];
    for(var i = 0; i < num; i++){
      var value = bits.Next();
      result[i] = value;
    }
    return result;
  }

  long Value(IEnumerable<bool> bits){
    var result = 0L;
    foreach(var x in bits){
      result = result * 2 + (x ? 1 : 0);
    }
    return result;
  }

  void PrintType(int type){
    var desc = type switch{
      0 => "SUM",
      1 => "MUL",
      2 => "MIN",
      3 => "MAX",
      4 => "VAL",
      5 => "GT",
      6 => "LT",
      7 => "EQ",
      _ => "????"
    };
    Console.Write($", T={type} {desc}");
  }

  long Eval(int type, List<long> values)
    => type switch{
      0 => values.Sum(),
      1 => values.Aggregate((a, b) => a*b),
      2 => values.Min(),
      3 => values.Max(),
      5 => values[0] > values[1] ? 1 : 0,
      6 => values[0] < values[1] ? 1 : 0,
      7 => values.Distinct().Count() == 1 ? 1 : 0,
      _ => throw new Exception("Unknown operation to evaluate")
    };

  long parsePackage(){
    var version = (int)Value(Take(bits, 3));
    Console.Write($"Version={version}");

    var type = (int)Value(Take(bits, 3));
    PrintType(type);

    if(type == 4){
      // literal
      List<bool> literal = new List<bool>();
      bool hasMore = false;
      do{
        hasMore = bits.Next();
        literal.AddRange(Take(bits, 4));
      } while(hasMore);
      var value = Value(literal);
      Console.WriteLine($", Value={value}");
      return value;
    } else {
      // operator
      var I = bits.Next();
      Console.Write($", I={(I?1:0)}");
      var subValues = new List<long>();

      if(I){ // length type
        // 1 -> 11 bits number of sub-packets in this packet
        var subPackets = Value(Take(bits, 11));
        Console.WriteLine($", SubPkgs={subPackets}");
        for(var i = 0; i < subPackets; i++)
        {
          subValues.Add(parsePackage());
        }
      } else {
        // 0 -> 15 bits total length in bits of sub-packets
        var subBits = Value(Take(bits, 15));
        Console.WriteLine($", SubBits={subBits}");
        var start = bits.Position;
        var endBits = start.Bit + start.Byte*8 + subBits;
        var endByte = endBits / 8;
        var endBit = endBits % 8;
        while(!bits.EOF() && (bits.Position.Byte < endByte ||
              (bits.Position.Byte == endByte && bits.Position.Bit < endBit)))
        {
          subValues.Add(parsePackage());
        }
      }

      return Eval(type, subValues);
    }
  }

  var eval = parsePackage();

  Console.WriteLine($" >> EVAL = {eval}");
}
