fn main(){
    let var = 2;
    match var{
        //As var's value is 2 this first branch will run
        2 => println!("Var's value is 2."),
        3 => println!("Var's value is 3."),
        _ => println!("Var's value is neither 2 or 3")
    }
}

