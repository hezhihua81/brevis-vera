// Image editing module using the image crate
use image::{DynamicImage, ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CropParams {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResizeParams {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformParams {
    pub crop: Option<CropParams>,
    pub resize: Option<ResizeParams>,
    pub brightness: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditResult {
    pub success: bool,
    pub output_path: String,
    pub params: TransformParams,
    pub error: Option<String>,
}

pub fn edit_image(
    input_path: &str,
    output_path: &str,
    crop: Option<String>,
    resize: Option<String>,
    brightness: Option<i32>,
) -> EditResult {
    // Parse input path
    let input = Path::new(input_path);
    if !input.exists() {
        return EditResult {
            success: false,
            output_path: output_path.to_string(),
            params: TransformParams {
                crop: None,
                resize: None,
                brightness: None,
            },
            error: Some(format!("Input file not found: {}", input_path)),
        };
    }

    // Load the image
    let img = match image::open(input) {
        Ok(i) => i,
        Err(e) => {
            return EditResult {
                success: false,
                output_path: output_path.to_string(),
                params: TransformParams {
                    crop: None,
                    resize: None,
                    brightness: None,
                },
                error: Some(format!("Failed to load image: {}", e)),
            }
        }
    };

    let mut processed = img;

    // Apply crop
    let crop_params = if let Some(crop_str) = crop {
        match parse_crop(&crop_str) {
            Ok(crop_parsed) => {
                processed = processed.crop_imm(
                    crop_parsed.x,
                    crop_parsed.y,
                    crop_parsed.width,
                    crop_parsed.height,
                );
                Some(crop_parsed)
            }
            Err(e) => {
                return EditResult {
                    success: false,
                    output_path: output_path.to_string(),
                    params: TransformParams {
                        crop: None,
                        resize: None,
                        brightness: None,
                    },
                    error: Some(format!("Invalid crop params: {}", e)),
                }
            }
        }
    } else {
        None
    };

    // Apply resize
    let resize_params = if let Some(resize_str) = resize {
        match parse_resize(&resize_str) {
            Ok(resize_parsed) => {
                processed = processed.resize(
                    resize_parsed.width,
                    resize_parsed.height,
                    image::imageops::FilterType::Lanczos3,
                );
                Some(resize_parsed)
            }
            Err(e) => {
                return EditResult {
                    success: false,
                    output_path: output_path.to_string(),
                    params: TransformParams {
                        crop: crop_params,
                        resize: None,
                        brightness: None,
                    },
                    error: Some(format!("Invalid resize params: {}", e)),
                }
            }
        }
    } else {
        None
    };

    // Apply brightness adjustment
    let brightness_val = if let Some(b) = brightness {
        if b != 0 {
            processed = adjust_brightness(processed, b);
        }
        Some(b)
    } else {
        None
    };

    // Save the result
    let output = Path::new(output_path);
    if let Err(e) = processed.save(output) {
        return EditResult {
            success: false,
            output_path: output_path.to_string(),
            params: TransformParams {
                crop: crop_params,
                resize: resize_params,
                brightness: brightness_val,
            },
            error: Some(format!("Failed to save image: {}", e)),
        };
    }

    EditResult {
        success: true,
        output_path: output_path.to_string(),
        params: TransformParams {
            crop: crop_params,
            resize: resize_params,
            brightness: brightness_val,
        },
        error: None,
    }
}

fn parse_crop(s: &str) -> Result<CropParams, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 4 {
        return Err("Crop must be in format: x,y,width,height".to_string());
    }

    Ok(CropParams {
        x: parts[0].trim().parse().map_err(|_| "Invalid x value")?,
        y: parts[1].trim().parse().map_err(|_| "Invalid y value")?,
        width: parts[2].trim().parse().map_err(|_| "Invalid width value")?,
        height: parts[3].trim().parse().map_err(|_| "Invalid height value")?,
    })
}

fn parse_resize(s: &str) -> Result<ResizeParams, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Resize must be in format: width,height".to_string());
    }

    Ok(ResizeParams {
        width: parts[0].trim().parse().map_err(|_| "Invalid width value")?,
        height: parts[1].trim().parse().map_err(|_| "Invalid height value")?,
    })
}

fn adjust_brightness(img: DynamicImage, value: i32) -> DynamicImage {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Scale the -100 to 100 range to actual pixel value changes
    let adjustment = (value as f32 * 2.55) as i16;

    for (x, y, pixel) in rgba.enumerate_pixels() {
        let r = (pixel[0] as i16 + adjustment).clamp(0, 255) as u8;
        let g = (pixel[1] as i16 + adjustment).clamp(0, 255) as u8;
        let b = (pixel[2] as i16 + adjustment).clamp(0, 255) as u8;

        output.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    DynamicImage::ImageRgba8(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crop() {
        let result = parse_crop("10,20,100,200").unwrap();
        assert_eq!(result.x, 10);
        assert_eq!(result.y, 20);
        assert_eq!(result.width, 100);
        assert_eq!(result.height, 200);
    }

    #[test]
    fn test_parse_crop_invalid() {
        let result = parse_crop("10,20");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_resize() {
        let result = parse_resize("800,600").unwrap();
        assert_eq!(result.width, 800);
        assert_eq!(result.height, 600);
    }
}
