using System.Diagnostics;

var initTs = Stopwatch.GetTimestamp();


var file = "D:\\Personal\\AdventOfCode\\DATASET\\four\\input.txt";
var lines = File.ReadAllLines(file);

var rows = lines.Length;
var columns = lines[0].Length;

int xmasOccurrencies = 0;

foreach (var (row, line) in lines.Index())
    foreach(var (column, c) in line.Index())
        if(c == 'X')
            xmasOccurrencies += FindOccurrenciesFromX(row, column, rows, columns, lines);

int xOccurrencies = 0;
foreach (var (row, line) in lines[1..^1].Index())
    foreach (var (column, c) in line[1..^1].Index())
        if (c == 'A')
            xOccurrencies += IsXMasMas(row+1, column+1, rows, columns, lines) ? 1 : 0;

Console.WriteLine($"XMAS occurrencies {xmasOccurrencies}, XXMAS {xOccurrencies} in: {Stopwatch.GetElapsedTime(initTs).TotalMilliseconds}ms");

int FindOccurrenciesFromX(int row, int column, int rows, int columns, string[] lines)
{
    int occurrencies = 0;
    bool canSearchTop = row >= 3;
    bool canSearcBottom = row < rows - 3;
    bool canSearchLeft = column >= 3;
    bool canSearchRight = column < columns - 3;
    // can search top:
    if (canSearchTop)
    {
        occurrencies += MatchXmas(row, column, 0, -1, lines) ? 1 : 0;

        if (canSearchRight)
            occurrencies += MatchXmas(row, column, 1, -1, lines) ? 1 : 0;

        if (canSearchLeft)
            occurrencies += MatchXmas(row, column, -1, -1, lines) ? 1 : 0;
    }

    // can search bottom:
    if(canSearcBottom)
    {
        occurrencies += MatchXmas(row, column, 0, 1, lines) ? 1 : 0;

        if (canSearchRight)
            occurrencies += MatchXmas(row, column, 1, 1, lines) ? 1 : 0;

        if (canSearchLeft)
            occurrencies += MatchXmas(row, column, -1, 1, lines) ? 1 : 0;
    }
    // can search left:
    if (canSearchLeft)
        occurrencies += MatchXmas(row, column, -1, 0, lines) ? 1 : 0;
    // can search right:
    if (canSearchRight)
        occurrencies += MatchXmas(row, column, 1, 0, lines) ? 1 : 0;

    return occurrencies;
}

bool MatchXmas(int xrow, int xcolumn, int dx, int dy, string[] lines)
{
    return lines[xrow + dy][xcolumn + dx] == 'M' &&
        lines[xrow + 2 * dy][xcolumn + 2 * dx] == 'A' &&
        lines[xrow + 3 * dy][xcolumn + 3 * dx] == 'S';
}

bool IsXMasMas(int arow, int acolumn, int rows, int columns, string[] lines)
{
    char topLeft = lines[arow - 1][acolumn - 1];
    char topRight = lines[arow - 1][acolumn + 1];
    char bottomLeft = lines[arow + 1][acolumn - 1];
    char bottomRight = lines[arow + 1][acolumn + 1];

    return ((topLeft == 'M' && bottomRight == 'S') || (topLeft == 'S' && bottomRight == 'M')) &&
        ((topRight == 'M' && bottomLeft == 'S') || (topRight == 'S' && bottomLeft == 'M'));
}
