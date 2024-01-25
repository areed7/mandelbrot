use crate::complex::Complex;

#[derive(Copy, Clone)]
pub struct Pixel{
    pub x : u32,
    pub y : u32
}
#[derive(Copy, Clone)]
pub struct ScreenSize{
    pub width   : usize,
    pub height  : usize
}

#[derive(Copy, Clone)]
pub struct BrotInfo{
    pub center : Complex,
    pub zoom   : f64,
    pub i_max  : u64
}


fn pack_pixel_data(r : u8, g : u8, b : u8, a : u8 ) -> u32 {
    (b as u32)          |
    (g as u32) << 8     |
    (r as u32) << 16    |
    (!a as u32) << 24
}

fn iterate_pixel(pixel : &Pixel, element: &mut u32, screen_size : &ScreenSize, brot_info : &BrotInfo) {
    
    let c : Complex = (Complex::new( ((pixel.x as f64)/(screen_size.width as f64)) - 0.5, ((pixel.y as f64)/(screen_size.height as f64)) - 0.5))*brot_info.zoom + brot_info.center;
    let mut z : Complex = c;
    let mut i: u64 = 0;
    while i < brot_info.i_max {
        z = z*z + c;
        if z.abs() > 2.0 {
            *element = pack_pixel_data(0, ((i+256/2)%255) as u8, (i%255) as u8,255);
            return
        }
        i = i + 1;
    }
    *element = 0;
}


pub fn process_set( pixel_data : &mut Vec<u32> , screen_size : &ScreenSize, brot_info : &BrotInfo ) {
    
    //TODO: This can be massively parallelized. Look into different GPU libraries. 
    
    let mut pix_i: Pixel = Pixel{ x: 0, y: 0};
    for (index, element) in pixel_data.iter_mut().enumerate(){
        let x = index%screen_size.width;
        let y = index/(screen_size.height);
        pix_i.x = x as u32;
        pix_i.y = y as u32;
        let screen_size_copy = *screen_size; // Copy screen_size
        let brot_info_copy = *brot_info; // Copy brot_info

        iterate_pixel(&pix_i, element, &screen_size_copy, &brot_info_copy);
    }

}