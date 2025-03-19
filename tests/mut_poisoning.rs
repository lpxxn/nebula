use std::sync::{Arc, Mutex};
use std::thread;
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_poisonign_thread() {
        let counter = Arc::new(Mutex::new(0));
        // let mut handles = vec![];

        let counter_for_panic = counter.clone();
        let panic_handle = thread::spawn(move || {
            let mut num = counter_for_panic.lock().unwrap();
            println!("thread 1 get lock, value: {}", *num);

            *num += 1;
            println!("thread 1 add 1, value: {}", *num);
            panic!("Thread 1 panics, the lock will be poisoned!");
            // 这里的锁会自动释放，但因为panic所以锁被标记为poisoned
        });

        thread::sleep(std::time::Duration::from_millis(200));
        let counter_fo_recovery = Arc::clone(&counter);
        let recovery_handle = thread::spawn(move || {
            println!("线程2尝试获取锁...");

            let lock_result = counter_fo_recovery.lock();
            match lock_result {
                Ok(mut guard) => {
                    print!("线程2得到了锁（不应该发生）");
                    *guard += 1;
                }
                Err(poisoned) => {
                    println!("线程2获取锁失败，锁已经被污染");
                    // 仍然可以通过 `into_inner` 方法或者 get_mut 方法来获取 MutexGuard
                    let mut guard = poisoned.into_inner();
                    println!("线程2获取锁成功，值为：{}", *guard);
                    *guard += 1;
                    println!("线程2释放锁，值为：{}", *guard);
                }
            }
        });

        // panic_handle.join().unwrap();
        // 忽略每一个线程的panic结果
        let _ = panic_handle.join();
        recovery_handle.join().unwrap();
        match counter.lock() {
            Ok(guard) => {
                println!("最终的值为：{}", *guard);
            }
            Err(mut poisoned) => {
                println!("主线程检测到锁已被毒化!");

                // 方法2: 使用get_mut()获取内部值的可变引用
                let value = poisoned.get_mut();
                println!("主线程从毒化的锁中恢复值(using get_mut): {}", *value);

                // 修改恢复的值 - 使用双重解引用
                **value += 10; // 注意这里是双星号
                println!("主线程将值增加到: {}", **value);
            }
        }
    }
}
