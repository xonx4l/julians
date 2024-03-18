use image::{ImageBuffer, Rgb};
use nalgebra::{complex, Normed};


fn main() {
       let width = 800;
       let height = 600;

       let scale_x = 3.0 / width as f64;
       let scale_y = 3.0 / height as f64;

       let mut img = ImageBuffer::new(width, height);

       for (x,y, pixel) in img.enumerate_pixels_mut() {
              *pixel = Rgb ([100, 100, 100]);
       }

       img.save("julia.png");

    }
