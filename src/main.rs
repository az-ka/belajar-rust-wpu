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
