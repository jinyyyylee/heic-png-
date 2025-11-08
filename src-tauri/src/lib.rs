// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn convert_heic_to_image(
    input_path: String,
    output_path: String,
    format: String, // "jpg" or "png"
) -> Result<String, String> {
    let format_lower = format.to_lowercase();
    if format_lower != "jpg" && format_lower != "jpeg" && format_lower != "png" {
        return Err(format!("지원하지 않는 형식입니다: {}", format));
    }

    use std::fs::File;
    use std::io::Read;

    // HEIC 파일 읽기
    let mut file = File::open(&input_path)
        .map_err(|e| format!("파일을 열 수 없습니다: {}", e))?;
    
    let mut heic_data = Vec::new();
    file.read_to_end(&mut heic_data)
        .map_err(|e| format!("파일을 읽을 수 없습니다: {}", e))?;

    if heic_data.is_empty() {
        return Err("파일이 비어있습니다.".to_string());
    }

    // libheif-rs를 사용하여 HEIC 파일 처리
    let lib_heif = libheif_rs::LibHeif::new();
    let ctx = libheif_rs::HeifContext::read_from_bytes(&heic_data)
        .map_err(|e| format!("HEIC 파일을 파싱할 수 없습니다: {}", e))?;

    // 첫 번째 이미지 핸들 가져오기
    let handle = ctx.primary_image_handle()
        .map_err(|e| format!("이미지 핸들을 가져올 수 없습니다: {}", e))?;

    // 이미지 크기 가져오기
    let width = handle.width();
    let height = handle.height();

    if width == 0 || height == 0 {
        return Err("이미지 크기가 0입니다.".to_string());
    }

    // 이미지 디코딩 (RGB로)
    let image = lib_heif.decode(
        &handle,
        libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgb),
        None,
    )
    .map_err(|e| format!("HEIC 이미지를 디코딩할 수 없습니다: {}", e))?;

    // RGB 평면 가져오기
    let planes = image.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or_else(|| "인터리브된 평면을 가져올 수 없습니다.".to_string())?;

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

    // 저장
    match format_lower.as_str() {
        "jpg" | "jpeg" => {
            dynamic_img
                .save_with_format(&output_path, image::ImageFormat::Jpeg)
                .map_err(|e| format!("JPG 저장 실패: {}", e))?;
        }
        "png" => {
            dynamic_img
                .save_with_format(&output_path, image::ImageFormat::Png)
                .map_err(|e| format!("PNG 저장 실패: {}", e))?;
        }
        _ => return Err(format!("지원하지 않는 형식: {}", format)),
    }

    Ok(format!("변환 완료: {}", output_path))
}

#[tauri::command]
async fn get_preview_image(input_path: String) -> Result<String, String> {
    use std::fs::File;
    use std::io::Read;

    // HEIC 파일 읽기
    let mut file = File::open(&input_path)
        .map_err(|e| format!("파일을 열 수 없습니다: {}", e))?;
    
    let mut heic_data = Vec::new();
    file.read_to_end(&mut heic_data)
        .map_err(|e| format!("파일을 읽을 수 없습니다: {}", e))?;

    if heic_data.is_empty() {
        return Err("파일이 비어있습니다.".to_string());
    }

    // libheif-rs를 사용하여 HEIC 파일 처리
    let lib_heif = libheif_rs::LibHeif::new();
    let ctx = libheif_rs::HeifContext::read_from_bytes(&heic_data)
        .map_err(|e| format!("HEIC 파일을 파싱할 수 없습니다: {}", e))?;

    // 첫 번째 이미지 핸들 가져오기
    let handle = ctx.primary_image_handle()
        .map_err(|e| format!("이미지 핸들을 가져올 수 없습니다: {}", e))?;

    // 이미지 크기 가져오기
    let width = handle.width();
    let height = handle.height();

    if width == 0 || height == 0 {
        return Err("이미지 크기가 0입니다.".to_string());
    }

    // 이미지 디코딩 (RGB로)
    let image = lib_heif.decode(
        &handle,
        libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgb),
        None,
    )
    .map_err(|e| format!("HEIC 이미지를 디코딩할 수 없습니다: {}", e))?;

    // RGB 평면 가져오기
    let planes = image.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or_else(|| "인터리브된 평면을 가져올 수 없습니다.".to_string())?;

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
    
    // 미리보기를 위해 최대 크기 제한 (예: 800px)
    let max_dimension = 800u32;
    let resized_img = if width as u32 > max_dimension || height as u32 > max_dimension {
        let ratio = (max_dimension as f32 / width.max(height) as f32).min(1.0);
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
        .map_err(|e| format!("이미지 인코딩 실패: {}", e))?;
    
    // 파일 내용을 메모리로 읽기
    let buffer = fs::read(&temp_file)
        .map_err(|e| format!("임시 파일 읽기 실패: {}", e))?;
    
    // 임시 파일 삭제 (실패해도 무시)
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
