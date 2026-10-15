// ex 1

fn add_chars_n(mut s: String, c: char, n: u32) -> String {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
    s
}

// ex 2
fn add_chars_n_modified(s: &mut String, c: char, n: u32) {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
}

// ex 3
fn add_space(s: &mut String, n: u32) {
    let mut i = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, t: &str) {
    s.push_str(t);
}

fn add_integer(s: &mut String, n: i64) {
    if n == 0 {
        s.push('0');
        return;
    }
    let is_negative = n < 0;
    let mut num = n.abs();
    let mut caractere: Vec<char> = Vec::new();
    let mut contor_cifre = 0;
    while num > 0 {
        if contor_cifre > 0 && contor_cifre % 3 == 0 {
            caractere.push('_');
        }
        let cifra = (num % 10) as u8;
        caractere.push((b'0' + cifra) as char);

        num /= 10;
        contor_cifre += 1;
    }
    if is_negative {
        caractere.push('-');
    }

    let mut rezultat = String::with_capacity(caractere.len());
    for &c in caractere.iter().rev() {
        rezultat.push(c);
    }

    s.push_str(&rezultat)
}
fn add_float(s: &mut String, n: f64) {
    let is_negative = n < 0.0;
    let mut num = (n as i64).abs();
    let mut fractionara = n.abs() - num as f64;
    let mut caractere: Vec<char> = Vec::new();
    let mut contor_cifre = 0;
    if num == 0 {
        caractere.push('0');
    } else {
        while num > 0 {
            if contor_cifre > 0 && contor_cifre % 3 == 0 {
                caractere.push('.');
            }
            let cifra = (num % 10) as u8;
            caractere.push((b'0' + cifra) as char);

            num /= 10;
            contor_cifre += 1;
        }
    }
    if is_negative {
        caractere.push('-');
    }
    let mut rezultat = String::with_capacity(caractere.len());
    for &c in caractere.iter().rev() {
        rezultat.push(c);
    }
    
    let mut zecimale = (fractionara * 1_000_000_000.0).round() as i64;

    if zecimale > 0 {
       rezultat.push('.');
       let mut nr_zecimale = 9;
      while zecimale % 10 == 0 {
        zecimale /= 10;
        nr_zecimale -= 1;
    }

    let mut cifre_zecimale: Vec<char> = Vec::new();
    for _ in 0..nr_zecimale {
        let cifra = (zecimale % 10) as u8;
        cifre_zecimale.push((b'0' + cifra) as char);
        zecimale /= 10;
    }

    for &c in cifre_zecimale.iter().rev() {
        rezultat.push(c);
    }
}
    s.push_str(&rezultat);
}
fn main() {
    let mut s = String::from("bac");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_modified(&mut s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
    
    let mut s = String::new();

    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_chars_n_modified(&mut s, '💚', 1);
    add_chars_n_modified(&mut s, '\n', 1);

    add_space(&mut s, 40);
    add_str(&mut s, "RUST.");
    add_chars_n_modified(&mut s, '\n', 2);

    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306_437_968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is");
    add_chars_n_modified(&mut s, '\n', 1);

    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_chars_n_modified(&mut s, '.', 1);

    println!("{}", s);

}
