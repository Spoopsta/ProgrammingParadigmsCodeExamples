fn main(){
    let mut shape = Quadrilateral{
        name: String::from("Square"),
        height: 5,
        width: 5
    };
    
    println!("{} has a height of {} and width of {}.", 
    shape.name, shape.height, shape.width);
    
    shape.change_height(10);
    shape.change_width(12);
    println!("It now has a height of {} and width of {}", 
    shape.height, shape.width);
}
    
pub struct Quadrilateral {
    name: String,
    height: i32,
    width: i32,
}

impl Quadrilateral{
    pub fn change_height(&mut self, newval: i32){
        self.height = newval;
    }
    
    pub fn change_width(&mut self, newval: i32){
        self.width = newval
    }
}