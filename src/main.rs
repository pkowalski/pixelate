extern crate image;

use std::env;
use image::*;

fn load_image(file_path: &str) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let buffer = image::open(file_path).unwrap();
    buffer.to_rgb()
}

fn resize_image(
    image: ImageBuffer<image::Rgb<u8>, Vec<u8>>, ratio: u32
) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let width = image.width();
    let height = image.height();

    if width % ratio > 0 || height % ratio > 0 {
        let remain_width = width % ratio;
        let remain_height = height % ratio;

        let resized = image::imageops::resize(
            &image,
            (width - remain_width) as u32,
            (height - remain_height) as u32,
            image::imageops::FilterType::Nearest,
        );
        return resized;
    }
    image
}

fn get_rbg_averages(image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>, ratio: u32, output_image: &str) {
    let width = image.width();
    let height = image.height();
    let mut buffer = image::ImageBuffer::new(width, height);

    let width_ratio = width / ratio;
    let height_ratio = height / ratio;

    for i in 0..height_ratio {
        for j in 0..width_ratio {
            let avgs = get_average_of_square(
                image,
                j * ratio,
                i * ratio,
                (j + 1) * ratio,
                (i + 1) * ratio,
                ratio
            );
            write_square_to_image(
                &mut buffer,
                j * ratio,
                i * ratio,
                (j + 1) * ratio,
                (i + 1) * ratio,
                avgs,
            );
        }
    }

    buffer.save(output_image).unwrap();
}

fn get_average_of_square(
    image_buff: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    x1: u32,
    y1: u32,
    x: u32,
    y: u32,
    ratio: u32
) -> (u8, u8, u8) {
    let ratio_squared = ratio as i32 * ratio as i32;

    let mut avg_r: i32 = 0;
    let mut avg_g: i32 = 0;
    let mut avg_b: i32 = 0;

    for i in y1..y {
        for j in x1..x {
            let pixel = image_buff.get_pixel(j, i);
            let image::Rgb(data) = *pixel;
            avg_r = avg_r + data[0] as i32;
            avg_g = avg_g + data[1] as i32;
            avg_b = avg_b + data[2] as i32;
        }
    }

    return (
        (avg_r / ratio_squared) as u8,
        (avg_g / ratio_squared) as u8,
        (avg_b / ratio_squared) as u8,
    );
}

fn write_square_to_image(
    image_buff: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    x1: u32,
    y1: u32,
    x: u32,
    y: u32,
    avg: (u8, u8, u8),
) {
    for i in y1..y {
        for j in x1..x {
            let mut pixel = image_buff.get_pixel_mut(j, i);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([avg.0, avg.1, avg.2]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut default_image = "dog.jpg";
    let mut output_image = "out.jpg";
    let ratio = 25;

    if args.len() > 1 {
        default_image = &args[1];
    }
    if args.len() > 2 {
        output_image = &args[2];
    }

    let image = load_image(default_image);
    let resized_image = resize_image(image, ratio);
    get_rbg_averages(&resized_image, ratio, output_image);
}
