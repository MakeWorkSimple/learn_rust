#[derive(Debug)]
struct Man {
    age: i32,
    gender: &'static str,
}
impl Man {
    fn get_age(&self) {
        println!("age: {}", self.age)
    }
    fn get_gender(&self) {
        println!("gander: {}", self.gender)
    }
}
fn main() {
    println!("{}", "hellow word");
    let x: i32 = sum(1, 100);
    println!("{}", x);
    let man = Man {
        age: 13,
        gender: "mail",
    };
    man.get_age();
    man.get_gender();
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
