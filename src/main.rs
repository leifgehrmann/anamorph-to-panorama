use std::cmp::{max, min};
use std::env;
use std::error::Error;
use std::f32::consts::PI;
use image::{GenericImageView, Pixel, RgbImage};

fn help(script_name: &String) {
    println!("{} [INPUT_FILE] [OUTPUT_FILE] [center X] [center Y] [inner diameter] [outer diameter]", script_name);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        help(&args[0]);
        return Err(format!("Expected 7 arguments, found {}", args.len()).into());
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let center_x = args[3].parse::<u32>().unwrap();
    let center_y = args[4].parse::<u32>().unwrap();
    let inner_radius = args[5].parse::<u32>().unwrap() / 2;
    let outer_radius = args[6].parse::<u32>().unwrap() / 2;

    if inner_radius >= outer_radius {
        help(&args[0]);
        return Err("Outer radius must be larger than inner radius".into());
    }

    let img = image::io::Reader::open(input_path)?.decode()?;
    let img_width = img.width();
    let img_height = img.height();

    let scale = 2.0;
    let outer_circumference = 2.0 * PI * (outer_radius as f32);
    let width = (outer_circumference * scale) as u32;
    let height = ((outer_radius - inner_radius) as f32 * scale)  as u32;
    let mut image: RgbImage = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let theta = (x as f32) / (width as f32) * 2.0 * PI;
            let magnitude = (outer_radius - inner_radius) as f32 - (y as f32) / scale + inner_radius as f32;
            let delta_x = (theta.cos() * magnitude) as i32;
            let delta_y = (theta.sin() * magnitude) as i32;
            let mut projected_x = (center_x as i32 + delta_x) as u32;
            let mut projected_y = (center_y as i32 + delta_y) as u32;
            projected_x = min(max(0, projected_x), img_width - 1);
            projected_y = min(max(0, projected_y), img_height - 1);
            *image.get_pixel_mut(x, y) = img.get_pixel(projected_x, projected_y).to_rgb();
        }
    }
    image.save(output_path).unwrap();
    Ok(())
}
