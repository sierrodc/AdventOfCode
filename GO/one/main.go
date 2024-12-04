package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func countOccurrencies(list []int) map[int]int {
	occurrencies := make(map[int]int)

	for _, val := range list {
		occurrencies[val]++
	}

	return occurrencies
}

func main() {
	start := time.Now()

	file, err := os.Open("X:\\Personal\\AdventOfCode\\DATASET\\one\\input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var inputList1 []int
	var inputList2 []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		tokens := strings.Split(line, "   ")
		if num, err := strconv.Atoi(tokens[0]); err != nil {
			log.Fatal(err)
		} else {
			inputList1 = append(inputList1, num)
		}
		if num, err := strconv.Atoi(tokens[1]); err != nil {
			log.Fatal(err)
		} else {
			inputList2 = append(inputList2, num)
		}
	}

	sort.Ints(inputList1)
	sort.Ints(inputList2)

	var firstTotalDistance = 0
	for n := range inputList1 {
		firstTotalDistance += int(math.Abs(float64(inputList1[n] - inputList2[n])))
	}

	firstListOccurrence := countOccurrencies(inputList1)
	secondListOccurrence := countOccurrencies(inputList2)

	var secondTotalDistance = 0
	for id, occurrencies := range firstListOccurrence {
		secondTotalDistance += id * occurrencies * secondListOccurrence[id]
	}

	fmt.Println(firstTotalDistance)
	elapsed := time.Since(start)

	fmt.Printf("Distancies: d1:%d d2: %d in %s\n", firstTotalDistance, secondTotalDistance, elapsed)
}
