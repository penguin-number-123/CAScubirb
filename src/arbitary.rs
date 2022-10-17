pub use self::BigFloat;
mod arbitary;
use std::cmp;
[#derive(eq)]
pub struct BigFloat{
    sign: i32,
    front: Vec<i32>,
    back: Vec<i32>,
    prec:i64
}

 impl BigFloat{
    fn new(sign: i32,front: Vec<i32>,back: Vec<i32>) -> Self{
        let p = cmp::max(front.len(),back.len()) as i64;
        BigFloat{sign,front,back,prec:p}
    }
    fn fix(BigFloat:a,BigFloat:b)->(BigFloat,BigFloat){
        let mut ma:BigFloat;
        let mut mb:BigFloat
        if a.prec>b.prec{
            mb.front= vec![0;a.prec-b.prec].append(&mut b.front[0]);
            mb.back = mb.back.append(&mut vec![0;a.prec-b.prec]);
        }
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