
trait SpecialObject {
    fn i_am_special(&self) -> &str;
}

#[derive(Debug)]
struct Object {
    id: String,
    value: i32
}

impl Object {
    fn new(id: String, value: i32) -> Self {
        Object {
            id,
            value
        }
    }
}

impl SpecialObject for Object{
    fn i_am_special(&self) -> &str {
        self.id.as_str()
    }

}

fn main() {
    let my_object = Object::new("what?".to_string(), 42);
    println!("I am special: {}", my_object.i_am_special());
    println!("{:?}", my_object);
}