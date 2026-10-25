fn add_chars_n(mut s: String, t: char, mut x: i32) -> String {
    while x > 0 {
        s.push(t);
        x -= 1;
    }
    s
}

fn add_chars_n_version2(s: &mut String, t: char, mut x: i32) {
    while x > 0 {
        s.push(t);
        x -= 1;
    }
}
fn add_space(mut s: String, mut t: i32) -> String {
    while t > 0 {
        s.push(' ');
        t -= 1;
    }
    s
}
fn add_str(mut s: String, t: &str) -> String {
    s += t;
    s
}
fn add_integer(mut s: String, mut t: i32) -> String {
    if t < 0 {
        s.push('-');
        t = t.abs();
    }
    let mut ct = 0;
    let mut nr = 0;

    while t != 0 {
        ct = ct * 10 + t % 10;
        t /= 10;
    }
    while ct != 0 {
        let d = ct % 10;
        s.push((d as u8 + b'0') as char);
        nr += 1;
        if nr % 3 == 0 && ct > 9 {
            s.push(' ');
        }
        ct /= 10;
    }
    s
}

fn add_float(mut s: String, t: f64) -> String {
    let mut h = (t as i64).abs();
    let mut p: f64;
    let g = h;
    let mut ogl: i64 = 0;

    while h != 0 {
        ogl = ogl * 10 + h % 10;
        h /= 10;
    }
    while ogl != 0 {
        let y = ogl % 10;
        s.push((y as u8 + b'0') as char);
        ogl /= 10;
    }

    s.push('.');
    if t < 0.0 {
        p = t.abs() - (g as f64);
    } else {
        p = t - (g as f64);
    }
    p *= 1_000_000_000.0;

    let mut j: i64 = p as i64;
    while j > 0 && j % 10 == 0 {
        j /= 10;
    }
    ogl = 0;
    while j != 0 {
        ogl = ogl * 10 + j % 10;
        j /= 10;
    }
    while ogl != 0 {
        let y = ogl % 10;
        s.push((y as u8 + b'0') as char);
        ogl /= 10;
    }
    s
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();
    let mut i = 0;

    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_n_version2(&mut t, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
    println!("{}", t);
    s = String::from("dada");
    s = add_integer(s, 1234);
    println!("{}", s);
    s = add_space(s, 3);
    println!("{}", s);
    s = add_str(s, "testare");
    println!("{}", s);
    let t_float = String::from("float: ");
    println!("{}", add_float(t_float, 1234.56));
}
