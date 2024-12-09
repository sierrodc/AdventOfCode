package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"sort"
	"strconv"
	"strings"
	"time"
)

func main() {
	start := time.Now()
	inputByte, _ := os.ReadFile("input_list.txt")
	inputRaw := strings.Split(string(inputByte), "\r\n\r\n")
	rules := parseRules(inputRaw)
	pages := parsePages(inputRaw)
	validPages := filterValidPages(pages, rules)
	total := sumMiddleValues(validPages)
	fmt.Println(total)
	elapsed := time.Since(start)
	fmt.Printf("Elapsed %s\n", elapsed)
}

func parseRules(inputRaw []string) [][]int {
	rulesRaw := strings.Split(inputRaw[0], "\r\n")
	rules := make([][]int, len(rulesRaw))
	for n := range rulesRaw {
		numbers := strings.Split(rulesRaw[n], "|")
		prev, _ := strconv.Atoi(numbers[0])
		next, _ := strconv.Atoi(numbers[1])
		rules[n] = []int{prev, next}
	}
	return rules
}

func parsePages(inputRaw []string) [][]int {
	pagesRaw := strings.Split(inputRaw[1], "\r\n")
	pages := make([][]int, len(pagesRaw))
	for n := range pagesRaw {
		numbers := strings.Split(pagesRaw[n], ",")
		page := make([]int, len(numbers))
		for num := range numbers {
			parsedNum, _ := strconv.Atoi(numbers[num])
			page[num] = parsedNum
		}
		pages[n] = page
	}
	return pages
}

func isRuleRespected(page []int, rule []int) bool {
	prev := rule[0]
	next := rule[1]
	if !slices.Contains(page, prev) || !slices.Contains(page, next) {
		return true
	}
	if slices.Index(page, prev) > slices.Index(page, next) {
		return false
	}
	return true
}

func isPageSequenceValid(page []int, rules [][]int) bool {
	for _, rule := range rules {
		if !isRuleRespected(page, rule) {
			return false
		}
	}
	return true
}

func fixPage(pages []int, weightedRules map[int][]int) []int {
	weightedPages := make(map[int]int)
	for _, page := range pages {
		weight := 0
		for _, internalPage := range pages {
			if slices.Contains(weightedRules[page], internalPage) {
				weight++
			}
		}
		weightedPages[page] = weight
	}

	orderedPages := make([]int, 0, len(weightedPages))
	for k := range weightedPages {
		orderedPages = append(orderedPages, k)
	}

	sort.Slice(orderedPages, func(i, j int) bool {
		return weightedPages[orderedPages[i]] < weightedPages[orderedPages[j]]
	})

	return orderedPages
}

func computeWeightedRules(rules [][]int) map[int][]int {
	weightedRules := make(map[int][]int)
	for _, rule := range rules {
		weightedRules[rule[0]] = append(weightedRules[rule[0]], rule[1])
	}
	return weightedRules
}

func filterValidPages(pages [][]int, rules [][]int) [][]int {
	weightedRules := computeWeightedRules(rules)
	var validPages [][]int
	for _, page := range pages {
		if !isPageSequenceValid(page, rules) {
			fixedPage := fixPage(page, weightedRules)
			validPages = append(validPages, fixedPage)
		}
	}
	return validPages
}

func sumMiddleValues(validPages [][]int) int {
	total := 0
	for _, validPage := range validPages {
		total += validPage[int(math.Floor(float64(len(validPage)/2)))]
	}
	return total
}
