use std::time::Duration;

// ASCII consts
pub const ELEMENTS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

pub fn run_ascii() {
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
