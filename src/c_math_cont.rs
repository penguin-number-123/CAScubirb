use crate::arbitary::BigFloat;

struct Num{
    num:BigFloat,
    base:i32
}

impl Num{
    fn new(num:BigFloat,base:i32) -> Num{
        return Num{num:num,base:base};
    }
    fn value(&self) -> &BigFloat{
        &self.num
    }
}
struct Complex{
    real:Num,
    imaginary:Num
}
impl Complex{
    fn new(real:Num,imaginary:Num) -> Self{
        return Complex{real:real,imaginary:imaginary};
    }
}
struct Const{
    name:String,
    value:BigFloat,
}
impl Const{
    fn new(name:String,value:BigFloat) -> Self{
        Const{name:name,value:value}
    }
    fn value(&self) -> &BigFloat{
        &self.value
    }
}
struct Cconst{
    c:Const,
    coeff:BigFloat,
}

impl Cconst{
    fn new(c:Const,coeff:BigFloat) -> Self{
       return Cconst{c:c,coeff:coeff};
    }

}
struct Var{
    name:String,
}
enum Atom{
   Num(Num),
   Complex(Complex),
   Var(Var),
   Cconst(Cconst),
   Const(Const),

}
struct Egg{
    atom:Atom,
    atom2:Atom,
    op:i32
}
struct Term{
    term:Egg
}
struct Expr{
    expression:Vec<Term>
}