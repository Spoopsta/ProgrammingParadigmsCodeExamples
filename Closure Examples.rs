fn main(){
    let value = 1;

    let closure = |num| {num + value};

    println!("{}", closure(1));
}