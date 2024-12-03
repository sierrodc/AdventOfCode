using var file = File.OpenText("X:\\Personal\\AdventOfCode\\DATASET\\one\\input.txt");

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
Console.WriteLine($"Items: {firstList.Count}, {secondList.Count}: Total distance is {sum}");

// lists are sorted => this code can be optimized
var firstListOccurrence = firstList.CountBy(x => x).ToDictionary(x => x.Key, x => x.Value);
var secondListOccurrence = secondList.CountBy(x => x).ToDictionary(x => x.Key, x => x.Value);

var secondSum = firstListOccurrence.Sum(i => i.Key /*the number in first list*/ * i.Value /*how many times in first list*/ * secondListOccurrence.GetValueOrDefault(i.Key, 0) /*how many times in second list*/);
// secondSum += secondListOccurrence.Sum(i => i.Key /*the number in second list*/ * i.Value /*how many times in second list*/ * firstListOccurrence.GetValueOrDefault(i.Key, 0) /*how many times in second list*/);

Console.WriteLine($"Second total distance is {secondSum}");