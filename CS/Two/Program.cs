using System.Diagnostics;

var initTs = System.Diagnostics.Stopwatch.GetTimestamp();

static bool IsReportCorrect(ReadOnlySpan<char> report, Range? levelToSkip)
{
    if (report.StartsWith("16 14 12 8 6 8"))
        Console.WriteLine("ah");

    var items = report.Split(' ');
    byte? lastLevel = null;
    Range lastLevelRange = 0..0;
    char direction = '='; // no direction
    bool isCorrect = true;
    while (items.MoveNext())
    {
        // level to skip based on Problem Dampener
        if (levelToSkip.HasValue && items.Current.Equals(levelToSkip.Value))
            continue;

        var newLevel = byte.Parse(report[items.Current]);
        if (lastLevel is null) // first number read
        {
            lastLevel = newLevel;
            lastLevelRange = items.Current;
            continue;
        }

        var deltaLevel = newLevel - lastLevel;
        if (deltaLevel > 3 || deltaLevel < -3 || deltaLevel == 0)
        {
            //Problem Dampener
            if(!levelToSkip.HasValue)
                isCorrect = IsReportCorrect(report, lastLevelRange) || IsReportCorrect(report, items.Current);
            else
                isCorrect = false;

            break;
        }

        var currentDirection = newLevel > lastLevel ? '>' : '<';

        if (direction == '=') // second number read. Set direction
        {
            direction = currentDirection;
            lastLevel = newLevel;
            lastLevelRange = items.Current;
            continue;
        }

        if (direction != currentDirection)
        {
            if (!levelToSkip.HasValue)
                isCorrect = IsReportCorrect(report, lastLevelRange) || IsReportCorrect(report, items.Current);
            else
                isCorrect = false;
            break;
        }

        lastLevel = newLevel;
        lastLevelRange = items.Current;
    }

    return isCorrect;
}

using var file = File.OpenText("X:\\Personal\\AdventOfCode\\DATASET\\two\\input.txt");

//read all lines of the file
string? report;
int correctReports = 0;
int correctReportsConsideringDampener = 0;
while ((report = await file.ReadLineAsync()) is not null)
{
    if (IsReportCorrect(report, 0..0))
        correctReports++;
    if (IsReportCorrect(report, null))
        correctReportsConsideringDampener++;
    else
        Console.WriteLine($"Wrong: {report}");
}

Console.WriteLine($"Correct reports: {correctReports}, {correctReportsConsideringDampener} considering Dampener in {System.Diagnostics.Stopwatch.GetElapsedTime(initTs).TotalMilliseconds}ms");