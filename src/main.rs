use std::io::{stdin,stdout,Write};
mod arbitary;

use crossterm;
fn main() {
    let count:i32 = 0;
    let mut count = 0;
    let mut a:arbitary::BigFloat = arbitary::BigFloat::instance(0,vec![0,1],vec![0,0]);
    let mut b:arbitary::BigFloat = arbitary::BigFloat::instance(0,vec![0,0,0],vec![0,0,1]);
    (a,b) = arbitary::BigFloat::fix(a,b);
    println!("{:?} {:?}",a,b);
    let size = crossterm::terminal::size();
    let clen = size.as_ref().unwrap().0 - 2 ;
    let rlen = size.as_ref().unwrap().1;
    println!("┌{:─^width$}┐","CAScubirb CLI Interface - V0.0.0",width = usize::try_from(clen).ok().unwrap());
    loop{
        let mut s=String::new();
        print!("In[{}]:= ",count);
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "exit" || s=="Exit"{
            println!("Out[{}]: Ending session...",count);
            std::process::exit(0);
        }else if s=="birb birb"{
            println!("Out[{}]: Birb birb (^)>",count);
            std::process::exit(0);
        }
        else{
            println!("Out[{}]: {}",count,s);
        }
        
        count += 1;
        
    }

}
