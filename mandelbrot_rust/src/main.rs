mod brot;
mod complex;

use std::sync::{Arc, Mutex};
use std::thread;


use crate::brot::{BrotInfo, ScreenSize};
use complex::Complex;

use minifb::{Key, Window, WindowOptions, KeyRepeat};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn controls(window: &Window, brot_info: &mut BrotInfo) {
    //TODO: Implement timing on controls. Currently changes speed based on frame rate of render.
    //Brot info structure could posssibly store time last changed plus some offset, then when cur time exceeds allow change.
    //TODO: Implement text rendering for the current brot info zoom, iterations and center.
    //TODO: Implement color controls
    //TODO: Implement screenshot controls.
    if window.is_key_pressed(Key::D, KeyRepeat::No) {
        brot_info.center.real += 0.1*brot_info.zoom;
    }
    if window.is_key_pressed(Key::A, KeyRepeat::No) {
        brot_info.center.real -= 0.1*brot_info.zoom;
    }
    if window.is_key_pressed(Key::S, KeyRepeat::No) {
        brot_info.center.imag += 0.1*brot_info.zoom;
    }
    if window.is_key_pressed(Key::W, KeyRepeat::No) {
        brot_info.center.imag -= 0.1*brot_info.zoom;
    }
    if window.is_key_pressed(Key::E, KeyRepeat::No) {
        brot_info.zoom *= 0.5;
    }
    if window.is_key_pressed(Key::Q, KeyRepeat::No) {
        brot_info.zoom /= 0.5;
    }

    if window.is_key_pressed(Key::C, KeyRepeat::No) {
        brot_info.i_max += 10;
    }
    if window.is_key_pressed(Key::V, KeyRepeat::No) {
        brot_info.i_max -= 10;
        if brot_info.i_max < 10 {
            brot_info.i_max = 10;
        }
    }

    if window.is_key_pressed(Key::R, KeyRepeat::No) {
        brot_info.i_max = 100;
        brot_info.center.real = 0.0;
        brot_info.center.imag = 0.0;
        brot_info.zoom = 2.0;
    }
    
    
}

fn main() {
    
    //Manelbrot Info.
    let screen_size: ScreenSize = ScreenSize { width: WIDTH, height: HEIGHT };
    let brot_info: Arc<Mutex<BrotInfo>> = Arc::new(Mutex::new(BrotInfo { center: Complex::new(0.0, 0.0), zoom: 2.0, i_max: 200 }));
    
    //Create Two Buffers. One for front end and one for back. Each should be wrapped in an ARC mutex
    let buffer_a: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![0; WIDTH*HEIGHT]));
    let buffer_b: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![0; WIDTH*HEIGHT]));
    let is_draw_finished: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    
    let is_program_killed = Arc::new(Mutex::new(false));
    
    let mut window = Window::new(
        "Mandelbrot",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    
    //Data Manager thread. Handles which buffer is currently front or back end.
    //Splits work apart into thread pool.
    let buffer_a_clone_worker = Arc::clone(&buffer_a);
    let buffer_b_clone_worker = Arc::clone(&buffer_b);
    let is_draw_finished_clone_worker = Arc::clone( &is_draw_finished );
    
    let brot_info_clone_worker = Arc::clone(&brot_info);

    let is_program_killed_clone_worker = Arc::clone(&is_program_killed);
    let process_thread = thread::spawn( move || {

        


        let mut is_a_current_draw_buffer = true;
        loop {

            //Check if program has been killed
            {
                let ipk = is_program_killed_clone_worker.lock().unwrap();
                if *ipk {
                    break
                }
            }


            //If the buffers haven't finished swapping then continue.
            {
                let swap_buffers = is_draw_finished_clone_worker.lock().unwrap();
                if *swap_buffers {
                    continue
                }
            }
            
            //TODO: Most of this can be shortened into a statement.
            if is_a_current_draw_buffer {
                let mut buf = buffer_a_clone_worker.lock().unwrap();
                //let b_i = brot_info_clone_worker.lock().unwrap();
                //Pass in a copy of the data. Once the brot_info data is set then there's no need to lock it during rendering.
                let brot_info_copy : BrotInfo;
                {
                    brot_info_copy = *(brot_info_clone_worker.lock().unwrap());
                }
                brot::process_set(&mut buf, &screen_size, &brot_info_copy);
            } else {
                let mut buf = buffer_b_clone_worker.lock().unwrap();
                //let b_i = brot_info_clone_worker.lock().unwrap();
                //Pass in a copy of the data. Once the brot_info data is set then there's no need to lock it during rendering.
                let brot_info_copy : BrotInfo;
                {
                    brot_info_copy = *(brot_info_clone_worker.lock().unwrap());
                }
                brot::process_set(&mut buf, &screen_size, &brot_info_copy);
            }

            {
                //Swap ready.
                let mut is_draw_finished_unlock = is_draw_finished_clone_worker.lock().unwrap();
                *is_draw_finished_unlock = true;
                is_a_current_draw_buffer = !is_a_current_draw_buffer;
            }
            
        }
    });


    
    let buffer_a_clone = Arc::clone(&buffer_a);
    let buffer_b_clone = Arc::clone(&buffer_b);
    let is_draw_finished_clone = Arc::clone(&is_draw_finished);
    let brot_info_clone = Arc::clone(&brot_info);

    let mut is_a_current_screen_buffer: bool = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        {
            let mut b_i = brot_info_clone.lock().unwrap();
            controls(&window, &mut b_i);
        }

        {
            let mut swap_buffers = is_draw_finished_clone.lock().unwrap();
            if *swap_buffers {
                *swap_buffers = false;
                is_a_current_screen_buffer = !is_a_current_screen_buffer;
            }
        }

        if is_a_current_screen_buffer {
            let buf = buffer_a_clone.lock().unwrap();
            window.update_with_buffer( &*(*buf), WIDTH, HEIGHT).unwrap();
        } else {
            let buf = buffer_b_clone.lock().unwrap();
            window.update_with_buffer( &*(*buf), WIDTH, HEIGHT).unwrap();
        }


    }

    {
        let mut ipk = is_program_killed.lock().unwrap();
        *ipk = true;
    }

    process_thread.join().unwrap();
}
