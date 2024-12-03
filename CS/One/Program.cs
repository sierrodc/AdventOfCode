using var file = File.OpenText("lists.txt");

//read all lines of the file
string? line;
var firstList = new List<int>();
var secondList = new List<int>();
while((line = await file.ReadLineAsync()) is not null)
{
    var x = line.AsSpan().Split("   ");
    x.MoveNext();
    firstList.Add(int.Parse(line[x.Current]));
    x.MoveNext();
    secondList.Add(int.Parse(line[x.Current]));
}

firstList.Sort();
secondList.Sort();

var sum = firstList.Zip(secondList).Sum(x => Math.Abs(x.First - x.Second));
Console.WriteLine($"Items: {firstList.Count}, {secondList.Count}");
Console.WriteLine(sum);