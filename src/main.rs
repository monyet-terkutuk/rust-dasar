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

#[test]
fn augmented_test() {
    let mut a = 10;
    println!("{}", a);
    a += 10;
    println!("{}", a);
    a *= 10;
    println!("{}", a);
    a %= 3;
    println!("{}", a);
}

#[test]
fn augmented_immutable_test() {
    let a = 10;
    println!("{}", a);
    // a += 10; // ini jadi error
    println!("{}", a);
}

#[test]
fn comparison_test() {
    let a = 10 <= 90;
    println!("hasil variable a : {a}");
}

#[test]
fn char_test() {
    let a: char = 'a';
    let b: char = 'b';
    println!("hasil variable a : {a}");
    println!("{} {}", a, b)
}

#[test]
fn tuple_test() {
    let a = (10, 2.3, false, 'a', "Hai");
    println!("{:?}", a);
    // println!("{}", a); // akan error

    let angka = a.0;
    println!("{}", angka);

    let sapa = a.4;
    println!("{}", sapa);
}

#[test]
fn destructuring_tuple_test() {
    let data = (100, -10, 3.0, 'a');
    println!("{:?}", data);

    let (a, b, c, _) = data;
    println!("{} {} {}", a, b, c);
}
