mod functions;
pub struct func{
    name: String,
    inputs: Vec<BigFloat>
}
impl func{
    fn new(name:String,values:Vec<BigFloat>) ->Self{
        func{name:name,inputs:values}
    }
    fn value(name:String, values:Vec<BigFloat>) -> BigFloat{
       BigFloat::zero()
    }
}