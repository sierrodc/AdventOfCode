package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
	"time"
)

type Guardian struct {
	currentCol       int
	currentRow       int
	currentDirection string
	obstacleCreated  []string
	isComplete       bool
	obstaclesVisited map[string]int
}

type Tuple struct {
	row int
	col int
}

func main() {
	start := time.Now()
	inputByte, _ := os.ReadFile("input_list.txt")
	inputRaw := strings.Split(string(inputByte), "\r\n")
	matrix := make([][]string, len(inputRaw))
	for index, row := range inputRaw {
		parsedRow := strings.Split(row, "")
		matrix[index] = parsedRow
	}
	row, col := getGuardianStartingPosition(matrix)
	guardian := Guardian{isComplete: false, currentDirection: "N", currentCol: col, currentRow: row, obstaclesVisited: make(map[string]int)}
	for !guardian.isComplete {
		newRow, newCol := takeStep(guardian.currentRow, guardian.currentCol, guardian.currentDirection, 1)
		if newRow < 0 || newCol < 0 || newRow >= len(matrix) || newCol >= len(matrix[newRow]) {
			guardian.isComplete = true
		} else {
			if matrix[newRow][newCol] != "#" {
				if matrix[newRow][newCol] != "$" {
					matrix[newRow][newCol] = "#"
					direction := guardian.currentDirection
					if isTheObstacleCreatingALoop(&guardian, matrix) {
						addToObstacleCreated(&guardian, Tuple{row: newRow, col: newCol})
					}
					guardian.currentDirection = direction
					//matrix[newRow][newCol] = "."
				}
				guardian.currentRow = newRow
				guardian.currentCol = newCol
				guardian.obstaclesVisited = make(map[string]int)
				guardian.isComplete = false
				matrix[newRow][newCol] = "$"
			} else {
				guardian.currentDirection = rotateDirection(guardian.currentDirection)
			}
		}
	}

	fmt.Println("obs created = ", len(guardian.obstacleCreated))
	elapsed := time.Since(start)
	fmt.Printf("Elapsed %s\n", elapsed)
}

func addToObstacleCreated(guardian *Guardian, obstacleCoords Tuple) {
	coordId := strconv.Itoa(obstacleCoords.row)
	coordId += "_" + strconv.Itoa(obstacleCoords.col)
	if !slices.Contains(guardian.obstacleCreated, coordId) {
		guardian.obstacleCreated = append(guardian.obstacleCreated, coordId)
	}
}

func takeStep(currentRow int, currentCol int, direction string, stepToTake int) (int, int) {
	newRow := currentRow
	newCol := currentCol
	switch direction {
	case "N":
		newRow -= stepToTake
	case "E":
		newCol += stepToTake
	case "S":
		newRow += stepToTake
	case "W":
		newCol -= stepToTake
	}
	return newRow, newCol
}

func isEndOfMatrixReached(mainMatrix [][]string, newRow int, newCol int) bool {
	return newRow < 0 || newCol < 0 || newRow >= len(mainMatrix) || newCol >= len(mainMatrix[newRow])
}

func isTheObstacleCreatingALoop(guardian *Guardian, mainMatrix [][]string) bool {
	for !guardian.isComplete {
		newRow, newCol := takeStep(guardian.currentRow, guardian.currentCol, guardian.currentDirection, 1)
		if isEndOfMatrixReached(mainMatrix, newRow, newCol) {
			return false
		}
		if isSamePositionPreviouslyReached(guardian, newRow, newCol) {
			return true
		} else {
			if mainMatrix[newRow][newCol] == "#" {
				addToObstacleVisited(guardian, newRow, newCol)
				guardian.currentDirection = rotateDirection(guardian.currentDirection)
			} else {
				guardian.currentCol = newCol
				guardian.currentRow = newRow
			}
		}
	}
	return false
}

func rotateDirection(currentDirection string) string {
	switch currentDirection {
	case "N":
		return "E"
	case "E":
		return "S"
	case "S":
		return "W"
	case "W":
		return "N"
	default:
		return currentDirection
	}
}

func addToObstacleVisited(guardian *Guardian, row int, col int) {
	coordId := strconv.Itoa(row)
	coordId += "_" + strconv.Itoa(col) + "_" + guardian.currentDirection
	_, ok := guardian.obstaclesVisited[coordId]
	if ok {
		guardian.obstaclesVisited[coordId] += 1
	} else {
		guardian.obstaclesVisited[coordId] = 0
	}
}

func isSamePositionPreviouslyReached(guardian *Guardian, row int, col int) bool {
	coordId := strconv.Itoa(row)
	coordId += "_" + strconv.Itoa(col) + "_" + guardian.currentDirection
	return guardian.obstaclesVisited[coordId] > 3
}

func getGuardianStartingPosition(matrix [][]string) (int, int) {
	for row := range matrix {
		col := slices.Index(matrix[row], "^")
		if col != -1 {
			return row, col
		}
	}
	panic("No sarting position found")
}
