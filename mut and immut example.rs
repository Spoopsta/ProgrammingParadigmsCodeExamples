fn main(){
    //This will create the error
    //'cannot assign twice to immutable variable'
    //let var = 2;
    //var = 3

    //Assignment and usage of mutable variable
    let mut var2 = 2;
    var2 = var2 + 1;
    println!("{}", var2);

}

