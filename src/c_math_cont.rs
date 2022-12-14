use arbitary::BigFloat;
struct Num{
    num:BigFloat,
    base:i32
}

impl Num{
    fn new(num:BigFloat,base:i32) -> Self{
        Num{num:num,base:base};
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
        Complex{real:real,imaginary:imaginary};
    }
    fn value(&self) -> (&Num,&Num){
        (&self.real,&self.complex)
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
struct cconst{
    c:Const,
    coeff:BigFloat,
}

impl cconst{
    fn new(c:Const,coeff:BigFloat) -> Self{
        cconst{c:c,coeff:coeff};
    }
    fn value(&self) -> &BigFloat{
        c.value()*coeff
    }

}


struct Atom{
    quark1:Box<dyn value>,
    quark2:Box<dyn value>,
    operand:i32, // * / //  etc.
}

struct Term{
    sign:bool,
    data:Vec<Atom>,
}
struct Expr{
    data:Vec<Term>,
}