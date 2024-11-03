use itertools::Itertools;

mod tools;

fn main() {
     let input: Vec<String> = tools::read_file_to_lines("aoc202103.txt");
     // part1
     let mut array: [u32; 12] = [0; 12];

     for line in input.clone(){
          for (i,c) in line.chars().enumerate(){
               array[i] += c.to_digit(10).unwrap()
          }
     }

     let mut gamma:u32 = 0;
     let mut epsilon:u32 = 0;

     let base: u32 = 2;
     for (i,x) in array.iter().rev().enumerate(){
          
          if x >&500{
               gamma += base.pow(i as u32);
          }
          else {
               epsilon += base.pow(i as u32);
          }
     }

     let out1 = gamma * epsilon;



     println!("AOC2021 01a: {}", out1);




     //part2

     let s1 =  rek(input.clone(), 0,'1');
     let s0 =  rek2(input, 0,'1');
     let mut oxygen: u32 = 0;
     let mut co2: u32 = 0;

     for (i,x) in s1.chars().into_iter().rev().enumerate(){
          
          if x =='1'{
               oxygen += base.pow(i as u32);
          }

     }
     for (i,x) in s0.chars().into_iter().rev().enumerate(){
          if x== '1' {
               co2 += base.pow(i as u32);
          }
     }



     println!("AOC2021 03b: {}", oxygen*co2);



}


fn rek(input : Vec<String>, n: usize, c: char) -> String {

     if input.len() ==1 || n == 12{
          println!( " {} {}", input.first().unwrap(), n);
          return input.first().unwrap().clone();
     }
     else {
          let mut input0: Vec<String> =  vec![] ;
          let mut input1: Vec<String> =  vec![] ;

         for line in input{

           if line.chars().nth(n).unwrap() == c{
               input1.push(line.clone());
           }
           else {
               input0.push(line.clone());
           }
           
         }
         println!("0: {}  1: {}",input0.len(),input1.len());
         if input0.len() > input1.len() {
          return rek(input0,n+1,c)
         }
         else {
          return rek(input1,n+1,c)
         }


     }

}


fn rek2(input : Vec<String>, n: usize, c: char) -> String {

     if input.len() ==1 || n == 12{
          println!( " {} {}", input.first().unwrap(), n);
          return input.first().unwrap().clone();
     }
     else {
          let mut input0: Vec<String> =  vec![] ;
          let mut input1: Vec<String> =  vec![] ;

         for line in input{

           if line.chars().nth(n).unwrap() == c{
               input1.push(line.clone());
           }
           else {
               input0.push(line.clone());
           }
           
         }
         println!("0: {}  1: {}",input0.len(),input1.len());
         if input0.len() <= input1.len() {
          return rek2(input0,n+1,c)
         }
         else {
          return rek2(input1,n+1,c)
         }


     }

}