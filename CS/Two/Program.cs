var initTs = System.Diagnostics.Stopwatch.GetTimestamp();

static bool IsReportCorrect(List<byte> report, int? levelIdxToSkip)
{
    byte? lastLevel = null;
    char direction = '='; // no direction
    bool isCorrect = true;

    int currentLevelIndex = -1;
    while (currentLevelIndex + 1 < report.Count)
    {
        currentLevelIndex++;
        // level to skip based on Problem Dampener
        if (levelIdxToSkip.HasValue && currentLevelIndex.Equals(levelIdxToSkip.Value))
            continue;

        var newLevel = report[currentLevelIndex];
        if (lastLevel is null) // first number read
        {
            lastLevel = newLevel;
            continue;
        }

        var deltaLevel = newLevel - lastLevel;
        if (deltaLevel > 3 || deltaLevel < -3 || deltaLevel == 0)
        {
            //Problem Dampener
            if (!levelIdxToSkip.HasValue)
                isCorrect = IsReportCorrect(report, 0) || IsReportCorrect(report, currentLevelIndex - 1) || IsReportCorrect(report, currentLevelIndex);
            else
                isCorrect = false;

            break;
        }

        var currentDirection = newLevel > lastLevel ? '>' : '<';

        if (direction == '=') // second number read. Set direction
        {
            direction = currentDirection;
            lastLevel = newLevel;
            continue;
        }

        if (direction != currentDirection)
        {
            if (!levelIdxToSkip.HasValue)
                isCorrect = IsReportCorrect(report, 0) || IsReportCorrect(report, currentLevelIndex - 1) || IsReportCorrect(report, currentLevelIndex);
            else
                isCorrect = false;
            break;
        }

        lastLevel = newLevel;
    }

    return isCorrect;
}

using var file = File.OpenText("X:\\Personal\\AdventOfCode\\DATASET\\two\\input.txt");

//read all lines of the file
string? report;
int correctReports = 0;
int correctReportsConsideringDampener = 0;
int reportIndex = 0;
List<byte> reportLevels = new List<byte>(10);

while ((report = await file.ReadLineAsync()) is not null)
{
    reportLevels.Clear();
    var reportSpan = report.AsSpan();
    foreach (var level in reportSpan.Split(' '))
        reportLevels.Add(byte.Parse(reportSpan[level]));

    reportIndex++;
    if (IsReportCorrect(reportLevels, -1))
        correctReports++;
    if (IsReportCorrect(reportLevels, null))
        correctReportsConsideringDampener++;
}

Console.WriteLine($"Correct reports: {correctReports}, {correctReportsConsideringDampener} considering Dampener in {System.Diagnostics.Stopwatch.GetElapsedTime(initTs).TotalMilliseconds}ms");