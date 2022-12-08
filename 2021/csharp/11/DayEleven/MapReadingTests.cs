using BenchmarkDotNet.Attributes;

namespace DayEleven;

[MemoryDiagnoser, EventPipeProfiler(BenchmarkDotNet.Diagnosers.EventPipeProfile.CpuSampling)]
public class MapReadingTests{

  [Params(9, 90)]
  public int height;

  [Params(10, 100)]
  public int width;

  private byte[] map;

  [GlobalSetup]
  public void Setup(){
    map = new byte[height*width];
    new Random(42).NextBytes(map);
  }

  private int idx(int x, int y) => y*width+x;

  [Benchmark]
  public void Iterate(){
    byte v = 0;
    for(var y = 0; y < height; y++){
      for(var x = 0; x < width; x++){
        v = map[idx(x,y)];
      }
    }
  }

  [Benchmark]
  public void IterateSpan(){
    byte v = 0;
    for(var y = 0; y < height; y++){
      var line = new Span<byte>(map, idx(0, y), width);
      for(var x = 0; x < width; x++){
        v = line[x];
      }
    }
  }
}
