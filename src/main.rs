fn nr_prim(x: i32) -> bool {
    if x == 0 || x == 1 {
        false
    } else if x == 2 {
        true
    } else if x % 2 == 0 {
        false
    } else {
        let mut ok = true;
        let mut d = 3;
        while d * d <= x {
            if x % d == 0 {
                ok = false;
                break;
            } else {
                d += 2;
            }
        }
        ok
    }
}
fn nr_coprime(mut x: i32, mut y: i32) -> bool {
    let mut r: i32;
    while y != 0 {
        r = x % y;
        x = y;
        y = r;
    }
    x == 1
}
fn ninetynine_bottlesofbeer(mut x: i32) {
    while x != 0 {
        println!("{} bottles of beer on the wall", x);
        println!("{} bottles of beer.", x);
        println!("Take one down, pass it around,");
        x -= 1;
        if x != 0 {
            println!("{} bottles of beer on the wall", x);
        } else {
            println!("No bottles of beer on the wall.")
        }
        println!(" ");
    }
}
fn main() {
    let mut x = 0;
    while x <= 100 {
        println!("Pentru {}:", x);
        println!("{}", nr_prim(x));
        x += 1;
    }
    println!("Coprime:");
    let mut y = 1;
    x = 1;
    while x <= 100 && y <= 100 {
        println!("Pentru {} si {}:", x, y);
        println!("{}", nr_coprime(x, y));
        x += 1;
        if x == 101 {
            x = 1;
            y += 1;
        }
    }
    ninetynine_bottlesofbeer(100);
}
