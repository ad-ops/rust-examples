fn main() {


    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }

    fn calculate(f: fn(i32, i32) -> i32, x: i32, y: i32 ) -> i32 {
        f(x, y)
    }
    
    let (x, y) = (4, 2);
    println!("variables {}, {}", x, y);
    println!("Trying add = {}", calculate(add, x, y));
    println!("Trying subtract = {}", calculate(subtract, x, y));
}

