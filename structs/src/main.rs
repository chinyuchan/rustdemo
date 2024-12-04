#![allow(dead_code, unused)]
// named struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// tuple struct相当于有名字的tuple
#[derive(Debug)]
struct Point(i32, i32);

// tuple struct常用于包装单个类型成一个新的类型
struct PoundsOfForce(f64);
struct Newtons(f64);

// 如果只用f64类型，可能无法区分或用错某些函数
fn compute_thruster_force() -> PoundsOfForce {
    todo!()
}

fn set_thruster_force(force: Newtons) {
    todo!()
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 25,
    };
    let alice2 = Person { age: 18, ..alice };

    println!("{:?}", alice2);
    // println!("{:?}", alice); // moved

    let p = Point(1, 2);
    println!("{:?}", p);
}
