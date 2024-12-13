package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	start := time.Now()
	inputByte, _ := os.ReadFile("input_list.txt")
	inputRow := strings.Split(string(inputByte), "\r\n")
	var results []int
	numbers := make([][]int, len(inputRow))
	for index, row := range inputRow {
		parsedRow := strings.Split(row, ":")
		res, _ := strconv.Atoi(parsedRow[0])
		results = append(results, res)
		for _, el := range strings.Split(parsedRow[1][1:], " ") {
			parsedEl, _ := strconv.Atoi(el)
			numbers[index] = append(numbers[index], parsedEl)
		}
	}

	finalResult := 0
	for index, result := range results {
		partialResult := getNumberOfSameResult(result, numbers[index])
		finalResult += partialResult
	}

	elapsed := time.Since(start)
	fmt.Println("Result", finalResult)
	fmt.Printf("Elapsed %s\n", elapsed)
}

func getNumberOfSameResult(result int, numbers []int) int {
	counter := 0
	for float64(counter) < math.Pow(3, float64(len(numbers)-1)) {
		binary := getBinary(counter, len(numbers)-1)
		partialResult := getPartialResult(binary, numbers)
		if partialResult == result {
			return result
		}
		counter++
	}
	return 0
}

func getPartialResult(binary string, numbers []int) int {
	total := numbers[0]
	charIndex := 0
	for charIndex < len(binary) {
		if string(binary[charIndex]) == "2" {
			a := strconv.Itoa(total)
			b := strconv.Itoa(numbers[charIndex+1])
			newTotal, _ := strconv.Atoi(a + b)
			total = newTotal
		} else if string(binary[charIndex]) == "0" {
			total += numbers[charIndex+1]
		} else if string(binary[charIndex]) == "1" {
			total *= numbers[charIndex+1]
		}
		charIndex++
	}
	return total
}

func getBinary(num int, length int) string {
	converted := strconv.FormatInt(int64(num), 3)
	if len(converted) < length {
		for range length - len(converted) {
			converted = "0" + converted
		}
	}
	return converted
}
