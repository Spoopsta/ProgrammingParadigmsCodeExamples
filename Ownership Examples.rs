fn main(){
    let string = String::from("This is a String.");
    claims_ownership(&string);
    println!("{}", string);

    let string2 = String::from("Test");
    let (string2, str_length) = uses_then_returns(string2);
    println!("{} is {} chars", string2, str_length);

    let test  = String::from("test");
    let new = ("this is a {}", &test);
    println!("{}", test);
}

fn claims_ownership(val: &String){
    println!("the length of the String is {} characters",
        val.len());
}

fn uses_then_returns(val: String) -> (String, usize){
    let length = val.len();
    (val, length)
}