package main_a

import (
	"fmt"
	"os"
	"strings"
	"time"
)

func main() {
	start := time.Now()
	inputByte, _ := os.ReadFile("input_list.txt")
	inputRaw := strings.Split(string(inputByte), "\n")
	rows := len(inputRaw)
	mainMatrix := make([][]string, len(inputRaw))
	for n := range inputRaw {
		matrixRow := strings.ReplaceAll(inputRaw[n], "\r", "")
		mainMatrix[n] = strings.Split(matrixRow, "")
	}
	columns := len(mainMatrix[0])

	total := 0
	for row := range rows {
		for column := range columns {
			if mainMatrix[row][column] == "X" {
				checkNorth(mainMatrix, row, column, &total)
				checkNorthEast(mainMatrix, row, column, &total)
				checkEast(mainMatrix, row, column, &total)
				checkSouthEast(mainMatrix, row, column, &total)
				checkSouth(mainMatrix, row, column, &total)
				checkSouthOvest(mainMatrix, row, column, &total)
				checkOvest(mainMatrix, row, column, &total)
				checkNorthOvest(mainMatrix, row, column, &total)
			}
		}
	}
	fmt.Println(total)
	elapsed := time.Since(start)
	fmt.Printf("Elapsed %s\n", elapsed)
}

func checkNorth(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		rowToCheck := row - n
		if rowToCheck < 0 {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][column] != "M" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][column] != "A" {
			return
		}
		if n == 3 && mainMatrix[rowToCheck][column] != "S" {
			return
		}
	}
	*total += 1
}

func checkNorthEast(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		rowToCheck := row - n
		colToCheck := column + n
		if rowToCheck < 0 || colToCheck >= len(mainMatrix[rowToCheck]) {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "M" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 3 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	*total += 1
}

func checkEast(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		colToCheck := column + n
		if colToCheck >= len(mainMatrix[row]) {
			return
		}
		if n == 1 && mainMatrix[row][colToCheck] != "M" {
			return
		}
		if n == 2 && mainMatrix[row][colToCheck] != "A" {
			return
		}
		if n == 3 && mainMatrix[row][colToCheck] != "S" {
			return
		}
	}
	*total += 1
}

func checkSouthEast(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		rowToCheck := row + n
		colToCheck := column + n
		if rowToCheck >= len(mainMatrix) || colToCheck >= len(mainMatrix[rowToCheck]) {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "M" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 3 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	*total += 1
}

func checkSouth(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		rowToCheck := row + n
		if rowToCheck >= len(mainMatrix) {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][column] != "M" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][column] != "A" {
			return
		}
		if n == 3 && mainMatrix[rowToCheck][column] != "S" {
			return
		}
	}
	*total += 1
}

func checkSouthOvest(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		rowToCheck := row + n
		colToCheck := column - n
		if rowToCheck >= len(mainMatrix) || colToCheck < 0 {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "M" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 3 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	*total += 1
}

func checkOvest(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		colToCheck := column - n
		if colToCheck < 0 {
			return
		}
		if n == 1 && mainMatrix[row][colToCheck] != "M" {
			return
		}
		if n == 2 && mainMatrix[row][colToCheck] != "A" {
			return
		}
		if n == 3 && mainMatrix[row][colToCheck] != "S" {
			return
		}
	}
	*total += 1
}

func checkNorthOvest(mainMatrix [][]string, row int, column int, total *int) {
	for n := 1; n <= 3; n++ {
		rowToCheck := row - n
		colToCheck := column - n
		if rowToCheck < 0 || colToCheck < 0 {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "M" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 3 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	*total += 1
}
