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
        print!("s: {}", s);
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
        let mut y = x;
        y += 1;
        println!("y: {}", y);

        let s = String::from("hello"); // s 是不可变的
        // 将 s 移动到函数中
        /*
        当您将一个值传递给函数时，Rust 不是传递原始变量，而是：
        复制/移动这个值
        在函数内部创建一个全新的变量来存储这个值
        这个新变量的可变性完全由函数参数的声明决定，与原始变量无关。
        */
        make_mutable(s);
        // 此处不能再使用 s，因为它的所有权已经被移动
        // println!("{}", s);  // 这行会导致编译错误
        let s = String::from("hello"); // s 是不可变的
        let mut s2 = s; // s2 是可变的
        s2.push_str(" world"); // 可以修改 s2
        println!("s2 = {}", s2); // 输出: "s2 = hello world"
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

    #[test]
    fn test_iter() {
        let arr = [String::from("a"), "b".to_string(), "c".to_string()];
        let a1 = &arr[0];
        // let mut arr_iter = arr.into_iter();
        // let mut arr_iter = arr.iter();
        let mut arr_iter = (&arr).into_iter();

        assert_eq!(arr_iter.next(), Some(&"a".to_string()));
        assert_eq!(arr_iter.next(), Some(&"b".to_string()));
        assert_eq!(arr_iter.next(), Some(&"c".to_string()));
        assert_eq!(arr_iter.next(), None);
        let a2 = &arr[1];
    }
}

fn str_append_v(s: &mut String, v: &str) {
    s.push_str(v);
}
