package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func main() {
	start := time.Now()
	inputByte, err := os.ReadFile("input_list.txt")
	inputRaw := string(inputByte)
	inputListRaw := strings.Split(inputRaw, "\n")
	if err != nil {
	}

	var inputList1 []int
	var inputList2 []int

	for n := range inputListRaw {
		numbers := strings.Split(inputListRaw[n], "   ")
		num_list_1, err1 := strconv.Atoi(numbers[0])
		if err1 != nil {
			log.Fatal("Cannot parse string to num", numbers[0])
		}
		num_list_2, err2 := strconv.Atoi(strings.TrimSuffix(numbers[1], "\r"))
		if err2 != nil {
			log.Fatal("Cannot parse string to num", numbers[1])
		}
		inputList1 = append(inputList1, num_list_1)
		inputList2 = append(inputList2, num_list_2)
	}

	sort.Ints(inputList1)
	sort.Ints(inputList2)
	// First Part
	/*	var totalDistance = 0
		for n := range inputList1 {
			totalDistance += int(math.Abs(float64(inputList1[n] - inputList2[n])))
		}*/
	// Second Part
	var totalSimilarity = 0
	for n := range inputList1 {
		totalSimilarity += inputList1[n] * estimateSimilarity(inputList1[n], inputList2)
	}

	fmt.Println(totalSimilarity)
	elapsed := time.Since(start)

	fmt.Printf("Elapsed %s\n", elapsed)
}

func estimateSimilarity(num int, list []int) int {
	total := 0
	for n := range list {
		if list[n] == num {
			total += 1
		}
	}
	return total
}
