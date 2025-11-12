fn main() {
    // vcpkg 경로 설정
    #[cfg(target_os = "windows")]
    {
        // 환경 변수에서 VCPKG_ROOT 확인 (우선순위 1)
        let vcpkg_root = if let Ok(env_path) = std::env::var("VCPKG_ROOT") {
            std::path::PathBuf::from(env_path)
        } else {
            // 하드코딩된 경로들 확인 (우선순위 2)
            let possible_paths = [
                std::path::Path::new("C:/vcpkg"),
                std::path::Path::new("C:/dev/vcpkg"),
            ];
            
            possible_paths
                .iter()
                .find(|path| path.exists())
                .map(|p| p.to_path_buf())
                .unwrap_or_default()
        };
        
        if !vcpkg_root.as_os_str().is_empty() && vcpkg_root.exists() {
            std::env::set_var("VCPKG_ROOT", &vcpkg_root);
            println!("cargo:rustc-env=VCPKG_ROOT={}", vcpkg_root.display());
            println!("cargo:warning=VCPKG_ROOT found at: {}", vcpkg_root.display());
            
            // 정적 링크 강제
            std::env::set_var("LIBHEIF_STATIC", "1");
            println!("cargo:rustc-env=LIBHEIF_STATIC=1");
        } else {
            println!("cargo:warning=VCPKG_ROOT not found. Please set VCPKG_ROOT environment variable.");
        }
    }
    
    tauri_build::build()
}
