#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageInfo {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

pub fn decode_header(bytes: &[u8]) -> Option<ImageInfo> {
    decode_png_header(bytes).or_else(|| decode_jpeg_header(bytes))
}

pub fn decode_png_header(bytes: &[u8]) -> Option<ImageInfo> {
    if bytes.len() < 24 {
        return None;
    }
    let sig = &bytes[0..8];
    if sig != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    Some(ImageInfo { format: ImageFormat::Png, width, height })
}

pub fn decode_jpeg_header(bytes: &[u8]) -> Option<ImageInfo> {
    if bytes.len() < 10 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }
    let mut i = 2usize;
    while i + 8 < bytes.len() {
        if bytes[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = bytes[i + 1];
        if marker == 0xC0 || marker == 0xC2 {
            let height = u16::from_be_bytes([bytes[i + 5], bytes[i + 6]]) as u32;
            let width = u16::from_be_bytes([bytes[i + 7], bytes[i + 8]]) as u32;
            return Some(ImageInfo { format: ImageFormat::Jpeg, width, height });
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o4_9_image_decode_png_and_jpeg_headers() {
        let png = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // sig
            0x00, 0x00, 0x00, 0x0D, // len
            0x49, 0x48, 0x44, 0x52, // IHDR
            0x00, 0x00, 0x01, 0x00, // w = 256
            0x00, 0x00, 0x00, 0x80, // h = 128
        ];
        let p = decode_header(&png).expect("png header");
        assert_eq!(p, ImageInfo { format: ImageFormat::Png, width: 256, height: 128 });

        let jpg = [
            0xFF, 0xD8, // SOI
            0xFF, 0xE0, 0x00, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // APP0
            0xFF, 0xC0, 0x00, 0x11, 0x08, // SOF0 + len + precision
            0x00, 0x78, // h = 120
            0x00, 0xA0, // w = 160
            0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00,
        ];
        let j = decode_header(&jpg).expect("jpeg header");
        assert_eq!(j, ImageInfo { format: ImageFormat::Jpeg, width: 160, height: 120 });
    }
}
