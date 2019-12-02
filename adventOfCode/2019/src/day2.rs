pub fn part1(input: Vec<String>) -> usize {
  let mut program = input[0]
    .split(",")
    .filter_map(|i| i.parse::<usize>().ok())
    .collect::<Vec<usize>>();

  program[1] = 12;
  program[2] = 2;

  run(&mut program)[0]
}

pub fn part2(input: Vec<String>) -> usize {
  let program = input[0]
    .split(",")
    .filter_map(|i| i.parse::<usize>().ok())
    .collect::<Vec<usize>>();

  let search = 19690720;

  for i in 0..100 {
    for j in 0..100 {
      let mut mod_input = program.clone().to_vec();
      mod_input[1] = i;
      mod_input[2] = j;
      if run(&mut mod_input)[0] == search {
        return i * 100 + j;
      }
    }
  }
  0
}

fn run(program: &mut Vec<usize>) -> &Vec<usize> {
  for i in (0..program.len()).step_by(4) {
    let result = program[i + 3];
    let first = program[i + 1];
    let second = program[i + 2];
    match &program[i] {
      1 => program[result] = program[first] + program[second],
      2 => program[result] = program[first] * program[second],
      99 => break,
      _ => panic!(""),
    }
  }
  program
}
