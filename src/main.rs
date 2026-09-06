use clap::{Parser, ValueEnum};
use image::{Rgb, RgbImage, open};
use lab::Lab;
use std::fs;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum BuiltinTheme {
    Nord,
    Gruvbox,
    Catppuccin,
}

#[derive(Parser)]
#[command(name = "theemer-rs", version, about = "change the theme of any images")]
struct Args {
    // Input image path
    input_file: PathBuf,

    // Output image path
    #[arg(short, default_value = "output.png")]
    output_file: PathBuf,

    // temperature value
    #[arg(long, default_value_t = 6.0)]
    temp: f32,

    // Built-in palette theme
    #[arg(long, value_enum, default_value_t = BuiltinTheme::Nord)]
    theme: BuiltinTheme,

    // Custom palette file containing hex colors
    #[arg(long)]
    theme_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 1. Resolve palette (custom file overrides built-in theme)
    let palette_rgb = if let Some(file_path) = &args.theme_file {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read theme file '{:?}': {}", file_path, e))?;

        let parsed: Vec<[u8; 3]> = content.split_whitespace().filter_map(hex_to_rgb).collect();

        if parsed.is_empty() {
            return Err("Theme file contained no valid 6-digit hex color codes.".into());
        }
        parsed
    } else {
        get_builtin_palette(args.theme)
    };

    // Pre-convert palette to CIELAB space
    let palette_lab: Vec<Lab> = palette_rgb.iter().map(Lab::from_rgb).collect();

    // Open image
    let input_image = open(&args.input_file)
        .map_err(|e| format!("Failed to open image '{:?}': {}", args.input_file, e))?
        .to_rgb8();

    let (width, height) = input_image.dimensions();
    let mut output = RgbImage::new(width, height);

    // prosses the pixels
    for x in 0..width {
        for y in 0..height {
            let pixel = input_image.get_pixel(x, y);
            let new_pixel = closest_scheme(pixel[0], pixel[1], pixel[2], args.temp, &palette_lab);

            output.put_pixel(x, y, Rgb(new_pixel.to_rgb()));
        }
    }

    // Save output image
    output.save(&args.output_file).map_err(|e| {
        format!(
            "Failed to save output image to '{:?}': {}",
            args.output_file, e
        )
    })?;

    println!("Image successfully saved to {:?}", args.output_file);
    Ok(())
}

fn get_builtin_palette(theme: BuiltinTheme) -> Vec<[u8; 3]> {
    match theme {
        BuiltinTheme::Nord => vec![
            [25, 29, 36],
            [30, 34, 42],
            [34, 38, 48],
            [36, 41, 51],
            [46, 52, 64],
            [59, 66, 82],
            [67, 76, 94],
            [76, 86, 106],
            [96, 114, 138],
            [192, 200, 216],
            [187, 195, 212],
            [216, 222, 233],
            [229, 233, 240],
            [236, 239, 244],
            [94, 129, 172],
            [129, 161, 193],
            [136, 192, 208],
            [143, 188, 187],
            [159, 198, 197],
            [128, 179, 178],
            [191, 97, 106],
            [197, 114, 122],
            [183, 78, 88],
            [208, 135, 112],
            [215, 151, 132],
            [203, 119, 93],
            [235, 203, 139],
            [239, 212, 159],
            [231, 193, 115],
            [163, 190, 140],
            [177, 200, 157],
            [151, 182, 124],
            [180, 142, 173],
            [190, 157, 184],
            [169, 126, 161],
        ],
        BuiltinTheme::Gruvbox => vec![
            [29, 32, 33],
            [40, 40, 40],
            [50, 48, 47],
            [60, 56, 54],
            [80, 73, 69],
            [102, 92, 84],
            [124, 111, 100],
            [146, 131, 116],
            [249, 245, 215],
            [251, 241, 199],
            [242, 229, 188],
            [235, 219, 178],
            [213, 196, 161],
            [189, 174, 147],
            [168, 153, 132],
            [251, 73, 52],
            [184, 187, 38],
            [250, 189, 47],
            [131, 165, 152],
            [211, 134, 155],
            [142, 192, 124],
            [254, 128, 25],
            [204, 36, 29],
            [152, 151, 26],
            [215, 153, 33],
            [69, 133, 136],
            [177, 98, 134],
            [104, 157, 106],
            [214, 93, 14],
            [157, 0, 6],
            [121, 116, 14],
            [181, 118, 20],
            [7, 102, 120],
            [143, 63, 113],
            [66, 123, 88],
            [175, 58, 3],
        ],
        BuiltinTheme::Catppuccin => vec![
            [244, 219, 214],
            [240, 198, 198],
            [245, 189, 230],
            [198, 160, 246],
            [237, 135, 150],
            [238, 153, 160],
            [245, 169, 127],
            [238, 212, 159],
            [166, 218, 149],
            [139, 213, 202],
            [145, 215, 227],
            [125, 196, 228],
            [138, 173, 244],
            [183, 189, 248],
            [202, 211, 245],
            [184, 192, 224],
            [165, 173, 203],
            [147, 154, 183],
            [128, 135, 162],
            [110, 115, 141],
            [91, 96, 120],
            [73, 77, 100],
            [54, 58, 79],
            [36, 39, 58],
            [30, 32, 48],
            [24, 25, 38],
        ],
    }
}

