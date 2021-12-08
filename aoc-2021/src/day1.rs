const INPUT1: &'static str = include_str!("../inputs/day1.txt");

pub(crate) fn run() {
  let lines: Vec<u16> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
  let count = lines.array_windows().filter_map(|[n1,n2]| (n2 > n1).then(|| ())).count();
  println!("number of bigger than: {}", count);
}