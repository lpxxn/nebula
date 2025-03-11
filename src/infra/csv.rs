use crate::infra;
pub fn path() -> String {
    // 使用 mod.rs 中的方法
    infra::get_current_dir()
}
