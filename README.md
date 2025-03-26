# Pixel-Art
A simple Rust crate for creating pixel art images with the image crate. Supports additional functionality for fetching pixel neighbours.

## Usage
```
use pixel_art::PixelImage;

//Instantiate image. First parameter is the resolution of each pixel. Second is the resolution of the image. 800 = 800x800
let mut img: PixelImage = PixelImage::new(10, 800);

//Draw a pixel, according to coordinates and RGB values.
img.draw_pixel(2, 3, [255, 2, 2]);

//Get pixel neighbour coordinates in tuple arrays.
img.get_neighbours_all(x, y) -> All neighbours.
img.get_neighbours_adjascent(x, y) -> All directly adjascent neighbours.
img.get_neighbours_diagonal(x, y) -> All diagonally adjascent neighbours.
img.get_neighbours_direction(x, y, Direction) -> All neighbours in a certain direction: Up, Down, Left, Right

//Save image.
img.save_image(path, ImageFormat);
```
