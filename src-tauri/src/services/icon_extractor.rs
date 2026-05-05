use base64::Engine;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, SelectObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
};
use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyIcon, DrawIconEx, HICON, DI_NORMAL,
};

const ICON_SIZE: i32 = 32;

/// 将 HICON 转换为 PNG 字节数组
unsafe fn hicon_to_png_bytes(hicon: HICON) -> Result<Vec<u8>, String> {
    let hdc_screen = CreateCompatibleDC(None);
    if hdc_screen.is_invalid() {
        return Err("CreateCompatibleDC 失败".to_string());
    }

    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: ICON_SIZE,
            biHeight: -ICON_SIZE, // top-down DIB
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [Default::default(); 1],
    };

    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    let hbm = CreateDIBSection(
        Some(hdc_screen),
        &bmi,
        DIB_RGB_COLORS,
        &mut bits,
        None,
        0,
    )
    .map_err(|e| format!("CreateDIBSection 失败: {}", e))?;

    let hdc_mem = CreateCompatibleDC(Some(hdc_screen));
    if hdc_mem.is_invalid() {
        let _ = DeleteObject(hbm.into());
        let _ = DeleteDC(hdc_screen);
        return Err("CreateCompatibleDC (mem) 失败".to_string());
    }

    SelectObject(hdc_mem, hbm.into());

    DrawIconEx(
        hdc_mem,
        0,
        0,
        hicon,
        ICON_SIZE,
        ICON_SIZE,
        0,
        None,
        DI_NORMAL,
    )
    .map_err(|e| format!("DrawIconEx 失败: {}", e))?;

    // 读取像素数据（Windows DIB 为 BGRA 格式）
    let pixel_count = (ICON_SIZE * ICON_SIZE) as usize;
    let pixels = std::slice::from_raw_parts(bits as *const u8, pixel_count * 4);

    // BGRA → RGBA 转换
    let mut img_data = vec![0u8; pixel_count * 4];
    for i in 0..pixel_count {
        let src = i * 4;
        let dst = i * 4;
        img_data[dst] = pixels[src + 2];     // R
        img_data[dst + 1] = pixels[src + 1]; // G
        img_data[dst + 2] = pixels[src];     // B
        img_data[dst + 3] = pixels[src + 3]; // A
    }

    let img = image::RgbaImage::from_raw(ICON_SIZE as u32, ICON_SIZE as u32, img_data)
        .ok_or("创建 image buffer 失败")?;

    let mut png_bytes: Vec<u8> = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| format!("PNG 编码失败: {}", e))?;
    }

    // 清理 GDI 资源
    let _ = DeleteDC(hdc_mem);
    let _ = DeleteObject(hbm.into());
    let _ = DeleteDC(hdc_screen);

    Ok(png_bytes)
}

/// 从文件路径提取图标，返回 base64 编码的 PNG 数据 URL
pub fn extract_file_icon_base64(path: &str) -> Result<String, String> {
    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let png_bytes = unsafe {
        let mut shinfo = SHFILEINFOW::default();
        let ret = SHGetFileInfoW(
            windows::core::PCWSTR(wide_path.as_ptr()),
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut shinfo),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );

        if ret == 0 || shinfo.hIcon.is_invalid() {
            return Err("SHGetFileInfo 无法获取图标".to_string());
        }

        let hicon = shinfo.hIcon;
        let result = hicon_to_png_bytes(hicon);
        let _ = DestroyIcon(hicon);
        result?
    };

    let base64_str = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    Ok(format!("data:image/png;base64,{}", base64_str))
}

/// 从文件路径提取图标并缓存到指定目录，返回缓存的 PNG 文件路径
/// 保留此函数供后续需要文件路径的场景使用
pub fn extract_file_icon(path: &str, cache_dir: &Path) -> Result<String, String> {
    let cache_key = compute_cache_key(path);
    let cache_path = cache_dir.join(format!("{}.png", cache_key));

    if cache_path.exists() {
        return Ok(cache_path.to_string_lossy().to_string());
    }

    std::fs::create_dir_all(cache_dir).map_err(|e| e.to_string())?;

    let png_bytes = extract_file_icon_base64(path)?;
    // base64 前缀去掉，只保留数据部分
    let base64_data = png_bytes
        .strip_prefix("data:image/png;base64,")
        .ok_or("无效的 base64 数据")?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_data)
        .map_err(|e| format!("base64 解码失败: {}", e))?;
    std::fs::write(&cache_path, bytes).map_err(|e| e.to_string())?;

    Ok(cache_path.to_string_lossy().to_string())
}

fn compute_cache_key(path: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    format!("{:.16}", hex::encode(hasher.finalize()))
}
