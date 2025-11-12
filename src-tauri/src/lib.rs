// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 보안 강화: 경고 활성화
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::path::{Path, PathBuf};
use std::fs;

// 보안 상수
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
const MAX_IMAGE_DIMENSION: u32 = 32767; // 최대 이미지 크기 (Windows 제한)
const MAX_PREVIEW_DIMENSION: u32 = 800; // 미리보기 최대 크기

// 파일 경로 검증 및 정규화 (기존 파일용)
fn validate_and_normalize_path(path_str: &str) -> Result<PathBuf, String> {
    let path = Path::new(path_str);
    
    // 절대 경로로 변환
    let normalized = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|_| "Unable to determine current directory.")?
            .join(path)
    };
    
    // 경로 정규화 (.. 제거) - 파일이 존재해야 함
    let canonical = normalized.canonicalize()
        .map_err(|_| "Invalid file path.")?;
    
    // 경로 길이 제한 (Windows 경로 최대 길이: 260자, 확장 경로: 32767자)
    let path_string = canonical.to_string_lossy();
    if path_string.len() > 260 {
        return Err("File path is too long.".to_string());
    }
    
    Ok(canonical)
}

// 출력 파일 경로 검증 및 정규화 (존재하지 않는 파일용)
fn validate_and_normalize_output_path(path_str: &str) -> Result<PathBuf, String> {
    let path = Path::new(path_str);
    
    // 절대 경로로 변환
    let normalized = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| format!("Unable to determine current directory: {}", e))?
            .join(path)
    };
    
    // 파일명이 있는지 확인
    let file_name = normalized.file_name()
        .ok_or_else(|| "Output path has no filename.".to_string())?;
    
    // 부모 디렉토리 가져오기
    let parent = normalized.parent()
        .ok_or_else(|| "Output path has no parent directory.".to_string())?;
    
    // 부모 디렉토리 정규화 (존재해야 함)
    // 먼저 부모 디렉토리가 존재하는지 확인
    if !parent.exists() {
        return Err(format!(
            "Output directory does not exist: {}",
            parent.display()
        ));
    }
    
    // 부모 디렉토리 정규화
    let canonical_parent = parent.canonicalize()
        .map_err(|e| format!(
            "Output directory '{}' cannot be accessed: {}",
            parent.display(),
            e
        ))?;
    
    // 정규화된 부모 디렉토리와 파일명 결합
    let canonical_path = canonical_parent.join(file_name);
    
    // 경로 길이 제한 (Windows 경로 최대 길이: 260자)
    let path_string = canonical_path.to_string_lossy();
    if path_string.len() > 260 {
        return Err(format!(
            "Output file path is too long ({} characters). Maximum 260 characters allowed.",
            path_string.len()
        ));
    }
    
    // 파일명에 유효하지 않은 문자 확인 (Windows)
    let file_name_str = file_name.to_string_lossy();
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    if file_name_str.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(format!(
            "Output filename contains invalid characters: {}. Invalid characters: < > : \" | ? *",
            file_name_str
        ));
    }
    
    Ok(canonical_path)
}

// 파일 크기 검증
fn validate_file_size(path: &Path) -> Result<u64, String> {
    let metadata = fs::metadata(path)
        .map_err(|_| "Unable to read file information.")?;
    
    let size = metadata.len();
    if size > MAX_FILE_SIZE {
        return Err(format!("File size is too large. Maximum {}MB is supported.", MAX_FILE_SIZE / (1024 * 1024)));
    }
    
    if size == 0 {
        return Err("File is empty.".to_string());
    }
    
    Ok(size)
}

// 파일 확장자 검증
fn validate_heic_extension(path: &Path) -> Result<(), String> {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        if ext_lower == "heic" || ext_lower == "heif" {
            return Ok(());
        }
    }
    Err("Unsupported file format. Only HEIC or HEIF files are supported.".to_string())
}

