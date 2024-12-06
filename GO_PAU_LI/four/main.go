package main

import (
	"fmt"
	"os"
	"strconv"
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

	aAlreadyFound := make(map[string]int)
	total := 0
	for row := range rows {
		for column := range columns {
			if mainMatrix[row][column] == "M" {
				checkNorthEast(mainMatrix, row, column, aAlreadyFound, &total)
				checkSouthEast(mainMatrix, row, column, aAlreadyFound, &total)
				checkSouthOvest(mainMatrix, row, column, aAlreadyFound, &total)
				checkNorthOvest(mainMatrix, row, column, aAlreadyFound, &total)
			}
		}
	}
	fmt.Println(total)
	elapsed := time.Since(start)
	fmt.Printf("Elapsed %s\n", elapsed)
}

func checkNorthEast(mainMatrix [][]string, row int, column int, aAlreadyFound map[string]int, total *int) {
	for n := 1; n <= 2; n++ {
		rowToCheck := row - n
		colToCheck := column + n
		if rowToCheck < 0 || colToCheck >= len(mainMatrix[rowToCheck]) {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	_, ok := aAlreadyFound[strconv.Itoa(row-1)+","+strconv.Itoa(column+1)]
	if ok {
		*total += 1
	} else {
		aAlreadyFound[strconv.Itoa(row-1)+","+strconv.Itoa(column+1)] = 1
	}

}

func checkSouthEast(mainMatrix [][]string, row int, column int, aAlreadyFound map[string]int, total *int) {
	for n := 1; n <= 2; n++ {
		rowToCheck := row + n
		colToCheck := column + n
		if rowToCheck >= len(mainMatrix) || colToCheck >= len(mainMatrix[rowToCheck]) {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	_, ok := aAlreadyFound[strconv.Itoa(row+1)+","+strconv.Itoa(column+1)]
	if ok {
		*total += 1
	} else {
		aAlreadyFound[strconv.Itoa(row+1)+","+strconv.Itoa(column+1)] = 1
	}
}

func checkSouthOvest(mainMatrix [][]string, row int, column int, aAlreadyFound map[string]int, total *int) {
	for n := 1; n <= 2; n++ {
		rowToCheck := row + n
		colToCheck := column - n
		if rowToCheck >= len(mainMatrix) || colToCheck < 0 {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	_, ok := aAlreadyFound[strconv.Itoa(row+1)+","+strconv.Itoa(column-1)]
	if ok {
		*total += 1
	} else {
		aAlreadyFound[strconv.Itoa(row+1)+","+strconv.Itoa(column-1)] = 1
	}
}

func checkNorthOvest(mainMatrix [][]string, row int, column int, aAlreadyFound map[string]int, total *int) {
	for n := 1; n <= 2; n++ {
		rowToCheck := row - n
		colToCheck := column - n
		if rowToCheck < 0 || colToCheck < 0 {
			return
		}
		if n == 1 && mainMatrix[rowToCheck][colToCheck] != "A" {
			return
		}
		if n == 2 && mainMatrix[rowToCheck][colToCheck] != "S" {
			return
		}
	}
	_, ok := aAlreadyFound[strconv.Itoa(row-1)+","+strconv.Itoa(column-1)]
	if ok {
		*total += 1
	} else {
		aAlreadyFound[strconv.Itoa(row-1)+","+strconv.Itoa(column-1)] = 1
	}
}
