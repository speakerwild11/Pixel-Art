use image::{ImageFormat, Rgba, RgbaImage};
use std::vec::Vec;
use std::collections::HashMap;

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}
pub struct PixelImage{
    img: RgbaImage,
    pixels: HashMap<(u32, u32), Vec<(u32, u32)>>,
}

impl PixelImage{

    pub fn new(pixel_resolution: u32, image_size: u32) -> PixelImage{

        //Autocorrect image to a compatible size, per pixel resolution.
        let pixel_area = pixel_resolution * pixel_resolution;
        let mut image_size = image_size;
        let mut image_area = image_size * image_size;
        while image_area % pixel_area != 0{
            image_size += 1;
            image_area = image_size * image_size;
        }

        let mut pixel_coordinates: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::new();
        let rows = image_size/pixel_resolution;
        let pixels_per_row = image_size / pixel_resolution;
        let mut offset_y: u32 = 0;
        let mut current_y: u32 = 0;
        for _row in 0..rows{
            let mut offset_x: u32 = 0;
            let mut current_x: u32 = 0;
            for _p in 0..pixels_per_row{
                let mut pixel: Vec<(u32, u32)> = Vec::new();
                for x in 0..pixel_resolution{
                    for y in 0..pixel_resolution{
                        pixel.push((x as u32 + offset_x as u32, y as u32 + offset_y as u32));
                    }
                }
                pixel_coordinates.insert((current_x, current_y), pixel);
                current_x += 1;
                offset_x += pixel_resolution;
            }
            current_y += 1;
            offset_y += pixel_resolution;
        }
        let img: RgbaImage = image::ImageBuffer::new(image_size as u32, image_size as u32);
        let pi = PixelImage {
            img: img,
            pixels: pixel_coordinates,
        };
        pi
    }

    pub fn pixels(&mut self) -> std::collections::hash_map::Keys<'_, (u32, u32), Vec<(u32, u32)>>{
        self.pixels.keys()
    }
    
    pub fn get_neighbours_all(&mut self, x: u32, y: u32) -> [(u32, u32); 8]{
        let neighbours;
        if x == 0 || y == 0{
            panic!("get_neighbours_all(): Can't get neighbours of x or y coordinates == 0!");
        }else{
            neighbours = [(x-1, y), (x+1, y), (x, y+1), (x, y-1), (x-1, y+1), (x-1, y-1), (x+1, y+1), (x+1, y-1)];
        }
        neighbours
    }

    pub fn get_neighbours_adjascent(&mut self, x: u32, y: u32) -> [(u32, u32); 4]{
        let neighbours;
        if x == 0 || y == 0{
            panic!("get_neighbours_adjascent(): Can't get neighbours of x or y coordinates == 0!");
        }else{
            neighbours = [(x-1, y), (x+1, y), (x, y+1), (x, y-1)];
        }
        neighbours
    }

    pub fn get_neighbours_diagonal(&mut self, x: u32, y: u32) -> [(u32, u32); 4]{
        let neighbours;
        if x == 0 || y == 0{
            panic!("get_neighbours_diagonal(): Can't get neighbours of x or y coordinates == 0!");
        }else{
            neighbours = [(x-1, y+1), (x-1, y-1), (x+1, y+1), (x+1, y-1)];
        }
        neighbours
    }

    pub fn get_neighbours_direction(&mut self, x: u32, y: u32, direction: Direction) -> [(u32, u32); 3]{
        if x == 0 || y == 0{
            panic!("get_neighbours_direction(): Can't get neighbours of x or y coordinates == 0!");
        }
        let neighbours: [(u32, u32); 3];
        match direction{
            Direction::Up => {
                neighbours = [(x-1, y+1), (x, y+1), (x+1, y+1)];
            },
            Direction::Down => {
                neighbours = [(x-1, y-1), (x, y-1), (x+1, y-1)];
            },
            Direction::Right => {
                neighbours = [(x+1, y+1), (x+1, y), (x+1, y-1)];
            },
            Direction::Left => {
                neighbours = [(x-1, y+1), (x-1, y), (x-1, y-1)];
            }
        }
        neighbours
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, rgb: [u8; 3]){
        let pixels = self.pixels.get(&(x, y)).expect("");
        for p in pixels{
            let mut _real_pixel = self.img.get_pixel_mut(p.0, p.1);
            *_real_pixel = Rgba([rgb[0], rgb[1], rgb[2], 255]);
        }
    }

    pub fn save_image(&mut self, path: &str, format: ImageFormat){
        self.img.save_with_format(path, format).expect("Could not save image. Check if path exists.");
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut image: PixelImage = PixelImage::new(10, 800);
        image.draw_pixel(0, 0, [255, 3, 3]);
        image.draw_pixel(1, 1, [255, 3, 3]);
        image.draw_pixel(2, 2, [255, 3, 3]);
        image.draw_pixel(3, 3, [255, 3, 3]);
        image.draw_pixel(4, 4, [255, 3, 3]);
        for neighbour in image.get_neighbours_direction(4, 4, Direction::Right){
            image.draw_pixel(neighbour.0, neighbour.1, [255, 3, 3]);
        }
        image.save_image("testimage.png", image::ImageFormat::Png);
    }
}
