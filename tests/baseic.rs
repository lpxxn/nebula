use clap::builder::Str;

#[cfg(test)]
mod tests {
    #[test]
    fn test_re_borrow() {
        use super::*;
        // 测试重新借用的例子
        let mut s = String::from("hello");

        // 可变借用
        let r1 = &mut s;
        r1.push_str(", world");
        str_append_v(r1, "aaa");
        // 重新借用
        let r2 = &mut *r1;
        r2.push_str(" hey");
        // assert_eq!(*r2, "hello, world hey");
        println!("重新借用测试通过: {}", r2);
        r1.push_str("bar");
        println!("r1: {}", r1);
    }

    #[test]
    fn test_simple_reborrow() {
        let mut x = 42;
        let r1 = &mut x;

        // 这两种调用方式在效果上是完全一样的:
        // 1. modify(r1) - 直接传递可变引用
        // 2. modify(&mut *r1) - 显式地重新借用
        // 编译器会在 modify(r1) 时自动进行重新借用转换
        modify(r1);
        modify(&mut *r1); // 通过 Reborrow 传递可变借用

        println!("{}", r1); // r1 仍然有效
    }

    fn modify(val: &mut i32) {
        *val += 1;
    }
}

fn str_append_v(s: &mut String, v: &str) {
    s.push_str(v);
}
