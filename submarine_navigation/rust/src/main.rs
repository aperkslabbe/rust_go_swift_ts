fn get_input() -> &'static str {
  return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

struct Coordinates {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Coordinates {
    let (direction, amount) = line.split_once(" ").expect("Invalid input");
    println!("Direction: {}, Amount: {}", direction, amount);
    let amount = str::parse::<i32>(amount).expect("Invalid number");
    match direction {
        "forward" => Coordinates { x: amount, y: 0 },
        "up" => Coordinates { x: 0, y: amount },
        "down" => Coordinates { x: 0, y: -amount },
        _ => panic!("Invalid input"),
    }
}

fn main() {
    let input = get_input();
    let mut coordinates = Coordinates { x: 0, y: 0 };
    for line in input.lines() {
        let parsed = parse_line(line);
        coordinates.x += parsed.x;
        coordinates.y += parsed.y;
    }
    println!("Final coordinates: ({}, {})", coordinates.x, coordinates.y);
}
