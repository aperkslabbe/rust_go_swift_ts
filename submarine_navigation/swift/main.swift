func getInput() -> String {
  return """
forward 5
down 5
forward 8
up 3
down 8
forward 2
""";
}

func parseLine(_ line: String) -> (String, Int) {
  let parts = line.split(separator: " ")
  return (String(parts[0]), Int(parts[1])!)
}

func navigateSubmarine(_ input: String) -> (Int, Int) {
  let lines = input.split(separator: "\n")
  var x = 0
  var y = 0
  for line in lines {
    let (direction, distance) = parseLine(String(line))
    switch direction {
    case "forward":
      x += distance
    case "back":
      x -= distance
    case "up":
      y += distance
    case "down":
      y -= distance
    default:
      break
    }
  }
   return (x, y)
}

print(navigateSubmarine(getInput()))

