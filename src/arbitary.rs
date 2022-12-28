
use std::{cmp, sync::Arc};
//printing
#[derive(Debug)]
//reduce warnings, nothing serious.
#[allow(dead_code)]
//easier to code with parens. so why not.
#[allow(unused_parens)]
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

    pub fn zero() -> Self{
        BigFloat{sign:0,
            front: vec![0],
            back: vec![0],
            prec:1}
    }
    pub fn clamp(mut a:BigFloat) ->BigFloat{
        let mut ma = BigFloat::zero();
        if a.back.len() > a.front.len(){
            //a.back is longer, so append zeroes to a.front
            let mut zeroes = vec![0;(a.back.len() - a.front.len()-1) as usize];
            zeroes.append(&mut a.front);
            ma.front = zeroes;
            ma
        }else if a.back.len() == a.front.len(){
            //default case
            a
        }else{
            //othewise, reduce time.
            //a.front is longer, append to back.
            let mut zeroes = vec![0;(a.front.len()-a.back.len()-1) as usize];
            a.back.append(&mut zeroes);
            a
        }
    }
    pub fn fix(mut a:BigFloat,mut b:BigFloat) -> (BigFloat,BigFloat){
        //Make sure that the Bfs are clamped.
        a = BigFloat::clamp(a);
        b = BigFloat::clamp(b);
        let mut ma:BigFloat = Self::instance(0,vec![0],vec![0]);
        let mut mb:BigFloat= Self::instance(0,vec![0],vec![0]);
        let mut zeroes= vec![0;(a.prec-b.prec).abs() as usize];
        if a.prec>b.prec{
            zeroes.append(&mut vec![b.front[0]]);
            //println!("{:?}",zeroes);
            mb.front = zeroes;
            zeroes = vec![0;(a.prec-b.prec-1).abs() as usize];
            zeroes.append(&mut b.back);
            mb.back = zeroes;
            a.prec = a.prec;
            mb.prec = a.prec;
            (a,mb)
        }
        else if a.prec<b.prec {
            a.back.append(&mut zeroes);
            //println!("{:?}",a.back);
            ma.back = a.back;
            zeroes= vec![0;(a.prec-b.prec).abs() as usize];
            zeroes.append(&mut a.front);
            ma.front = zeroes;
            ma.prec = b.prec;
            b.prec = b.prec;
            (ma,b)
        }else{
            return (a,b)
        }
        
    }
    
    pub fn eq(mut a:BigFloat,mut b:BigFloat) -> bool{
        (a,b) = BigFloat::fix(a,b);
        //make sure both are same length, otherise cringe.
        if a.sign != b.sign{
            //fastest case
            return false;
        }
        //given a.front: [0,0,0,1,0,2], b.front:[0,0,0,1,0,2]
        //They are equal in value but the array is different
        //as such, lzero tells if last elem is zero, and tells whether to count the zero
        //Process is revered for a.back
        let mut lzero = true;
        //could do double way check, but annoying to implement.
        for i in 0..a.front.len() {
            if(lzero){
                if(a.front[i] !=0){
                    lzero = false;
                }
            }
            if(a.front[i] !=0 || (a.front[i]==0 && !lzero)){
                if(b.front[i]!= a.front[i]){
                    //break as soon as possible, faster.
                    return false;

                }
            }
        }
        let backlength = a.back.len();
        //backlength - i is lazy way to reverse array.
        //at 0 it gets the last element.
        for i in 0..backlength {
            if(lzero){
                if(a.front[backlength-1 - i] !=0){
                    lzero = false;
                }
            }
            if(a.front[backlength-1 - i] !=0 || (a.front[backlength-1 - i]==0 && !lzero)){
                if(b.front[backlength-1 - i]!= a.front[backlength-1 - i]){
                    //break as soon as possible, faster.
                    return false;

                }
            }
        }
        return true
    }
    ///Truncates a number
    ///#Arguments
    ///* `a` - a mutable number to be truncated.  
    ///a.trunc() will remove excess padded 0s,
    /// 
    ///{0,[0,0,1,2,3],[3,2,1,0,0],5} ->{0,[1,2,3],[3,2,1],3}
    ///Saves excess operations due to meaningless zeros.
    /// 
    ///#Examples
    ///```
    ///let mut a = BigFloat::instance(0,[0,0,1],[1,0,0]);
    ///a.trunc();
    ///assert_eq!(a.front,[1]);
    ///assert_eq!(a.back,[1]);
    ///assert_eq!(a.prec,1);
    ///```
    
    pub fn trunc(mut self) ->Self {
        self = BigFloat::clamp(self);
        let mut max_zeroes = 0;
        for i in 0..self.front.len(){
            if(self.front[i] == self.back[self.back.len()-1-i] && max_zeroes+1 <self.front.len()){
                max_zeroes+=1;             
            }else{
                break;
            }
        }
        self.front.drain(0..max_zeroes);
        self.back.reverse();
        self.back.drain(0..max_zeroes);
        self.back.reverse();
        self.prec = self.front.len() as i64;
        return self;
        
    }
    /**Greater than
     * Returns true if a > b, else false.
     * Takes in mutable
     */
    fn gt(mut a:BigFloat,mut b:BigFloat) -> bool{
        (a,b) = BigFloat::fix(a,b);
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
    /**Less than
     * Evaluates a<b.
     * Returns b>a.
     */
    fn lt(mut a:BigFloat,mut b:BigFloat) -> bool{
        //the big lazy
        return Self::gt(b,a)

    }
    /**Add
     * 
     */
    fn add(mut a:BigFloat,mut b:BigFloat){
        (a,b) = BigFloat::fix(a,b);

        if(a.sign == b.sign){
            //a = -x, b = -y, a+b = -(x+y)
            
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