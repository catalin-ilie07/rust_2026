fn next_prime(x: u16) -> Option<u16> {
    if x == u16::MAX {
        return None;
    }
    let mut i = x + 1;
    loop {
        let mut d = 2;
        let mut ok = true;
        while d <= i / d {
            if i.is_multiple_of(d) {
                ok = false;
                break;
            }
            d += 1;
        }
        if ok {
            return Some(i);
        }
        if i == u16::MAX {
            return None;
        }
        i += 1;
    }
}

fn checked_addition(x: u32, y: u32) -> u32 {
    if x > u32::MAX - y {
        panic!("nu incape suma pe u32");
    } else {
        x + y
    }
}

fn checked_multiplication(x: u32, y: u32) -> u32 {
    if y != 0 && x > u32::MAX / y {
        panic!("nu incape inmultirea pe u32");
    } else {
        x * y
    }
}

enum Eroare {
    Overflow,
}
fn checked_addition1(x: u32, y: u32) -> Result<u32, Eroare> {
    if x > u32::MAX - y {
        Err(Eroare::Overflow)
    } else {
        Ok(x + y)
    }
}

fn checked_multiplication1(x: u32, y: u32) -> Result<u32, Eroare> {
    if y != 0 && x > u32::MAX / y {
        Err(Eroare::Overflow)
    } else {
        Ok(x * y)
    }
}

fn adunare(a: u32, b: u32, c: u32, d: u32) -> Result<u32, Eroare> {
    let s1 = checked_addition1(a, b)?;
    let s2 = checked_addition1(s1, c)?;
    let s3 = checked_addition1(s2, d)?;
    Ok(s3)
}

fn inmultire(a: u32, b: u32, c: u32) -> Result<u32, Eroare> {
    let s1 = checked_multiplication1(a, b)?;
    let s2 = checked_multiplication1(s1, c)?;
    Ok(s2)
}

enum Erori {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, Erori> {
    if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(Erori::NotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, Erori> {
    if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(Erori::NotLetter)
    }
}

fn print_char(c: char) -> Result<char, Erori> {
    if c as u32 <= 31 || c as u32 == 127 {
        Err(Erori::NotPrintable)
    } else {
        Ok(c)
    }
}

fn char_to_number(c: char) -> Result<u32, Erori> {
    if !c.is_ascii() {
        return Err(Erori::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(Erori::NotDigit);
    }
    Ok(c.to_digit(10).unwrap())
}

fn char_to_hex(c: char) -> Result<u32, Erori> {
    if !c.is_ascii() {
        return Err(Erori::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(Erori::NotBase16Digit);
    }
    Ok(c.to_digit(16).unwrap())
}

fn print_error(e: Erori) {
    match e {
        Erori::NotAscii => println!("Caracterul nu este ASCII"),
        Erori::NotDigit => println!("Caracterul nu este cifra"),
        Erori::NotBase16Digit => println!("Caracterul nu este cifra hexazecimala"),
        Erori::NotLetter => println!("Caracterul nu este litera"),
        Erori::NotPrintable => println!("Caracterul nu este printabil"),
    }
}

// voi face o aplicatie care numara golurile unui fotbalist de la o echipa

enum Echipa {
    Steaua,
    Dinamo,
}

fn nume_echipa(echipa: &Echipa) -> &str {
    match echipa {
        Echipa::Steaua => "Steaua",
        Echipa::Dinamo => "Dinamo",
    }
}

struct Fotbalist {
    echipa: Echipa,
    goluri: u32,
    name: String,
}

fn goluri_fotbalist(f: &Fotbalist) -> Result<u32, &str> {
    if f.goluri > 0 {
        Ok(f.goluri)
    } else {
        Err("Fotbalistul nu are goluri")
    }
}
fn main() {
    // ex 1
    match next_prime(2) {
        Some(n) => println!("{}", n),
        None => println!("nu incape in u16 acest numar prim"),
    }
    match next_prime(551) {
        Some(n) => println!("{}", n),
        None => println!("nu incape in u16 acest numar prim"),
    }
    match next_prime(9512) {
        Some(n) => println!("{}", n),
        None => println!("nu incape in u16 acest numar prim"),
    }
    match next_prime(65535) {
        Some(n) => println!("{}", n),
        None => println!("nu incape in u16 acest numar prim"),
    }

    // ex 2
    println!("adunarea fara result este:{}", checked_addition(2, 3));
    println!(
        "inmultirea fara result este:{}",
        checked_multiplication(2, 3)
    );

    // ex 3
    match adunare(2, 3, 4, 5) {
        Ok(s) => println!("Suma este: {}", s),
        Err(Eroare::Overflow) => println!("Overflow la adunare"),
    }
    match inmultire(2, 3, 4) {
        Ok(s) => println!("Inmultirea este: {}", s),
        Err(Eroare::Overflow) => println!("Overflow la inmultire"),
    }

    // ex 4
    match to_uppercase('a') {
        Ok(c) => println!("litera mare este: {}", c),
        Err(e) => print_error(e),
    }
    match to_lowercase('A') {
        Ok(c) => println!("litera mica este: {}", c),
        Err(e) => print_error(e),
    }
    match print_char('A') {
        Ok(c) => println!("caracterul printabil este: {}", c),
        Err(e) => print_error(e),
    }
    match char_to_number('5') {
        Ok(c) => println!("cifra este: {}", c),
        Err(e) => print_error(e),
    }
    match char_to_hex('F') {
        Ok(c) => println!("cifra hexazecimala este: {}", c),
        Err(e) => print_error(e),
    }
    match char_to_hex(' ') {
        Ok(c) => println!("cifra hexazecimala este: {}", c),
        Err(e) => print_error(e),
    }
    // aplicatia cu fotbalistul
    let f: Fotbalist = Fotbalist {
        echipa: Echipa::Steaua,
        goluri: 5,
        name: String::from("Catalin"),
    };
    let f2: Fotbalist = Fotbalist {
        echipa: Echipa::Dinamo,
        goluri: 0,
        name: String::from("Andrei"),
    };
    match goluri_fotbalist(&f) {
        Ok(goluri) => println!(
            "Fotbalistul {} de la {} are {} goluri",
            f.name,
            nume_echipa(&f.echipa),
            goluri
        ),
        Err(e) => println!("Eroare: {}", e),
    }
    match goluri_fotbalist(&f2) {
        Ok(goluri) => println!(
            "Fotbalistul {} de la {} are {} goluri",
            f2.name,
            nume_echipa(&f2.echipa),
            goluri
        ),
        Err(e) => println!("Eroare: {}", e),
    }
}
