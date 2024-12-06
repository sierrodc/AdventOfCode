using System.Text.RegularExpressions;

var initTs = System.Diagnostics.Stopwatch.GetTimestamp();


using var file = File.OpenText("D:\\Personal\\AdventOfCode\\DATASET\\three\\input.txt");

int total = 0;
int totalEnabled = 0;
//read all lines of the file
string code = await file.ReadToEndAsync();
var regex = Recognizer.Multiplication();
var codeSpan = code.AsSpan();

total = GetMultiplications(regex, codeSpan);


foreach (var enabledRange in codeSpan.Split("do()"))
{
    var enabledCode = codeSpan[enabledRange];
    var untillDisabled = enabledCode.Split("don't()");
    untillDisabled.MoveNext();
    var enabledCodeUntilDisabled = enabledCode[untillDisabled.Current];

    totalEnabled += GetMultiplications(regex, enabledCodeUntilDisabled);
}

// 161289189, 83595109
Console.WriteLine($"Total: {total}, enabled {totalEnabled} in {System.Diagnostics.Stopwatch.GetElapsedTime(initTs).TotalMilliseconds}ms");

static int GetMultiplications(Regex regex, ReadOnlySpan<char> codeSpan)
{
    int total = 0;
    foreach (var m in regex.EnumerateMatches(codeSpan))
    {
        var factorsSpan = codeSpan.Slice(m.Index + 4 /* mul( */, m.Length - 4 - 1 /* mul() */);
        var comaIndex = factorsSpan.IndexOf(',');

        var first = int.Parse(factorsSpan.Slice(0, comaIndex));
        var second = int.Parse(factorsSpan.Slice(comaIndex + 1));
                
        total += first * second;
    }

    return total;
}

public partial class Recognizer
{
    [GeneratedRegex(@"mul\(\d+,\d+\)")]
    public static partial Regex Multiplication();
}