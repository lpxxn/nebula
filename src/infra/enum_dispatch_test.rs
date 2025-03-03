#![cfg(test)]

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait Calculator {
    fn calculate(&self) -> i32;
}

struct Add {
    a: i32,
    b: i32,
}

impl Calculator for Add {
    fn calculate(&self) -> i32 {
        self.a + self.b
    }
}

struct Sub {
    a: i32,
    b: i32,
}
impl Calculator for Sub {
    fn calculate(&self) -> i32 {
        self.a - self.b
    }
}

#[enum_dispatch(Calculator)]
enum Operation {
    Add,
    Sub,
}

#[cfg(test)]
mod calc_tests {
    use super::*;
    #[test]
    fn test_add_operation() {
        let add = Add { a: 1, b: 2 };
        let add_op = Operation::Add(add);
        assert_eq!(add_op.calculate(), 3);
    }
}

trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

#[cfg(test)]
mod animal_tests {
    use super::*;

    // 使用传统的 trait 对象
    #[test]
    fn test_animal() {
        let dog = Dog;
        let cat = Cat;
        dog.make_sound();
        cat.make_sound();

        // 创建一个存储 trait 对象的向量
        let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
        // 遍历向量并调用每个 trait 对象的方法
        for animal in animals {
            animal.make_sound();
        }
    }
}
