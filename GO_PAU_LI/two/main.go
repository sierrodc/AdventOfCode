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
	inputByte, err := os.ReadFile("input_list.txt")
	inputReportString := strings.Split(string(inputByte), "\r")
	if err != nil {
	}

	var inputList [][]int

	for n := range inputReportString {
		report := strings.Split(inputReportString[n], " ")
		parsedReport := make([]int, len(report))
		for r := range report {
			num, _ := strconv.Atoi(strings.TrimPrefix(report[r], "\n"))
			parsedReport[r] = num
		}
		inputList = append(inputList, parsedReport)
	}

	var totalSafe = 0
	for n := range inputList {
		if isReportSafe(inputList[n]) {
			totalSafe += 1
		} else {
			if isReportSafeRemovingOneElement(inputList[n]) {
				totalSafe += 1
			}
		}
	}

	fmt.Println(totalSafe)
	elapsed := time.Since(start)

	fmt.Printf("Elapsed %s\n", elapsed)
}

func isReportSafe(list []int) bool {
	isAscending := list[0]-list[1] < 0

	for n := range list {
		if n == len(list)-1 {
			return true
		}
		lead := list[n] - list[n+1]
		if lead > 3 || lead < -3 || lead == 0 {
			return false
		}
		if lead < 0 && !isAscending {
			return false
		}
		if lead > 0 && isAscending {
			return false
		}

	}
	return true
}

func isReportSafeRemovingOneElement(list []int) bool {
	for n := range list {
		newArr := make([]int, 0, len(list)-1)
		newArr = append(newArr, list[:n]...)
		newArr = append(newArr, list[n+1:]...)
		if isReportSafe(newArr) {
			return true
		}
	}
	return false
}
