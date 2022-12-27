
use std::{cmp, sync::Arc};
#[derive(Debug)]
#[allow(dead_code)]
pub struct BigFloat{
    sign: i32,
    front: Vec<i32>,
    back: Vec<i32>,
    prec:i64
}

 impl BigFloat{
    pub fn instance(sign: i32,front: Vec<i32>,back: Vec<i32>) -> Self{
        let p = cmp::max(front.len()+1,back.len()+1) as i64;
        BigFloat{sign,
            front,
            back,
            prec:p}
    }
    pub fn zero() -> Self{
        BigFloat{sign:0,
            front: vec![0],
            back: vec![0],
            prec:1}
    }
    
    pub fn fix(mut a:BigFloat,mut b:BigFloat) -> (BigFloat,BigFloat){
        let mut ma:BigFloat = Self::instance(0,vec![0],vec![0]);
        let mut mb:BigFloat= Self::instance(0,vec![0],vec![0]);
        let mut zeroes= vec![0;(a.prec-b.prec).abs() as usize];
        if a.prec>b.prec{
            zeroes.append(&mut vec![b.front[0]]);
            println!("{:?}",zeroes);
            mb.front = zeroes;
            zeroes = vec![0;(a.prec-b.prec).abs() as usize];
            zeroes.append(&mut b.back);
            mb.back = zeroes;
            a.prec = a.prec;
            mb.prec = a.prec;
            (a,mb)
        }
        else if a.prec<b.prec {
            a.back.append(&mut zeroes);
            println!("{:?}",zeroes);
            ma.back = a.back;
            zeroes= vec![0;(a.prec-b.prec).abs() as usize];
            zeroes.append(&mut a.front);
            ma.front = zeroes;
            ma.prec = a.prec;
            b.prec = a.prec;
            (ma,b)
        }else{
            return (a,b)
        }
        
    }
    fn clamp(mut a:BigFloat) ->BigFloat{
        let mut ma = BigFloat::zero();
        if(a.back.len() > a.front.len()){
            let mut zeroes = vec![0;(a.back.len() - a.front.len()) as usize];
            zeroes.append(&mut a.front);
            ma.front = zeroes;
        }
        ma
    }
    pub fn eq(a:BigFloat,b:BigFloat) -> bool{
        if a.sign != b.sign{
            return false;
        }
        if a.front !=b.front{
            return false
        }
        if a.back!=b.back{
            return false
        }
        return true
    }
    fn gt(a:BigFloat,b:BigFloat) -> bool{
        if a.sign < b.sign{
            return false;
        }
        for i in 0..(a.back.len()){
            if a.back[i] < b.back[i]{
                return false;
            }
        }
        for i in 0..(a.front.len() ){
            if a.front[i] < b.front[i]{
                return false;
            }
        }
        return true;
    }
    fn lt(a:BigFloat,b:BigFloat) -> bool{
        return !Self::gt(a,b)
    }
    fn add(a:BigFloat,b:BigFloat){
        
    }
    // def fix(self,other):
        // if self.prec > other.prec:
            // other.front= [0]*(self.prec-other.prec) + self.front
            // other.back+=[0]*(self.prec-other.prec)
            // other.prec=self.prec
        // elif other.prec>self.prec:
            // self.front=[0]*(other.prec-self.prec)+self.front
            // self.back+= [0]*(other.prec-self.prec)
            // self.prec=other.prec
}