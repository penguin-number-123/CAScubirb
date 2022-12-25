use arbitary::BigFloat;
struct Num{
    num:BigFloat,
}

impl Num{
    fn new(num:BigFloat) -> Self{
        Num{num:num}
    }
    fn value(&self) -> &BigFloat{
        &self.num
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