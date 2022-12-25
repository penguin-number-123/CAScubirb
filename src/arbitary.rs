
use std::cmp;

pub struct BigFloat{
    sign: i32,
    front: Vec<i32>,
    back: Vec<i32>,
    prec:i64
}

impl BigFloat{
    pub fn instance(sign: i32,front: Vec<i32>,back: Vec<i32>) -> Self{
        let p = cmp::max(front.len(),back.len()) as i64;
        BigFloat{sign,
            front,
            back,
            prec:p}
    }
    pub fn zero() -> BigFloat{
        BigFloat{sign:0,
            front: vec![0],
            back: vec![0],
            prec:1}
    }
    //TODO:
    //Fix the array fusion and reassignment
    fn fix(a:BigFloat,b:BigFloat) -> (BigFloat,BigFloat){
        let mut ma:BigFloat = Self::instance(0,vec![0],vec![0]);
        let mut mb:BigFloat= Self::instance(0,vec![0],vec![0]);
        let mut bk= vec![0;(a.prec-b.prec) as usize];
        if a.prec>b.prec{
            bk.append(&mut vec![b.front[0]]);
            mb.front = bk;
            vec![0;(a.prec-b.prec) as usize].append(&mut mb.back);
        }
        else if a.prec<b.prec {
            bk.append(&mut vec![a.front[0]]);
            ma.back = bk;
            let mut zeroes = vec![0;(b.prec-a.prec) as usize];
            ma.front = zeroes.append(&mut a.front);
        }
        (ma,mb)
    }
    //fn clamp(a:BigFloat){
    //    let mut ma = BigFloat::zero();
    //    if(a.back.len() > a.front.len()){
    //        let mut zeroes = &vec![0;(a.back.len() - a.front.len()) as usize];
    //        ma.front = zeroes.append(&mut a.front);
    //    }
    //}
    fn eq(a:BigFloat,b:BigFloat) -> bool{
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