// 출력 경로 검증
fn validate_output_path(path: &Path, format: &str) -> Result<(), String> {
    // 출력 경로의 확장자 검증
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        let expected_ext = match format.to_lowercase().as_str() {
            "jpg" | "jpeg" => "jpg",
            "png" => "png",
            _ => return Err("Unsupported output format.".to_string()),
        };
        
        if ext_lower != expected_ext {
            return Err(format!("Output file extension does not match the format. Please use .{} extension.", expected_ext));
        }
    } else {
        return Err("Output file has no extension.".to_string());
    }
    
    // 출력 디렉토리 존재 확인
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Output directory does not exist.".to_string());
        }
        
        // 디렉토리 쓰기 권한 확인
        let metadata = fs::metadata(parent)
            .map_err(|_| "Unable to access output directory.".to_string())?;
        if !metadata.is_dir() {
            return Err("Output path parent is not a directory.".to_string());
        }
    }
    
    // 출력 파일이 이미 존재하는 경우 확인
    if path.exists() {
        // 파일이 디렉토리인지 확인
        let metadata = fs::metadata(path)
            .map_err(|_| "Output file exists but cannot be accessed.".to_string())?;
        if metadata.is_dir() {
            return Err("Output path is a directory, not a file.".to_string());
        }
        
        // 파일이 쓰기 가능한지 확인 (Windows에서는 파일이 열려있으면 실패할 수 있음)
        // 여기서는 경고만 하고 덮어쓰기를 시도 (실제 저장 시 에러가 발생하면 처리)
    }
    
    Ok(())
}

// 파일명에서 경로 추출 (에러 메시지용)
fn get_filename_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown file")
        .to_string()
}

// 이미지 크기 검증
fn validate_image_dimensions(width: u32, height: u32) -> Result<(), String> {
    if width == 0 || height == 0 {
        return Err("Invalid image dimensions.".to_string());
    }
    
    if width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION {
        return Err(format!("Image dimensions are too large. Maximum {}x{} pixels are supported.", MAX_IMAGE_DIMENSION, MAX_IMAGE_DIMENSION));
    }
    
    // 메모리 사용량 추정 (너무 큰 이미지 방지)
    let estimated_size = (width as u64) * (height as u64) * 3; // RGB
    if estimated_size > 500 * 1024 * 1024 { // 500MB
        return Err("Image is too large to process.".to_string());
    }
    
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    // XSS 방지를 위한 입력 검증
    let sanitized = name.chars()
        .take(100) // 최대 길이 제한
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || matches!(c, '-' | '_'))
        .collect::<String>();
    format!("Hello, {}! You've been greeted from Rust!", sanitized)
}

