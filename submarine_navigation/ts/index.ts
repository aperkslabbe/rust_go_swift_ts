function getInput(): String {
  return `forward 5
down 5 
forward 8
up 3
down 8
forward 2`;
}

function parseLine(input: string): [number, number] {
  const [dir, amount] = input.split(' ');
  const amountInt = parseInt(amount);
  if (dir === 'forward') {
    return [amountInt, 0];
  } else if (dir === 'up') {
    return [0, -amountInt];
  }
  return [0, amountInt]
}

const out = getInput()
  .split('\n')
  .map(x => parseLine(x))
  .reduce((acc, [x, y]) => [acc[0] + x, acc[1] + y], [0, 0]);

console.log(out)