use itertools::Itertools;

mod tools;

fn main() {
     let lines = tools::read_file_to_lines("aoc202102.txt");
     // part1
     let mut down:u64 = 0;
     let mut forward: u64 = 0;
     for line in lines.clone(){
          let tup:(String, String) = line.split_whitespace().map(|x| x.to_string()).collect_tuple().unwrap();
          let length = tup.1.parse::<u64>().unwrap();
          //print!("{} {} ",tup.0,tup.1);
          match tup.0.as_str() {
               "down" => down += length,
               "up" => down -=length,
               "forward" => forward += length,
               _ => print!("blad")
          }
      //    println!("{} {}",down,forward);
     }

     let out1 = down * forward;

     
    down = 0;
    forward = 0;

     println!("AOC2021 02a: {}", out1);


     let mut depth: u64  = 0;
     for line in lines{
          let tup:(String, String) = line.split_whitespace().map(|x| x.to_string()).collect_tuple().unwrap();
          let length = tup.1.parse::<u64>().unwrap();
          match tup.0.as_str() {
               "down" => down += length,
               "up" => down -=length,
               "forward" => {depth += down*length;
                             forward += length;
                              },
               _ => print!("blad")
          }
          print!("{} {} ",tup.0,tup.1);
          println!("aim: {} forward: {} depth: {}",down,forward,depth);
     }

     let out2 = depth * forward;

     
     
     println!("AOC2021 02b: {}", out2);
}