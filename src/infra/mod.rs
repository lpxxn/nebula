pub mod csv;
#[cfg(test)] // 这确保这个模块只在测试时编译
mod enum_dispatch_test;
// 当前工作目录
pub fn get_current_dir() -> String {
    use std::env;
    let current_dir = env::current_dir().unwrap();
    current_dir.display().to_string()
}

fn get_current_exe_dir() -> String {
    use std::env;
    let current_exe = env::current_exe().unwrap();
    current_exe.parent().unwrap().display().to_string()
}

fn verify_file_exist(filename: &str) -> Result<String, &'static str> {
    use std::path::Path;
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("file not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_file_exist() {
        let current_dir = get_current_dir();
        print!("current_dir: {}\n", current_dir);
        // 获取当前目录下的文件路径
        let readme: &str = "README.md";
        let file_path = format!("{}/{}", current_dir, readme);
        print!("file_path: {}\n", file_path);
        assert_eq!(verify_file_exist(readme), Ok(readme.to_string()));
        assert_eq!(
            verify_file_exist(file_path.as_str()),
            Ok(file_path.to_string())
        );
        assert_eq!(verify_file_exist("not_exist.txt"), Err("file not found"));
    }
}
