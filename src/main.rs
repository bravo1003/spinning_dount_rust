extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;
use std::time::Duration;

static SCREEN_WDITH: f64 = 1280.0;
static SCREEN_HEIGHT: f64 = 720.0;
static THETA_SPACING: f64 = 0.07;
static PHI_SPACING: f64 = 0.03;
static R1: f64 = 1.0;
static R2: f64 = 2.0;
static K1: f64 = SCREEN_WDITH * K2 * 2.0 / (8.0 * (R1 + R2));
static K2: f64 = 10.0;
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
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

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut a = 1.0;
    let mut b = 1.0;

    // render_donut(&mut a, &mut b, &mut canvas);
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
        render_donut(&mut a, &mut b, &mut canvas);
        a += 0.07;
        b += 0.02;
        ::std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

fn render_donut(a: &mut f64, b: &mut f64, canvas: &mut Canvas<Window>) {
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
            let xp: i32 = (SCREEN_WDITH / 2.0 + K1 * ooz * x) as i32;
            let yp: i32 = (SCREEN_HEIGHT / 2.0 - K1 * ooz * y) as i32;
            let l = 178.0
                * ((cp * ct * sb) - (ca * ct * sp) - (sa * st) + cb * ((ca * st) - (ct * sa * sp)));
            if l > 0.0 {
                canvas.set_draw_color(Color::RGBA(180, 190, 254, l as u8));
                rects.push(Rect::new(xp, yp, 1, 1));
            }
        }
    }
    canvas.fill_rects(&rects).unwrap();
    canvas.present();
}