#[tauri::command]
async fn convert_heic_to_image(
    input_path: String,
    output_path: String,
    format: String, // "jpg" or "png"
) -> Result<String, String> {
    // 입력 검증
    let format_lower = format.to_lowercase();
    if format_lower != "jpg" && format_lower != "jpeg" && format_lower != "png" {
        return Err("Unsupported format.".to_string());
    }
    
    // 경로 검증 및 정규화
    let validated_input = validate_and_normalize_path(&input_path)
        .map_err(|e| format!("Input file path error: {}", e))?;
    
    // 출력 경로는 존재하지 않는 파일일 수 있으므로 별도 함수 사용
    let validated_output = validate_and_normalize_output_path(&output_path)
        .map_err(|e| format!("Output file path error: {}", e))?;
    
    // 파일 확장자 검증
    validate_heic_extension(&validated_input)?;
    validate_output_path(&validated_output, &format_lower)?;
    
    // 파일 크기 검증
    validate_file_size(&validated_input)?;
    
    // 파일명 추출 (에러 메시지용)
    let input_filename = get_filename_from_path(&validated_input);
    
    use std::fs::File;
    use std::io::Read;

    // HEIC 파일 읽기
    let mut file = File::open(&validated_input)
        .map_err(|e| format!("Unable to open file '{}'. Error: {}. The file may be in use by another program or you may not have permission to read it.", input_filename, e))?;
    
    let mut heic_data = Vec::new();
    file.read_to_end(&mut heic_data)
        .map_err(|e| format!("Unable to read file '{}'. Error: {}. The file may be corrupted or inaccessible.", input_filename, e))?;

    if heic_data.is_empty() {
        return Err(format!("File '{}' is empty or cannot be read.", input_filename));
    }

    // libheif-rs를 사용하여 HEIC 파일 처리
    // 주의: 디버그 모드에서 DLL 파일을 찾을 수 없으면 여기서 패닉이 발생할 수 있습니다.
    // DLL 파일을 실행 파일과 같은 디렉토리에 복사하거나 PATH에 추가하세요.
    let lib_heif = libheif_rs::LibHeif::new();
    
    let ctx = libheif_rs::HeifContext::read_from_bytes(&heic_data)
        .map_err(|e| {
            format!(
                "Unable to parse HEIC file '{}'. Error: {:?}. This may indicate:\n1. The file is corrupted\n2. The file is not a valid HEIC/HEIF file\n3. In debug mode, DLL files (libheif.dll, libx265.dll, libde265.dll) may be missing. See FIX_DEBUG_BUILD.md for details.",
                input_filename, e
            )
        })?;

    // 첫 번째 이미지 핸들 가져오기
    let handle = ctx.primary_image_handle()
        .map_err(|e| format!("Unable to process image '{}'. Error: {:?}", input_filename, e))?;

    // 이미지 크기 가져오기
    let width = handle.width();
    let height = handle.height();
    
    // 이미지 크기 검증
    validate_image_dimensions(width, height)?;

    // 이미지 디코딩 (RGB로)
    let image = lib_heif.decode(
        &handle,
        libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgb),
        None,
    )
    .map_err(|e| format!("Unable to decode image '{}'. Error: {:?}", input_filename, e))?;

    // RGB 평면 가져오기
    let planes = image.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or_else(|| format!("Unable to process image data: {}", input_filename))?;

    let data = interleaved_plane.data;
    let stride = interleaved_plane.stride;

    // image crate의 ImageBuffer로 변환
    let mut img_buffer = image::ImageBuffer::<image::Rgb<u8>, _>::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let offset = (y as usize * stride as usize + x as usize * 3) as usize;
            if offset + 2 < data.len() {
                let r = data[offset];
                let g = data[offset + 1];
                let b = data[offset + 2];
                img_buffer.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
            }
        }
    }

    let dynamic_img = image::DynamicImage::ImageRgb8(img_buffer);

    // 출력 파일명 추출
    let output_filename = get_filename_from_path(&validated_output);
    
    // 출력 파일이 이미 존재하는 경우, 기존 파일 삭제 시도 (덮어쓰기)
    if validated_output.exists() {
        // 파일이 잠겨있거나 다른 프로그램에서 사용 중일 수 있음
        if let Err(e) = fs::remove_file(&validated_output) {
            return Err(format!(
                "Output file '{}' already exists and cannot be overwritten. It may be open in another program. Error: {}",
                output_filename, e
            ));
        }
    }
    
    // 저장
    match format_lower.as_str() {
        "jpg" | "jpeg" => {
            dynamic_img
                .save_with_format(&validated_output, image::ImageFormat::Jpeg)
                .map_err(|e| {
                    let error_str = format!("{}", e);
                    // 에러 메시지에서 일반적인 패턴 확인
                    let detailed_msg = if error_str.contains("Permission") || error_str.contains("permission") {
                        "Permission denied. The file may be read-only or in use by another program. Please close the file if it's open elsewhere."
                    } else if error_str.contains("No such file") || error_str.contains("not found") {
                        "Output directory not found."
                    } else if error_str.contains("already exists") || error_str.contains("exists") {
                        "File already exists and cannot be overwritten. It may be open in another program."
                    } else if error_str.contains("full") || error_str.contains("space") {
                        "Disk is full. Free up space and try again."
                    } else if error_str.contains("being used") || error_str.contains("in use") {
                        "The file is being used by another program. Please close it and try again."
                    } else {
                        &format!("Error: {}", error_str)
                    };
                    format!("Failed to save image '{}'. {}", output_filename, detailed_msg)
                })?;
        }
        "png" => {
            dynamic_img
                .save_with_format(&validated_output, image::ImageFormat::Png)
                .map_err(|e| {
                    let error_str = format!("{}", e);
                    // 에러 메시지에서 일반적인 패턴 확인
                    let detailed_msg = if error_str.contains("Permission") || error_str.contains("permission") {
                        "Permission denied. The file may be read-only or in use by another program. Please close the file if it's open elsewhere."
                    } else if error_str.contains("No such file") || error_str.contains("not found") {
                        "Output directory not found."
                    } else if error_str.contains("already exists") || error_str.contains("exists") {
                        "File already exists and cannot be overwritten. It may be open in another program."
                    } else if error_str.contains("full") || error_str.contains("space") {
                        "Disk is full. Free up space and try again."
                    } else if error_str.contains("being used") || error_str.contains("in use") {
                        "The file is being used by another program. Please close it and try again."
                    } else {
                        &format!("Error: {}", error_str)
                    };
                    format!("Failed to save image '{}'. {}", output_filename, detailed_msg)
                })?;
        }
        _ => return Err("Unsupported format.".to_string()),
    }

    Ok(format!("Conversion completed: {}", output_filename))
}

