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
    // Multiply, // not found in this scope
}

#[cfg(test)]
mod calc_tests {
    use super::*;
    #[test]
    fn test_add_operation() {
        let add = Add { a: 1, b: 2 };
        let add_op = Operation::Add(add);
        assert_eq!(add_op.calculate(), 3);

        let sub = Sub { a: 1, b: 2 };
        // let sub_op = Operation::Add(sub);
        let sub_op = Operation::Sub(sub);
        assert_eq!(sub_op.calculate(), -1);
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
        // dog.make_sound();
        // cat.make_sound();

        // 创建一个存储 trait 对象的向量
        let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
        // 遍历向量并调用每个 trait 对象的方法
        for animal in animals {
            animal.make_sound();
        }
    }
}


/*
# enum_dispatch 的类型推导说明

这是一个很好的问题！让我来解释一下这个过程：

1. **类型推导过程**：
   - 当你使用 `#[enum_dispatch(Calculator)]` 标注 `Operation` 枚举时，`enum_dispatch` 宏会在编译时扫描代码
   - 它会寻找所有实现了 `Calculator` trait 的类型
   - 在你的代码中，它发现了 `Add` 和 `Sub` 这两个结构体实现了 `Calculator` trait

2. **为什么不用 `Calculator` 作为类型**：
   - 如果使用 `Add(Calculator)` 这种方式，就变成了 trait 对象，会导致动态分发
   - 这违背了 `enum_dispatch` 的设计初衷，因为它的目的就是要避免运行时开销
   - 通过直接使用具体类型 `Add(Add)` 和 `Sub(Sub)`，编译器可以在编译时就确定具体的类型，实现静态分发

3. **自动匹配机制**：
   - `enum_dispatch` 会自动匹配枚举变体名称和实现了对应 trait 的结构体名称
   - 在你的代码中：
     - 发现 `Operation` 枚举有 `Add` 变体 → 匹配到实现了 `Calculator` 的 `Add` 结构体
     - 发现 `Operation` 枚举有 `Sub` 变体 → 匹配到实现了 `Calculator` 的 `Sub` 结构体

4. **命名约定**：
   - 枚举变体的名称必须与实现了对应 trait 的结构体名称相匹配
   - 这就是为什么你的枚举变体名称要和结构体名称保持一致

这种设计让 `enum_dispatch` 能够在保持类型安全的同时，提供比传统 trait 对象更高效的实现方式。
 */