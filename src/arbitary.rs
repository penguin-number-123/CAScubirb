
use std::{cmp, sync::Arc};
use std::thread;
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
    const MUL_TABLE: [[[i32;2];10];10] = [
        [[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0]],
        [[0,0],[0,1],[0,2],[0,3],[0,4],[0,5],[0,6],[0,7],[0,8],[0,9]],
        [[0,0],[0,2],[0,4],[0,6],[0,8],[1,0],[1,2],[1,4],[1,6],[1,8]],
        [[0,0],[0,3],[0,6],[0,9],[1,2],[1,5],[1,8],[2,1],[2,4],[2,7]],
        [[0,0],[0,4],[0,8],[1,2],[1,6],[2,0],[2,4],[2,8],[3,2],[3,6]],
        [[0,0],[0,5],[1,0],[1,5],[2,0],[2,5],[3,0],[3,5],[4,0],[4,5]],
        [[0,0],[0,6],[1,2],[1,8],[1,2],[1,5],[1,8],[2,1],[2,4],[2,7]],
        [[0,0],[0,7],[1,4],[2,1],[2,8],[3,0],[4,2],[4,9],[5,6],[6,3]],
        [[0,0],[0,8],[1,6],[2,4],[3,2],[4,0],[4,8],[5,6],[6,4],[7,2]],
        [[0,0],[0,9],[1,8],[2,7],[3,6],[4,5],[5,4],[6,3],[7,2],[8,1]],
        ];
    //Add sin, cos table later. We can use the formula sin(a+b) = sin(a)cos(b)+cos(a)sin(b).
    //Notice that for a clever choice of b we can do good calculations.

    pub fn instance(sign: i32,front: Vec<i32>,back: Vec<i32>) -> Self{
        let p = cmp::max(front.len(),back.len()) as i64;
        BigFloat{sign,
            front,
            back,
            prec:p}
    }
    pub fn fromStr(s: &str) -> BigFloat{
        if s != ""{
            let sign = if (s.chars().nth(0).unwrap() != '-') { 0 } else { -1 };
            let k = s.replace("-","").replace("+","");
            let a = k.split(".").collect::<Vec<&str>>();
            println!("{:?}",a);
            let front = a[0].chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let back = a[1].chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let prec =  cmp::max(front.len(),back.len()) as i64;
            return BigFloat{sign,
            front,
            back,
            prec:prec}
        }
        return BigFloat::zero();
    }
    pub fn zero() -> Self{
        BigFloat{sign:0,
            front: vec![0],
            back: vec![0],
            prec:1}
    }
    //I don't think this is needed, since it wastes memory.
    //Consider removing it, but we'll see.
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
        //Removed clamping, avoid wasting space with zeros. For larger values it adds up.

        let mut ma:BigFloat = Self::instance(0,vec![0],vec![0]);
        let mut mb:BigFloat= Self::instance(0,vec![0],vec![0]);
        let mut zeroes= vec![0;(a.prec-b.prec).abs() as usize];
        if a.prec>b.prec{
            zeroes.append(&mut vec![b.front[0]]);
            //println!("{:?}",zeroes);
            mb.front = zeroes;
            let mut zero2 = vec![0;(a.prec-b.prec).abs() as usize];
            b.back.append(&mut zero2);
            println!("{:?}",b.back);
            mb.back = b.back;

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
        //given a.front: [0,0,0,1,0,2], b.front:[0,0,1,0,2]
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

    //Useful for optimization, since a+ (-a) = 0
    //Or a/(-a) = -1.
    pub fn unsigneq(mut a:BigFloat,mut b:BigFloat) -> bool{
        (a,b) = BigFloat::fix(a,b);
        
        //given a.front: [0,0,0,1,0,2], b.front:[0,0,1,0,2]
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
    
    /*Absolute Value
     *Changes sign to 0.
     */
    fn abs(mut self){
       self.sign = 0;
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
     * Adds a and b
     * 
     * 
     * 
     */
    pub fn add(mut a:BigFloat,mut b:BigFloat) -> BigFloat{
        
        let mut cthrough = 0;
        //We don't need this since we handle the len diff by just not adding.
        //(a,b) = BigFloat::fix(a,b);
        
        //normalize
        if a.sign == b.sign {
            //a = -x, b = -y, a+b = -(x+y)
           
            if(a.back.len()>b.back.len()){
                for i in 0..b.back.len(){
                    a.back[i]+=b.back[i];
                }
            }else{
                for i in 0..a.back.len(){
                    b.back[i]+=a.back[i];
                }
                a.back = b.back;
            }
                for i in 0..a.front.len(){
                    a.front[i]+=b.front[i];
            }
             for i in 0..a.back.len(){

                if(a.back[i]>=10){
                    if i==0 {
                        cthrough = a.back[i]/10;

                    }else{
                        a.back[i-1] += a.back[i]/10;
                    }
                    a.back[i] = a.back[i] % 10;

                }
            }

            let alen = a.front.len()-1;
            a.front[alen] += cthrough;
            for j in 0..alen{
                if(a.front[alen-j]>=10){

                    if alen-j>0 {
                        a.front[alen-j-1] += a.front[alen-j]/10;
                        a.front[alen-j] = a.front[alen-j]%10
                    }
                    
                }
            }
            a.front.insert(0,a.front[0]/10);
            a.front[1] = a.front[1]%10;
            a.prec = cmp::max(a.front.len(),a.back.len()) as i64;
           return a;
        }else{
            //a + -a = 0, since we know the two signs must be different, as long as the absolute value is the same they cancel out.
            if BigFloat::unsigneq(a,b) {
                return BigFloat::zero();
            }
            //TODO
            return BigFloat::zero();
        }
    }
    /**Add
     * Adds a and b
     * 
     * 
     * 
     */
    pub fn sub(mut a:BigFloat,mut b:BigFloat) -> BigFloat{
        
        let mut cthrough = 0;
        //We don't need this since we handle the len diff by just not adding.
        //(a,b) = BigFloat::fix(a,b);
        
        //normalize
        if a.sign == b.sign {
            //a = -x, b = -y, a+b = -(x+y)
           
            if(a.back.len()>b.back.len()){
                for i in 0..b.back.len(){
                    a.back[i]+=b.back[i];
                }
            }else{
                for i in 0..a.back.len(){
                    b.back[i]+=a.back[i];
                }
                a.back = b.back;
            }
                for i in 0..a.front.len(){
                    a.front[i]+=b.front[i];
            }
             for i in 0..a.back.len(){

                if(a.back[i]>=10){
                    if i==0 {
                        cthrough = a.back[i]/10;

                    }else{
                        a.back[i-1] += a.back[i]/10;
                    }
                    a.back[i] = a.back[i] % 10;

                }
            }

            let alen = a.front.len()-1;
            a.front[alen] += cthrough;
            for j in 0..alen{
                if(a.front[alen-j]>=10){

                    if alen-j>0 {
                        a.front[alen-j-1] += a.front[alen-j]/10;
                        a.front[alen-j] = a.front[alen-j]%10
                    }
                    
                }
            }
            a.front.insert(0,a.front[0]/10);
            a.front[1] = a.front[1]%10;
            a.prec = cmp::max(a.front.len(),a.back.len()) as i64;
           return a;
        }else{
            //a + -a = 0, since we know the two signs must be different, as long as the absolute value is the same they cancel out.
            if BigFloat::unsigneq(a,b) {
                return BigFloat::zero();
            }
            //TODO
            return BigFloat::zero();
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

