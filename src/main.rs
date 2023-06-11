use std::io::{stdin,stdout,Write};
mod arbitary;
mod c_math_cont;
use std::time::{Instant};
use crossterm::event::{Event, KeyCode, KeyEvent,KeyEventKind,KeyEventState}; /* modify */
use crossterm::{event, terminal,Command};
use std::io::Read;
use rand::prelude::*;
fn draw_terminal(x:i32){
    let size = crossterm::terminal::size();
    let clen = size.as_ref().unwrap().0 - 2 ;
    let _rlen = size.as_ref().unwrap().1;
    println!("┌{:─^width$}┐","CAScubirb CLI Interface - V0.0.0",width = usize::try_from(clen).ok().unwrap());
    let w_pad = usize::try_from(clen).ok().unwrap()-7-x.to_string().len();
    println!("│In[{}]:{:^width$}│",x,width = w_pad)
}
fn main() {
    let _count:i32 = 0;
    let mut count = 0;
    let mut a = arbitary::BigFloat::fromStr("8912341231238389318389331389018859289023872085389503537583589379059054865897238734579807527894590723789480590287397599013095720345727899350758927378905958205723785278937852903457897209348567346907289574756897896798789459723489687987583634682389545738957945789257895723689738957983457289573495728374895672845789627834787563479058370579235895805787502059257390593895378957345012389042359059889345857897937575895275975905759758937589589789475795475238957598757895767534958484890.0");
    let mut b = arbitary::BigFloat::fromStr("8912341231238389318389331389018859289023872085389503537583589379059054865897238734579807527894590723789480590287397599013095720345727899350758927378905958205723785278937852903457897209348567346907289574756897896798789459723489687987583634682389545738957945789257895723689738957983457289573495728374895672845789627834787563479058370579235895805787502059257390593895378957345012389042359059889345857897937575895275975905759758937589589789475795475238957598757895767534958484890.0");
    let now = Instant::now();
    let c = arbitary::BigFloat::add(a,b);
    let d = now.elapsed();
    println!("{:?}",d);
    println!("{:?}",c);
    let size = crossterm::terminal::size();
    let clen = size.as_ref().unwrap().0 - 2 ;
    let _rlen = size.as_ref().unwrap().1;
    println!("┌{:─^width$}┐","CAScubirb CLI Interface - V0.0.0",width = usize::try_from(clen).ok().unwrap());
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    count = 0;
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char(chr),
                    modifiers: event::KeyModifiers::NONE,
                    kind:KeyEventKind::Press,
                    state:KeyEventState::NONE,
                } => {
                    println!("{}",chr);

                },
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::ALT,
                    kind:KeyEventKind::Press,
                    state:KeyEventState::NONE,
                }=>{
                    
                    print!("{}[2J", 27 as char);
                    let r = rand::thread_rng().gen::<f64>();
                    if  r<0.1{
                        println!("birb birb (^)>");
                    }else{
                        println!("Exiting...")
                    }
                    std::process::exit(0);
                },
                KeyEvent{
                    code: KeyCode::Char('C'),
                    modifiers:event::KeyModifiers::SHIFT|event::KeyModifiers::CONTROL,
                    kind:KeyEventKind::Press,
                    state:KeyEventState::NONE,
                }=>{
                    draw_terminal(count)
                }
                KeyEvent{
                    code: KeyCode::Enter,
                    modifiers:event::KeyModifiers::NONE,
                    kind:KeyEventKind::Press,
                    state:KeyEventState::NONE,
                }=>{
                    count+=1;
                    
                }
                _ => {
                    //todo
                }
            
            }
            println!("{:?}",event)
        };
        draw_terminal(count);
        
    }
    /*loop{
        
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
        
    }*/

}
