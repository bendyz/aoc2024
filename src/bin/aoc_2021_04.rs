mod tools;
use std::fmt; 

fn main() {
     let input: Vec<String> = tools::read_file_to_lines("aoc202104a.txt");
     // part1


     let mut vec: Vec::<Bingo> = vec!();
     let out1 = "";

     for i in 0..(input.len()/6){
          let mut bin  = Bingo::new() ;

          for j in 0..5{
               let inp = input.get(i*6+j).unwrap().clone();
               bin.add_line(inp, j)
          }
          vec.push(bin);
     }

     let input2: String = tools::read_file_to_string("aoc202104.txt");
     // part1
     for i in input2.split(','){
          for  g in vec.iter_mut(){
               g.add_num(i.parse::<i32>().unwrap())
          }

          for g in vec.iter_mut(){
               if  g.marked_bingo == false && g.check_bingo() {
                    println!("Bingo {}", g.sum_false() * i.parse::<i32>().unwrap());
                    g.marked_bingo = true;
               //std::process::exit(0);
               }
          }
     }

    

   //  println!("AOC2021 01a: {}", vec.get(0).unwrap());

     //part2
     let out2 = "";
     println!("AOC2021 01b: {}", out2);
}

#[derive( Copy, Clone)]
struct Field{
     value : i32,
     state : bool,

}

impl Field{
     fn new() ->Field{
          Field{
               value : -1,
               state : false
          }
     }
}

struct Bingo{
    matrix:[[Field;5];5],
    marked_bingo: bool
}


impl Bingo{
     fn check_bingo(&self)->bool{
          
          for i in 0..5{
               if self.matrix[i].into_iter().all(|x| x.state){
                    println!("bingo");
                    println!("{}",self);
                    return true
               }
          }     

          for i in 0..5{
               if self.matrix[0][i].state 
               && self.matrix[1][i].state
               && self.matrix[2][i].state
               && self.matrix[3][i].state
               && self.matrix[4][i].state{
                    println!("bingo");
                    println!("{}",self);
                    return true
               }
          }    

          if self.matrix[0][0].state 
          && self.matrix[1][1].state
          && self.matrix[2][2].state
          && self.matrix[3][3].state
          && self.matrix[4][4].state{
               println!("bingo");
               println!("{}",self);
               return true
          }
          

          if self.matrix[0][4].state 
          && self.matrix[1][3].state
          && self.matrix[2][2].state
          && self.matrix[3][1].state
          && self.matrix[4][0].state{
               println!("bingo");
               println!("{}",self);
               return true
          }

          
          
          return false
     }

     fn set_bingo(&mut self){
          self.marked_bingo = true;
     }

     fn sum_false(&self) -> i32{
          let mut out = 0;
          for i in 0..5{
               for j in 0..5{
                    if !self.matrix[i][j].state{
                         out +=self.matrix[i][j].value
                    }
               }
          }

          return out
     }
     fn new() ->Bingo{
          Bingo{
               matrix : [[Field::new();5];5],
               marked_bingo : false
          }
     }

     fn add_num(&mut self,val:i32){
          for i in 0..5{
               for j in 0..5{
                    if self.matrix[i][j].value == val{
                         self.matrix[i][j].state = true
                    }
               }
          }

   
     }

     fn add_line(&mut self, line: String, linenum: usize) {
          
          for (i,st) in line.split_ascii_whitespace().into_iter().enumerate(){
               self.matrix[linenum][i].value = st.parse::<i32>().unwrap();
          }
     }
}

impl fmt::Display for Bingo {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         // Use `self.number` to refer to each positional data point.
         //write!(f, "({}, {})", self.0, self.1)
          let mut out: String = "".to_string();
          for j in 0..5{
               for i in self.matrix[j]{
                    let w = if i.state == false {" "} else {"|"};
                    out.push_str(&format!("{w}{}{w}  ", &(i.value)));
               };
               out.push_str("\n");
          }
          write!(f, "({})", out)
     }
 }
