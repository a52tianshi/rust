package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("Hello, World!")

}

// find closest num in two arrays
func findClosestNum(arr1 []int, arr2 []int) int {
	closestDiff := math.MaxInt32
	for _, num := range arr1 {
		for _, num2 := range arr2 {
			diff := num - num2
			if diff < closestDiff {
				closestDiff = diff
			}
		}
	}
	return closestDiff
}
