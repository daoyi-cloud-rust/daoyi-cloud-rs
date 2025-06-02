use wax::{Glob, Pattern};

pub fn path_matches(pattern: &str, target: &str) -> bool {
    // 将通配符模式编译为 Glob 表达式
    let glob = Glob::new(pattern).expect("Invalid glob pattern");
    // 判断目标路径是否匹配该模式
    glob.is_match(target)
}

pub fn path_any_matches(patterns: &[String], target: &str) -> bool {
    patterns.iter().any(|pattern| path_matches(pattern, target))
}
