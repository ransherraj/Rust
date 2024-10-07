
pub fn to_int(inp:String)->i32{
    let _int:i32 = inp.trim().parse().expect("Not valid integer");
    return _int;
}

pub fn to_string(inp:i32)->String{
    return inp.to_string();
}