use std::any::type_name;
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("key type: {}, value type: {}", type_of(key), type_of(value));
    }
    for (key, value) in &scores {
        println!(
            "key is reference: {}, value is reference: {}",
            (*key).is_empty(), // `*key` 强制解引用，如果 `key` 不是引用会报错
            *value + 1         // `*value` 也是强制解引用
        );
    }
    for (key, value) in &scores {
        // 在循环内部使用类型注解
        let key_ref: &String = key;
        let value_ref: &i32 = value;

        println!("{}: {}", key_ref, value_ref);
    }
    for (key, value) in &scores {
        println!(
            "2 key type: {}, value type: {}",
            type_of2(key),
            type_of2(value)
        );
    }
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn type_of2<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
