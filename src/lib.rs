pub mod infra;
// pub mod crate::infra;
pub use crate::infra::csv;
/*
- 告诉 Rust 编译器这个项目中有一个名为 infra 的模块
- 编译器会去寻找以下两种文件之一：
- /Users/li/go/src/github.com/lpxxn/nebula/src/infra.rs
- /Users/li/go/src/github.com/lpxxn/nebula/src/infra/mod.rs

pub mod 是 Rust 中的可见性修饰符组合，用于声明一个公开的模块。这里使用 pub mod 的原因是：

1. 可见性控制 ：
   - pub 表示这个模块是公开的，可以被外部代码访问
   - 如果只写 mod infra ，那么这个模块就是私有的，只能在当前 crate 内部使用
2. 库的对外接口 ：
   - 因为这是在 lib.rs 中，它是库的入口点
   - 如果想让其他项目或者自己的 main.rs 使用 infra 模块中的功能，就必须将其声明为 pub
3. 模块访问链 ：
   - 要让外部代码访问 infra/csv.rs 中的内容，需要确保从 lib.rs 到目标内容的整个路径都是公开的
   - 访问链： lib.rs -> infra -> csv -> path()
 */
