package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

func main() {
	start := time.Now()
	inputByte, _ := os.ReadFile("input_list.txt")
	inputRaw := string(inputByte)
	// Remove every string inside a don't() * do() substring, or don't() and the end of the string.
	subStringToBeRemovedRegex := regexp.MustCompile("don't\\(\\)(\\s|\\S)*?(do\\(\\)|\\z)")
	subStringToBeRemoved := subStringToBeRemovedRegex.FindAllString(inputRaw, -1)
	for n := range subStringToBeRemoved {
		inputRaw = strings.ReplaceAll(inputRaw, subStringToBeRemoved[n], "")
	}
	fmt.Println(estimateMulFromString(inputRaw))
	elapsed := time.Since(start)
	fmt.Printf("Elapsed %s\n", elapsed)
}

func estimateMulFromString(inputRaw string) int {
	mulRegex, _ := regexp.Compile("mul\\(\\d{1,3},\\d{1,3}\\)")
	digitRegex, _ := regexp.Compile("\\d{1,3},\\d{1,3}")
	mulRegexMatches := mulRegex.FindAllString(inputRaw, -1)
	total := 0
	for n := range mulRegexMatches {
		match := digitRegex.FindAllString(mulRegexMatches[n], -1)
		res := strings.Split(match[0], ",")
		final1, _ := strconv.Atoi(res[0])
		final2, _ := strconv.Atoi(res[1])
		total += final1 * final2
	}
	return total
}
