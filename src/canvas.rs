extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;
use sdl2::Sdl;

use std::f64::consts::PI;
use std::time::Duration;

pub const SCREEN_WDITH: f64 = 1000.0;
pub const SCREEN_HEIGHT: f64 = 800.0;
pub const THETA_SPACING: f64 = 0.14;
pub const PHI_SPACING: f64 = 0.05;
pub const R1: f64 = 1.0;
pub const R2: f64 = 2.0;
pub const K1: f64 = SCREEN_WDITH * K2 * 1.0 / (4.0 * (R1 + R2));
pub const K2: f64 = 5.0;

pub fn present_canvas() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_init(&sdl_context)?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut a = 1.0;
    let mut b = 1.0;
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

    Ok(())
}

fn sdl_init(sdl_context: &Sdl) -> Result<Canvas<Window>, String> {
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Canvas Donut",
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
    canvas.set_draw_color(Color::RGB(17, 17, 27));
    canvas.clear();
    canvas.set_blend_mode(BlendMode::Blend);
    let sa: f64 = a.sin();
    let sb: f64 = b.sin();
    let ca: f64 = a.cos();
    let cb: f64 = b.cos();

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
                canvas.set_draw_color(Color::RGBA(250, 179, 135, l.round() as u8));
                canvas.fill_rect(Rect::new(xp, yp, 3, 3)).unwrap();
            }
        }
    }
    canvas.present();
}