fn hex_to_rgb(hex: &str) -> Option<[u8; 3]> {
    let cleaned = hex.trim_start_matches('#');

    if cleaned.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&cleaned[0..2], 16).ok()?;
    let g = u8::from_str_radix(&cleaned[2..4], 16).ok()?;
    let b = u8::from_str_radix(&cleaned[4..6], 16).ok()?;

    Some([r, g, b])
}

fn closest_scheme(r: u8, g: u8, b: u8, temp: f32, palette_lab: &[Lab]) -> Lab {
    let target_lab = Lab::from_rgb(&[r, g, b]);

    let temperature = temp;
    let blend_factor = 0.90;

    let mut total_weight = 0.0;
    let mut sum_l = 0.0;
    let mut sum_a = 0.0;
    let mut sum_b = 0.0;

    for p_lab in palette_lab {
        let dist = calculate_delta(target_lab, *p_lab);
        let weight = (-dist / temperature).exp();

        total_weight += weight;
        sum_l += p_lab.l * weight;
        sum_a += p_lab.a * weight;
        sum_b += p_lab.b * weight;
    }

    if total_weight == 0.0 {
        return target_lab;
    }

    let p_smooth_l = sum_l / total_weight;
    let p_smooth_a = sum_a / total_weight;
    let p_smooth_b = sum_b / total_weight;

    Lab {
        l: (1.0 - blend_factor) * target_lab.l + blend_factor * p_smooth_l,
        a: (1.0 - blend_factor) * target_lab.a + blend_factor * p_smooth_a,
        b: (1.0 - blend_factor) * target_lab.b + blend_factor * p_smooth_b,
    }
}

fn calculate_delta(color1: Lab, color2: Lab) -> f32 {
    let c1 = (color1.a * color1.a + color1.b * color1.b).sqrt();
    let c2 = (color2.a * color2.a + color2.b * color2.b).sqrt();

    let delta_c = c1 - c2;
    let delta_l = color1.l - color2.l;
    let delta_a = color1.a - color2.a;
    let delta_b = color1.b - color2.b;

    let delta_hue = (delta_a * delta_a + delta_b * delta_b - delta_c * delta_c)
        .max(0.0)
        .sqrt();

    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;

    let s_l = 1.0;
    let s_c = 1.0 + 0.045 * c1;
    let s_h = 1.0 + 0.015 * c1;

    ((delta_l / (k_l * s_l)).powi(2)
        + (delta_c / (k_c * s_c)).powi(2)
        + (delta_hue / (k_h * s_h)).powi(2))
    .sqrt()
}