#[tauri::command]
async fn get_preview_image(input_path: String) -> Result<String, String> {
    // 경로 검증 및 정규화
    let validated_input = validate_and_normalize_path(&input_path)
        .map_err(|e| format!("Input file path error: {}", e))?;
    
    // 파일 확장자 검증
    validate_heic_extension(&validated_input)?;
    
    // 파일 크기 검증 (미리보기는 더 작은 제한 적용)
    validate_file_size(&validated_input)?;
    
    // 파일명 추출 (에러 메시지용)
    let input_filename = get_filename_from_path(&validated_input);
    
    use std::fs::File;
    use std::io::Read;

    // HEIC 파일 읽기
    let mut file = File::open(&validated_input)
        .map_err(|e| format!("Unable to open file '{}'. Error: {}. The file may be in use by another program or you may not have permission to read it.", input_filename, e))?;
    
    let mut heic_data = Vec::new();
    file.read_to_end(&mut heic_data)
        .map_err(|e| format!("Unable to read file '{}'. Error: {}. The file may be corrupted or inaccessible.", input_filename, e))?;

    if heic_data.is_empty() {
        return Err(format!("File '{}' is empty or cannot be read.", input_filename));
    }

    // libheif-rs를 사용하여 HEIC 파일 처리
    // 주의: 디버그 모드에서 DLL 파일을 찾을 수 없으면 여기서 패닉이 발생할 수 있습니다.
    // DLL 파일을 실행 파일과 같은 디렉토리에 복사하거나 PATH에 추가하세요.
    let lib_heif = libheif_rs::LibHeif::new();
    
    let ctx = libheif_rs::HeifContext::read_from_bytes(&heic_data)
        .map_err(|e| {
            format!(
                "Unable to parse HEIC file '{}'. Error: {:?}. This may indicate:\n1. The file is corrupted\n2. The file is not a valid HEIC/HEIF file\n3. In debug mode, DLL files (libheif.dll, libx265.dll, libde265.dll) may be missing. See FIX_DEBUG_BUILD.md for details.",
                input_filename, e
            )
        })?;

    // 첫 번째 이미지 핸들 가져오기
    let handle = ctx.primary_image_handle()
        .map_err(|e| format!("Unable to process image '{}'. Error: {:?}", input_filename, e))?;

    // 이미지 크기 가져오기
    let width = handle.width();
    let height = handle.height();
    
    // 이미지 크기 검증
    validate_image_dimensions(width, height)?;

    // 이미지 디코딩 (RGB로)
    let image = lib_heif.decode(
        &handle,
        libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgb),
        None,
    )
    .map_err(|e| format!("Unable to decode image '{}'. Error: {:?}", input_filename, e))?;

    // RGB 평면 가져오기
    let planes = image.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or_else(|| format!("Unable to process image data: {}", input_filename))?;

    let data = interleaved_plane.data;
    let stride = interleaved_plane.stride;

    // image crate의 ImageBuffer로 변환
    let mut img_buffer = image::ImageBuffer::<image::Rgb<u8>, _>::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let offset = (y as usize * stride as usize + x as usize * 3) as usize;
            if offset + 2 < data.len() {
                let r = data[offset];
                let g = data[offset + 1];
                let b = data[offset + 2];
                img_buffer.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
            }
        }
    }

    let dynamic_img = image::DynamicImage::ImageRgb8(img_buffer);
    
    // 미리보기를 위해 최대 크기 제한
    let resized_img = if width as u32 > MAX_PREVIEW_DIMENSION || height as u32 > MAX_PREVIEW_DIMENSION {
        let ratio = (MAX_PREVIEW_DIMENSION as f32 / width.max(height) as f32).min(1.0);
        let new_width = (width as f32 * ratio) as u32;
        let new_height = (height as f32 * ratio) as u32;
        dynamic_img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        dynamic_img
    };

    // 임시 파일에 PNG로 저장한 후 메모리로 읽기
    use std::fs;
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("heic_preview_{}.png", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
    
    // PNG로 저장
    resized_img
        .save_with_format(&temp_file, image::ImageFormat::Png)
        .map_err(|_| format!("Unable to generate preview image: {}", input_filename))?;
    
    // 파일 내용을 메모리로 읽기
    let buffer = fs::read(&temp_file)
        .map_err(|_| format!("Unable to read preview data: {}", input_filename))?;
    
    // 임시 파일 삭제 (보안: 민감한 데이터 제거)
    let _ = fs::remove_file(&temp_file);

    // base64로 인코딩
    use base64::{Engine as _, engine::general_purpose};
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    Ok(format!("data:image/png;base64,{}", base64_string))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|_app| {
            // Tauri 2.x에서는 bundle.icon에 설정된 아이콘이 자동으로 윈도우 아이콘으로 사용됩니다
            // tauri.conf.json의 bundle.icon 배열에 설정된 첫 번째 아이콘이 적용됩니다
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, convert_heic_to_image, get_preview_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
