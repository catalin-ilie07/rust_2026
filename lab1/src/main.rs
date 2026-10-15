fn prim(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn coprime(mut a: u32, mut b: u32) -> bool {
    if a == 0 || b == 0 {
        return false;
    }

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a == 1
}

fn bottles(mut n: u32) {
    while n != 0 {
        println!("{} bottles of beer on the wall", n);
        println!("{} bottles of beer.", n);
        println!("Take one down, pass it around,");
        n -= 1;
    }
    if n == 0 {
        println!("No bottles of beer on the wall.")
    }
}
fn main() {
    let mut i = 0;
    while i <= 100 {
        if prim(i) {
            println!("Prim: {}", i);
        }
        if i <= 99 && coprime(i, i + 1) {
            println!("perechea {} si {} este coprima", i, i + 1);
        }
        i += 1;
    }
    bottles(5);
}
