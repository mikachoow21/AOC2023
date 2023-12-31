
use std::collections::VecDeque;

use crate::utils::read_lines;

pub fn main(filename: &str) {
  let res: i64 = read_lines(filename).into_iter().map(parse_line).sum();

  println!("{}", res); // 1904165718
}

fn parse_line(line: String) -> i64 {
  let mut res: i64 = 0;
  let mut nums: VecDeque<i64> = line.split(' ')
                                    .map(|n| n.parse::<i64>().unwrap())
                                    .collect();

  while !nums.iter().all(|&n| n == 0) {
    let last = nums.pop_back().unwrap();
    nums = nums.into_iter()
               .rev()
               .fold(
                (VecDeque::<i64>::new(), last),
                  |(mut vec, acc) , x| {
                  vec.push_front(acc - x);
                  (vec, x)
                }
              )
              .0;
    res += last;
  }
  res
}