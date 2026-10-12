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
    while t!=0
    {
        s.push(' ');    
        t -=1;
    }
    s
}
fn add_str(mut s: String, t: String) -> String {
    s += &t;
    s
}
fn add_integer(mut s: String, mut t: i32)
{
    let mut ct=t;
    let mut d = String::from("");
    while ct !=0
    {
        s.push(ct%10);
        ct /= 10;
    }
    let mut i=0;
    
}
fn main() {
    let mut s = String::from("");
    let mut t = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_n_version2(&mut t, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
    print!("\n");
    print!("{}", t);
}
