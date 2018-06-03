fn main(){
    struct Quadrilateral {
        name: String,
        height: i32,
        width: i32,
    }

    let shape = Quadrilateral{
        name: String::from("Square"),
        height: 5,
        width: 5
    };
    
    let shape2 = Quadrilateral{
        name: String::from("Rectangle"),
        height: 5,
        width: 10,
    };

    println!("the shapes are {} and {} \n", shape.name, shape2.name);
    
    println!("the shape is a {} with a height of {}
    and a width of {}\n", shape.name, shape.height, shape.width);

    match shape2{
        //call the name of the structure with the values you want
        //access to.
        Quadrilateral{name: n, height: h, width: w} => 
        println!("shape2 is a {} with dimensions of {} X {}", n,h,w),
        //If you don't need all the values just call the ones you
        //need followed by '..'
        Quadrilateral{name: n, ..} => 
        println!("shape2 is a {}", n),
    }
}