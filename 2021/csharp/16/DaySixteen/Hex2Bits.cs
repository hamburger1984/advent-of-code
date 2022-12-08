namespace DaySixteen;

internal class Hex2Bits{
  private readonly string _hexString;
  private readonly byte[] _bytes;
  private int _byte = 0;
  private int _bit = 0;

  public Hex2Bits(string hexString){
    _hexString = hexString;
    _bytes = Convert.FromHexString(hexString);
  }

  public bool Peek(){
    return BitAtIndex();
  }

  public bool Next(){
    var result = BitAtIndex();
    _bit++;
    if(_bit == 8){
      _byte++;
      _bit = 0;
    }
    return result;
  }

  public void Align() {
    if(_bit > 4){
      _bit = 0;
      _byte++;
    }
    if(_bit > 0){
      _bit = 4;
    }
  }

  public bool EOF() => _byte >= _bytes.Length;

  private bool BitAtIndex(){
    if(_byte < 0 || _byte >= _bytes.Length)
      throw new Exception($"Failed to read after end of bytes @{Position}");

    var b = _bytes[_byte];

    var mask = 0b10000000 >> _bit;

    return (b & mask) > 0;
  }

  public (int Byte, int Bit) Position => (Byte:_byte, Bit:_bit);
}
