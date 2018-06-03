fn main(){

    //Create tupple with specific data types
    //and assign values
    let tup: (i32, char, f64) = (7, 'S', 7.0);

    //create initialised tupple and use pattern
    //matching to deconstruct tupple value
    let tup2 = (1, "2", 3.3);
    let (A, B, C) = tup2;
    //This will print "Value at C is: 3.3"
    println!("Value at C is: {}", C)
    
}
