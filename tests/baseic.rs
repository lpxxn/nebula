#[cfg(test)]
mod tests {
    #[test]
    fn test_re_borrow() {
        use super::*;
        let mut s = String::from("hello");

        let r1 = &mut s;
        r1.push_str(", world");
        str_append_v(r1, "aaa");

        let r2 = &mut *r1;
        r2.push_str(" hey");
        // assert_eq!(*r2, "hello, world hey");
        println!("reborrwo: {}", r2);
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
    #[test]
    fn test_mutfn() {
        let mut s = String::from("hello");
        let sp = || s.push_str("abfc");
        // sp()
        update_str(sp);
    }

    // fn update_str<T>(mut t: T)
    // where
    // T: FnMut(),
    fn update_str<T: FnMut()>(mut t: T) {
        t();
    }
    #[test]
    fn test_reborrow() {
        let x = 5;
        make_mutalbe(x);

        let s = String::from("hello"); // s 是不可变的
        // 将 s 移动到函数中
        make_mutable(s);
        // 此处不能再使用 s，因为它的所有权已经被移动
        // println!("{}", s);  // 这行会导致编译错误
    }
    fn make_mutable(mut t: String) {
        // t 是一个新的、可变的变量，拥有字符串的所有权
        t.push_str(" world"); // 可以修改 t
        println!("t = {}", t); // 输出: "t = hello world"
    }

    fn make_mutalbe(mut y: i32) {
        y += 1;
        println!("y: {}", y)
    }
}

fn str_append_v(s: &mut String, v: &str) {
    s.push_str(v);
}
