#[cfg(test)]
mod tests {
    // use std::{marker::PhantomPinned, pin::Pin};
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    #[test]
    fn test_re_borrow() {
        struct SelfRef {
            name: String,
            name_ptr: *const String,
        }
        let mut data = SelfRef {
            name: "Alice".to_string(),
            name_ptr: std::ptr::null(),
        };

        // 设置自引用
        data.name_ptr = &data.name;

        // 打印初始值
        println!("Name: {}", data.name);
        println!("Pointer: {}", unsafe { &*data.name_ptr });

        // 移动数据 - 这会导致问题！
        let data2 = data;

        // 危险！data2.name_ptr 指向错误的地址
        println!("After move: {}", unsafe { &*data2.name_ptr }); // 可能崩溃或输出垃圾
    }
    #[test]
    fn test_pin() {
        struct SelfRef {
            name: String,
            name_ptr: *const String,
            _marker: PhantomPinned, // 标记为不可移动
        }

        // 创建一个普通的 SelfRef
        let mut data = SelfRef {
            name: "Alice".to_string(),
            name_ptr: std::ptr::null(),
            _marker: PhantomPinned,
        };

        // 获取 name 的地址
        let name_ptr: *const String = &data.name;
        data.name_ptr = name_ptr;

        // 使用 Box::pin() 而不是 Pin::new()
        let pinned_data = Box::pin(data);

        // 打印数据
        println!("Name: {}", pinned_data.name);
        println!("Pointer: {}", unsafe { &*pinned_data.name_ptr });
        // 这行代码会编译失败！Pin 阻止了移动
        // let data2 = *pinned_data; // 错误
        // 你只能获取引用，不能移动数据
        let reference = Pin::as_ref(&pinned_data);
        println!("Reference: {}", reference.name);
    }
}
