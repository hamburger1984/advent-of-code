// See https://aka.ms/new-console-template for more information
//

//var input = "../sample_input";
var input = "../input";

/*
 * "Real" Segments:
 *
 *   aaaa
 *  b    c
 *  b    c
 *   dddd
 *  e    f
 *  e    f
 *   gggg
 *
 */

var realSegments = new[]{
  "abcefg"  /*"abc efg" 6 <- 0 */,
  "cf"      /*"  c  f " 2 <- 1 */,
  "acdeg"   /*"a cde g" 5 <- 2 */,
  "acdfg"   /*"a cd fg" 5 <- 3 */,
  "bcdf"    /*" bcd f " 4 <- 4 */,
  "abdfg"   /*"ab d fg" 5 <- 5 */,
  "abdefg"  /*"ab defg" 6 <- 6 */,
  "acf"     /*"a c  f " 3 <- 7 */,
  "abcdefg" /*"abcdefg" 7 <- 8 */,
  "abcdfg"  /*"abcd fg" 6 <- 9 */,
  /* -----------------*/
};

/*
   segment a XX
   - exists in 0,2,3,5,6,7,8,9 -> 8 digits
   -    having 6,5,5,5,6,3,7,6 segments

   segment b XX
   - exists in 0,4,6,7,8,9 -> 6 digits
   -    having 6,4,6,3,7,6 segments

   segment c XX
   - exists in 0,1,2,3,4,7,8,9 -> 8 digits
   -    having 6,1,5,5,4,3,7,6 segments

   segment d XX
   - exists in 2,3,4,5,6,8,9 -> 7 digits
   -    having 5,5,4,5,6,7,6 segments

   segment e XX
   - exists in 0,2,6,8 -> 4 digits
   -    having 6,5,6,7 segments

   segment f XX
   - exists in 0,1,3,4,5,6,7,8,9 -> 9 digits
   -    having 6,1,5,4,5,6,3,7,6 segments

   segment g
   - exists in 0,2,3,5,6,8,9 -> 7 digits
   -    having 6,5,5,5,6,7,6 segments
 */

var sum = 0;

foreach(var entry in File.ReadLines(input)){
  var parts = entry.Split('|');
  var config = parts[0];
  var output = parts[1];

  var configSegments = config.Split(' ', StringSplitOptions.RemoveEmptyEntries)
    .Select(c => string.Join("", c.ToCharArray().OrderBy(v => v))).ToArray();
  var outputSegments = output.Split(' ', StringSplitOptions.RemoveEmptyEntries)
    .Select(c => string.Join("", c.ToCharArray().OrderBy(v => v))).ToArray();

  var d = configSegments.Union(outputSegments).Distinct().ToArray();
  var hist = d.SelectMany(c => c.ToCharArray()).GroupBy(c => c).ToDictionary(g => g.Key, g => g.Count());

  Console.WriteLine($"-------- {d.Length} combinations\n{entry}");

  var output_B = '?';
  var output_E = '?';
  var output_F = '?';

  foreach(var e in hist){
    if(e.Value == 6)
      output_B = e.Key;
    if(e.Value == 4)
      output_E = e.Key;
    if(e.Value == 9)
      output_F = e.Key;
  }

  var oneCombo = d.First(c => c.Length == 2);
  var output_C = oneCombo.ToCharArray().First(c => c != output_F);

  var output_A = hist.First(e => e.Key != output_C && e.Value == 8).Key;

  var fourCombo = d.First(c => c.Length == 4);
  var output_D = fourCombo.ToCharArray().First(c =>
      c != output_B && c != output_C && c != output_F);

  var output_G = hist.First(e => e.Key != output_D && e.Value == 7).Key;

  Console.WriteLine($"A == {output_A}\nB == {output_B}\nC == {output_C}\n" +
      $"D == {output_D}\nE == {output_E}\nF == {output_F}\nG == {output_F}");

  var translator = new Dictionary<char, char>();
  translator[output_A] = 'a';
  translator[output_B] = 'b';
  translator[output_C] = 'c';
  translator[output_D] = 'd';
  translator[output_E] = 'e';
  translator[output_F] = 'f';
  translator[output_G] = 'g';


  var outputValue = 0;
  foreach(var digit in outputSegments){
    var translated = string.Join("", digit.ToCharArray().Select(c => translator[c]).OrderBy(c => c));

    outputValue = outputValue * 10 + Array.IndexOf(realSegments, translated);
    Console.WriteLine($"{digit} -> {translated} -> {Array.IndexOf(realSegments, translated)}");
  }
  sum += outputValue;
}

Console.WriteLine($"Sum = {sum}");

