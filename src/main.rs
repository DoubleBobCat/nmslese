use std::env;
use std::fs;

fn main() {
    let args:Vec<String>=env::args().collect();
    let name=&args[1];
    let text=fs::read_to_string(name).unwrap();
    let mut nums:Vec<i32>=Vec::new();
    for line in text.lines(){
        let mut n=0;
        for word in line.split_ascii_whitespace(){
            if word.eq_ignore_ascii_case("nmsl"){
                n+=1;
            }
        }
        nums.push(n);
    }
    let mut s:Vec<i32>=Vec::new();
    let mut i=0;
    loop{
        if i>=nums.len(){
            return;
        }else if nums[i]==0{
            return;
        }else if nums[i]==1 {
            s.pop();
            i+=1;
        }else if nums[i]==2 {
            s.push(nums[i+1]);
            i+=2;
        }else if nums[i]==3{
            let ch=std::char::from_u32(*s.last().unwrap() as u32).unwrap();
            print!("{}",ch);
            i+=1;
        }
    }
}
