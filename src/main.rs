use std::io::{stdin,stdout,Write};

fn main() {
    println!("CAScubirb CLI Interfact - V0.0.0 ");
    let mut count = 0;
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
        if(s == "exit".to_string()) {std::process::exit(0);}
        println!("Out[{}]: {}",count,s);

        count += 1;
    }

}
