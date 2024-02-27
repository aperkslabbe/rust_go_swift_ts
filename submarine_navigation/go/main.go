package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

func getInput() string {
  return `forward 5
down 5 
forward 8
up 3
down 8
forward 2`;
}

type Coordinates struct { 
	x int
	y int
}

func parseLine(line string) Coordinates {
	args := strings.Split(line, " ")
	amount, err := strconv.Atoi(args[1])
	direction := args[0]

	if err != nil {
		log.Fatal("Error parsing line")
	}

	if direction == "forward" {
		return Coordinates{x: amount, y: 0}
	}

	if direction == "down" {
		return Coordinates{x: 0, y: amount}
	}

	return Coordinates{x: 0, y: -amount}
}


func main() {
	lines := strings.Split(getInput(), "\n")
	position := Coordinates{x: 0, y: 0}
	for _, line := range lines {

		// split the line into words
		coordinates := parseLine(line)
		position.x += coordinates.x
		position.y += coordinates.y
	}
	fmt.Println("X: ", position.x, "Y: ", position.y)
}