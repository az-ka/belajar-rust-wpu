/**
 * Cara menjalankan rust:
 * cargo run
 */
fn main() {
    println!("Hello, world!");
}

/**
 * Cara menjalankan semua unit test:
 * cargo test
 * 
 * Cara menjalankan unit test:
 * cargo test {nama_fungsi} -- --exact
 * exact: hanya menjalankan test yang sesuai dengan nama fungsi
 * 
 * Cara menjalankan unit test dan menampilkan output println:
 * cargo test {nama_fungsi} -- --nocapture
 * nocapture: menampilkan output dari println
 */
#[test]
fn hello_test() {
    println!("Hello, test!");
}

/**
 * Variable let bersifat immutable, tidak bisa diubah
 * 
 * {} digunakan untuk menampilkan variable atau template dari variable
 * ex: println!("Name: {}", name); // Name: Azka
*/
#[test]
fn test_variable_immutable() {
    let name: &str = "Azka";
    println!("Name: {}", name);
}

/**
 * Variable let bersifat mutable, bisa diubah
 * Rust juga menyediakan keyword let mut untuk membuat variable mutable
 * 
 * let mut name: &str = "Azka";
 * name = "Azka Fathurrahman";
 * println!("Name: {}", name); // Name: Azka Fathurrahman
 * 
 * Dan diubah  
 * name = "Azka Fathurrahman";
 * println!("Name: {}", name); // Name: Azka Fathurrahman
 */
#[test]
fn test_variable_mutable() {
    let mut name: &str = "Azka";
    print!("Name: {}", name); // Name: Azka
    
    name = "Azka Fathurrahman";
    println!("Name: {}", name); // Name: Azka Fathurrahman
}

/**
 * Rust memiliki static typing, artinya setiap variable harus memiliki tipe data
 * Dan tidak bisa diubah setelah diinisialisasi, misalnya variable name yang bertipe &str tidak bisa diubah menjadi i32
 * 
 * let name: &str = "Azka";
 * let age: i32 = 25;
 * let is_married: bool = false;
 * let height: f32 = 170.5;
 * let weight: f32 = 60.5;
 */
#[test]
fn test_static_tryping() {
    let name: &str = "Azka";
    let age: i32 = 25;
    let is_married: bool = false;
    let height: f32 = 170.5;
    let weight: f32 = 60.5;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Married: {}", is_married);
    println!("Height: {}", height);
    println!("Weight: {}", weight);
}

/**
 * Rust memiliki shadowing, artinya kita bisa mendeklarasikan variable dengan nama yang sama
 * Contohnya kedua variable name dibawah ini memiliki nama yang sama, tetapi memiliki nilai yang berbeda
 * Maka variable yang terakhir akan menggantikan variable yang sebelumnya
 * Jadi ketika menambahkan code dibawah variable kedua maka variable yang pertama akan di shadowing atau tertimpa oleh variable yang kedua
 * Tapi kalo misalnya menambah code diatas variable kedua maka akan menggunakan variable yang pertama
 * 
 * let name: &str = "Azka";
 * println!("Name: {}", name); // Name: Azka
 * 
 * let name: &str = "Azka Fathurrahman";
 * println!("Name: {}", name); // Name: Azka Fathurrahman
 */
#[test]
fn test_shadowing() {
    let name: &str = "Azka"; // Variable pertama
    println!("Name: {}", name); // Name: Azka

    let name: &str = "Azka Fathurrahman"; // Variable kedua dengan nama yang sama
    println!("Name: {}", name); // Name: Azka Fathurrahman
}

/**
 * DATA TYPES
 */

 /**
  * Integer & Floating Point
  * Di rust ada beberapa tipe data integer dan floating point
    * i8, i16, i32, i64, i128, isize - signed integer (bilangan bulat dengan tanda) bisa positif dan negatif
    * u8, u16, u32, u64, u128, usize - unsigned integer (bilangan bulat tanpa tanda) hanya positif
    * f32, f64 - floating point (bilangan desimal)

  * Misalnya tidak menambahkan tipe data maka secara otomatis integer akan menjadi i32 dan floating point akan menjadi f64
  * Jika ingin merubah kita perlu explicit menambahkan tipe data seperti contoh dibawah ini menggunakan : {tipe_data}
  */
#[test]
fn test_number_float() {
    let number: i8 = 10; // i8 adalah tipe data integer
    println!("Number: {}", number);

    let number: f32 = 10.5; // f32 adalah tipe data floating point
    println!("Number: {}", number);

    let number: f64 = 10.5; // f64 adalah tipe data floating point
    println!("Number: {}", number);
}

/**
 * Konversi Tipe Data
 * Rush bisa melakukan konversi tipe data dengan menggunakan keyword as
 * 
 * Misalnya number: i8 = 10; // i8 adalah tipe data integer
 * Maka kita bisa mengkonversi tipe data tersebut menjadi tipe data lain seperti i16, i32, i64, i128, isize
 * Jika tipe data yang dituju lebih kecil maka akan terjadi overflow atau kelebihan kapasitas
 */
#[test]
fn test_convert_data_type() {
    // Contoh 1
    let number: i16 = 1000; // Gunakan nilai yang sesuai range i16
    let number_converted: i32 = number as i32;
    println!("Number: {}", number_converted);

    // Contoh 2
    let small_number: i8 = 100; // Nilai dalam range i8
    let bigger_number: i16 = small_number as i16;
    println!("Converted number: {}", bigger_number);
}

/**
 * Numeric Operations
 * Rust memiliki beberapa operator matematika seperti penjumlahan, pengurangan, perkalian, pembagian, modulus
 * 
 * Penjumlahan: +
 * Pengurangan: -
 * Perkalian: *
 * Pembagian: /
 * Modulus: %
 */
#[test]
fn test_numeric_operations() {
    let number1: i32 = 10;
    let number2: i32 = 5;

    let sum: i32 = number1 + number2;
    let sub: i32 = number1 - number2;
    let mul: i32 = number1 * number2;
    let div: i32 = number1 / number2;
    let modu: i32 = number1 % number2;

    println!("Sum: {}", sum); // 15
    println!("Sub: {}", sub); // 5
    println!("Mul: {}", mul); // 50
    println!("Div: {}", div); // 2
    println!("Modu: {}", modu); // 0
}
