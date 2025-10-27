fn main() {
    println!("Hello, world!");
}

#[test]
fn hallo_test() {
    println!("Hallo, Dek!");
    println!("Hallo, Dek!");
    println!("Hallo, Dek!");
}

#[test]
fn variable_test() {
    let name = "Hai Nama Saya Gabriel";
    println!("Hello {}", name);
}

#[test]
fn mutable_test() {
    let mut home = "cikole";
    println!("Lokasi saya di {}", home);
    home = "sukabumi";
    println!("Sekarang saya di {}", home);
}

#[test]
fn shadowing_test() {
    let umur = "17 tahun";
    println!("Umur saya adalah {}", umur);
    let umur = 21;
    println!("Tahun depan umur saya adalah {}", umur);
}
