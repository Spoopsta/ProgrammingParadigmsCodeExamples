fn main(){
    //function stored in variable
    let func = func_var;

    //this will print 4.
    println!("{}", func(2,2) );

    //This will print 6
    //Example of function as a parameter
    println!("{}",fun_as_par(3, 2, func));

    //This will print 10
    //example of function as a parameter, with parameter
    //declared inside parameter declaration.
    func_as_param_declared_in_line(5, &double);
    
    let returned_function = function_return_function(&2);
    println!("{}",returned_function(2));
}

//Example of Function as parameter using "where" Clause=======
fn func_var(x: i32, y: i32) -> i32{
    x + y
}

fn fun_as_par<F>(value:i32, value2:i32, func_input: F)  -> i32
                    where F: Fn(i32, i32) -> i32 {
    func_input(value, value2)
}
//===============================================================

//Example of a function parameter specifying function in brackets=====
fn func_as_param_declared_in_line(value: i32, f: &Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn double(value: i32) -> i32 {
    2 * value
}
//===================================================================

fn function_return_function<'a>(value:& 'a i32) -> 
                            Box<Fn(i32) -> i32 + 'a > {
       Box::new(move |x:i32| x + value)
}

//Extended return function example========================
fn triple<'a>(value:& 'a i32) -> Box<Fn(i32) -> i32 + 'a > {
    let trippled_value = value * 3;
    Box::new(move |x:i32| trippled_value * x)
}
/* Main will look like this.
fn main(){
    let returned_function = triple(&2);

    let final_value = returned_function(2);

    println!("The final value is {}", final_value);

}*/

//=======================================================

