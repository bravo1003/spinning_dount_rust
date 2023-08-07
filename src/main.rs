extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use std::f64::consts::PI;
use std::io;
use std::time::Duration;

// Canvas consts
static SCREEN_WDITH: f64 = 1280.0;
static SCREEN_HEIGHT: f64 = 720.0;
static THETA_SPACING: f64 = 0.07;
static PHI_SPACING: f64 = 0.03;
static R1: f64 = 1.0;
static R2: f64 = 2.0;
static K1: f64 = SCREEN_WDITH * K2 * 2.0 / (8.0 * (R1 + R2));
static K2: f64 = 10.0;

// ASCII consts
pub const ELEMENTS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

pub fn main() -> Result<(), String> {
    let mut user_input = String::new();
    let mut choice: u8;
    'selecting: loop {
        print!("Select your flavor of donut!\n1. ASCII\n2. Canvas\n0. Quit");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line"); // We get `Stdin` here.
        choice = user_input.trim().parse().expect("Failed to parse input");
        match choice {
            1 => break 'selecting,
            2 => break 'selecting,
            0 => return Ok(()),
            _ => println!(
                "Error choice ({}), number needs to be between 1 and 2",
                choice
            ),
        };
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("You've selected ({}) ", choice);

    let mut a = 1.0;
    let mut b = 1.0;

    if choice == 2 {
        let sdl_context = sdl2::init()?;
        let mut canvas = sdl_init(&sdl_context)?;
        let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            render_canvas(&mut a, &mut b, &mut canvas);
            a += 0.07;
            b += 0.02;
            ::std::thread::sleep(Duration::from_millis(50));
        }
    } else {
        let mut sin_a: i32 = 1024;
        let mut cos_a: i32 = 0;
        let mut sin_b: i32 = 1024;
        let mut cos_b: i32 = 0;
        loop {
            render_ascii(sin_a, sin_b, cos_a, cos_b);
            rotate(5, 7, &mut cos_a, &mut sin_a);
            rotate(5, 8, &mut cos_b, &mut sin_b);
            ::std::thread::sleep(Duration::from_millis(20));

            print!("\x1b[23A");
        }
    }

    Ok(())
}

fn sdl_init(sdl_context: &Sdl) -> Result<Canvas<Window>, String> {
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            SCREEN_WDITH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(e) => return Err(e.to_string()),
    };

    Ok(canvas)
}

fn render_canvas(a: &mut f64, b: &mut f64, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(Color::GREEN);
    let sa: f64 = a.sin();
    let sb: f64 = b.sin();
    let ca: f64 = a.cos();
    let cb: f64 = b.cos();

    let mut rects: Vec<Rect> = Vec::new();

    let mut theta: f64 = 0.0;
    while theta < (2.0 * PI) {
        let ct = theta.cos();
        let st = theta.sin();
        theta += THETA_SPACING;
        let mut phi: f64 = 0.0;
        while phi < (2.0 * PI) {
            let cp = phi.cos();
            let sp = phi.sin();
            phi += PHI_SPACING;

            let ox = R2 + (R1 * ct);
            let oy = R1 * st;

            let x = ox * ((cb * cp) + (sa * sb * sp)) - (oy * ca * sb);
            let y = ox * ((sb * cp) - (sa * cb * sp)) + (oy * ca * cb);
            let z = K2 + (ca * ox * sp) + (oy * sa);

            let ooz = 1.0 / z;
            let xp: i32 = (SCREEN_WDITH / 2.0 + K1 * ooz * x).round() as i32;
            let yp: i32 = (SCREEN_HEIGHT / 2.0 - K1 * ooz * y).round() as i32;
            let l = 178.0
                * ((cp * ct * sb) - (ca * ct * sp) - (sa * st) + cb * ((ca * st) - (ct * sa * sp)));
            if l > 0.0 {
                canvas.set_draw_color(Color::RGBA(180, 190, 254, l.round() as u8));
                rects.push(Rect::new(xp, yp, 2, 2));
            }
        }
    }
    canvas.fill_rects(&rects).unwrap();
    canvas.present();
}

fn rotate(mul: i32, shift: i32, x: &mut i32, y: &mut i32) {
    let mut tmp = *x;
    *x -= (mul * (*y)) >> shift;
    *y += (mul * tmp) >> shift;
    tmp = (3145728 - (*x) * (*x) - (*y) * (*y)) >> 11;
    *x = ((*x) * tmp) >> 10;
    *y = ((*y) * tmp) >> 10;
}

fn render_ascii(sin_a: i32, sin_b: i32, cos_a: i32, cos_b: i32) {
    let mut b_elements: Vec<i8> = vec![32; 1760];
    let mut z_elements: Vec<i8> = vec![127; 1760];
    let mut sj = 0;
    let mut cj = 1024;
    let mut theta = 0;
    while theta < 90 {
        theta += 1;
        let mut si = 0;
        let mut ci = 1024;
        let mut phi = 0;
        while phi < 324 {
            phi += 1;
            let r1 = 1;
            let r2 = 2048;
            let k2 = 5120 * 1024;
            let x0 = r1 * cj + r2;
            let x1 = (ci * x0) >> 10;
            let x2 = (cos_a * sj) >> 10;
            let x3 = (si * x0) >> 10;
            let x4 = r1 * x2 - ((sin_a * x3) >> 10);
            let x5 = (sin_a * sj) >> 10;
            let x6 = k2 + r1 * 1024 * x5 + cos_a * x3;
            let x7 = (cj * si) >> 10;
            let x = 40 + 30 * (cos_b * x1 - sin_b * x4) / x6;
            let y = 12 + 15 * (cos_b * x4 + sin_b * x1) / x6;
            let n = ((-cos_a * x7 - cos_b * ((-sin_a * x7 >> 10) + x2) - ci * (cj * sin_b >> 10)
                >> 10)
                - x5)
                >> 7;
            let o = usize::try_from(x + 80 * y).unwrap();
            let zz = ((x6 - k2) >> 15) as i8;
            if 22 > y && y > 0 && x > 0 && 80 > x && zz < z_elements[o] {
                z_elements[o] = zz;
                match n {
                    n if n > 0 => b_elements[o] = ELEMENTS[n as usize] as i8,
                    _ => b_elements[o] = ELEMENTS[0] as i8,
                };
            }
            rotate(5, 8, &mut ci, &mut si);
        }
        rotate(9, 7, &mut cj, &mut sj);
    }
    let mut k = 0;
    while k < 1761 {
        if k % 80 > 0 {
            print!("{}", u8::try_from(b_elements[k]).unwrap() as char);
        } else {
            print!("{}", 10 as char);
        }
        k += 1;
    }
}
