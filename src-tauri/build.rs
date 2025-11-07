fn main() {
    // vcpkg 경로 설정
    #[cfg(target_os = "windows")]
    {
        let vcpkg_root = std::path::Path::new("C:/vcpkg");
        if vcpkg_root.exists() {
            std::env::set_var("VCPKG_ROOT", vcpkg_root);
            println!("cargo:rustc-env=VCPKG_ROOT={}", vcpkg_root.display());
            
            // 정적 링크 강제
            std::env::set_var("LIBHEIF_STATIC", "1");
            println!("cargo:rustc-env=LIBHEIF_STATIC=1");
        }
    }
    
    tauri_build::build()
}
