/// `Documents/MaterialCraft/<sub>` in the user's home folder, or `$MATERIALCRAFT_DATA_DIR/<sub>` when overridden
#[cfg(not(target_arch = "wasm32"))]
pub fn user_dir(sub: &str) -> std::path::PathBuf {
    if let Some(root) = std::env::var_os("MATERIALCRAFT_DATA_DIR").filter(|v| !v.is_empty()) {
        return std::path::PathBuf::from(root).join(sub);
    }
    let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".into());
    std::path::Path::new(&home).join("Documents").join("MaterialCraft").join(sub)
}
