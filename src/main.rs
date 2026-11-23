fn main() {
    println!("Hello, world!");
    println!("Hello, Lewi!");
    println!("Hello, Messi!");
}

#[test]
fn hello_test() {
    println!("Hello, Test");
}

#[test]
fn test_variable(){
    let name = "Lewi Balut Maropo";
    // name = "Budi"; (tidak bisa) (imutable)
    println!("Hello {}", name);
}

#[test]
fn test_mutable(){
    let mut name = "Lewi Balut Maropo";
    println!("Hello {}", name);

    name = "Leo Messi";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Lewi Balut Maropo";
    println!("Hello {}", name);

    // name = 10; (tidak bisa)
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Lewi Balut Maropo";
    println!("Hello {}", name);

    let name = 10;
    print!("Hello {}", name);

    print!("Hah kok kamu angka {}", name);
}

#[test]
fn comment() {
    let name = "Lewi Balut Maropo";
    println!("Hello {}", name);
    //ini adalah komentar menggunakan slash dua kali
    /*
        ini adalah komentar kalau mau banyak baris menggunakan awal slash bintang dan diakhiri bintang slash 
     */
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("umurmu: {}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = a as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e)
}

#[test]
fn numeric_operator() {
    let a = 12;
    let b = 10;
    let c = a + b;
    println!("{}", c);
    let d = a - b;
    println!("{}", d);
    let e = a * b;
    println!("{}", e);
    let f = a / b;
    println!("{}", f);
    let g = a % b;
    println!("{}", g);
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a += 12;
    println!("{}", a);

    a -= 22;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;
    
    let result: bool = a > b;
    println!("{}", result);

    let d = 20;
    let e = 10;

    let hasil: bool = d > e;
    println!("{}", hasil);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;
    
    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus)
}

#[test]
fn chart_type() {
    let char1: char = 'a';
    let char2: char = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    //akses tuple
    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("{} {} {}", a, b, c);
}

#[test]
fn destructuring_tuple() {
    let data: (i32, f64, bool) = (20, 20.5, false);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}

#[test]
fn mutable_tuple() {
    let mut data: (i32, f64, bool) = (30, 30.5, false);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 10;
    data.1 = 100.5;
    data.2 = true;
    println!("{:?}", data);
}

#[test]
//empty tuple
fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
}

#[test]
fn array_access() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[4];
    println!("{} {}", a, b);
}