mod tools;

fn main() {
     let g = tools::read_file_to_lines("aoc202101.txt");
     // part1
     let z :i32 = g.iter()
     .zip(g.iter().skip(1))
     .map(|(current, next)| if current.parse::<i32>().unwrap() < next.parse::<i32>().unwrap() {1} else {0}).sum();
     
    

     println!("AOC2021 01a: {}", z);

     //part2
     let z1 : Vec<i32>= g.windows(3).map(|n| n[0].parse::<i32>().unwrap() + n[1].parse::<i32>().unwrap() + n[2].parse::<i32>().unwrap()).collect();

     let z3 :i32 = z1.iter()
     .zip(z1.iter().skip(1))
     .map(|(current, next)| if current < next {1} else {0}).sum();
     
     println!("AOC2021 01b: {}", z3);
}