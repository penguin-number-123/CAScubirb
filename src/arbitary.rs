
use std::cmp;

pub struct BigFloat{
    sign: i32,
    front: Vec<i32>,
    back: Vec<i32>,
    prec:i64
}

 impl BigFloat{
    pub fn instant(sign: i32,front: Vec<i32>,back: Vec<i32>) -> Self{
        let p = cmp::max(front.len(),back.len()) as i64;
        BigFloat{sign,
            front,
            back,
            prec:p}
    }
    pub fn Zero() -> BigFloat{
        BigFloat{sign:0,
            front: vec![0],
            back: vec![0],
            prec:1}
    }
    fn fix(a:BigFloat,b:BigFloat) -> (BigFloat,BigFloat){
        let mut ma:BigFloat;
        let mut mb:BigFloat;
        let mut bk= vec![0;(a.prec-b.prec) as usize];
        if a.prec>b.prec{
            bk.append(&mut vec![b.front[0]]);
            mb.front = bk;
            mb.back.append(&mut vec![0;(a.prec-b.prec) as usize]);
        }
        else if (a.prec<b.prec) {
            bk.append(&mut vec![a.front[0]]);
            ma.back = bk;
            ma.front.append(&mut vec![0;(b.prec-a.prec) as usize]);

        }
        (ma,mb)
    }
    fn eq(a:BigFloat,b:BigFloat) -> bool{
        if a.front !=b.front{
            return false
        }
        if a.back!=b.back{
            return false
        }
        return true
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