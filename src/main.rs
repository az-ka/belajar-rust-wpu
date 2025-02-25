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

/**
 * Augmented Assignment
 * Rust memiliki augmented assignment seperti +=, -=, *=, /=, %=
 * Yang digunakan untuk mempersingkat penulisan operasi matematika seperti penjumlahan, pengurangan, perkalian, pembagian, modulus
 *
 * number += 1; // number = number + 1;
 * number -= 1; // number = number - 1;
 * number *= 1; // number = number * 1;
 * number /= 1; // number = number / 1;
 * number %= 1; // number = number % 1;
 */
#[test]
fn test_augmented_assignment() {
    let mut number: i32 = 10; // hanya bisa menggunakan mutable variable, karena akan diubah nilainya
    number += 1;
    println!("Number: {}", number); // 11

    number -= 1;
    println!("Number: {}", number); // 10

    number *= 2;
    println!("Number: {}", number); // 20

    number /= 2;
    println!("Number: {}", number); // 10

    number %= 3;
    println!("Number: {}", number); // 1
}

/**
 * Boolean
 * Rust memiliki tipe data boolean yang hanya memiliki dua nilai yaitu true dan false
 *
 * let is_married: bool = false;
 * println!("Is Married: {}", is_married); // Is Married: false
 *
 * let is_married: bool = true;
 * println!("Is Married: {}", is_married); // Is Married: true
 */
#[test]
fn test_boolean() {
    let is_married: bool = false;
    println!("Is Married: {}", is_married);

    let is_married: bool = true;
    println!("Is Married: {}", is_married);
}

/**
 * Comparison Operators
 * Rust memiliki operator perbandingan seperti ==, !=, >, <, >=, <=
 * Operator ini digunakan untuk membandingkan dua nilai dan menghasilkan nilai boolean
 *
 * let is_equal: bool = number1 == number2;
 * println!("Is Equal: {}", is_equal); // false
 */
#[test]
fn test_comparison_operators() {
    let number1: i32 = 10;
    let number2: i32 = 5;

    let is_equal: bool = number1 == number2;
    let is_not_equal: bool = number1 != number2;
    let is_greater_than: bool = number1 > number2;
    let is_less_than: bool = number1 < number2;
    let is_greater_than_or_equal: bool = number1 >= number2;
    let is_less_than_or_equal: bool = number1 <= number2;

    println!("Is Equal: {}", is_equal); // false
    println!("Is Not Equal: {}", is_not_equal); // true
    println!("Is Greater Than: {}", is_greater_than); // true
    println!("Is Less Than: {}", is_less_than); // false
    println!("Is Greater Than or Equal: {}", is_greater_than_or_equal); // true
    println!("Is Less Than or Equal: {}", is_less_than_or_equal); // false
}

/**
 * Operator Boolean
 */

/**
 * && (AND) Operator
 * Rust memiliki operator && (AND) yang digunakan untuk menggabungkan dua nilai boolean
 * Operator ini akan menghasilkan nilai true jika kedua nilai boolean adalah true
 * Dan akan menghasilkan nilai false jika salah satu atau kedua nilai boolean adalah false
 */
#[test]
fn test_operator_and_and() {
    let is_married: bool = true;
    let is_adult: bool = true;

    let is_married_and_adult: bool = is_married && is_adult;
    println!("Is Married and Adult: {}", is_married_and_adult); // true
}

/**
 * OR (||) Operator
 * Rust memiliki operator || (OR) yang digunakan untuk menggabungkan dua nilai boolean
 * Operator ini akan menghasilkan nilai true jika salah satu atau kedua nilai boolean adalah true
 */
#[test]
fn test_operator_or() {
    let is_married: bool = true;
    let is_adult: bool = false;

    let is_married_or_adult: bool = is_married || is_adult;
    println!("Is Married or Adult: {}", is_married_or_adult); // true
}

/**
 * ! (NOT) Operator
 * Rust memiliki operator ! (NOT) yang digunakan untuk membalikkan nilai boolean
 * Operator ini akan menghasilkan nilai true jika nilai boolean adalah false
 * Dan akan menghasilkan nilai false jika nilai boolean adalah true
 */
#[test]
fn test_operator_not() {
    let is_married: bool = true;
    let is_not_married: bool = !is_married;
    println!("Is Not Married: {}", is_not_married); // false
}

/**
 * Char
 * Rust memiliki tipe data char yang digunakan untuk menyimpan karakter unicode
 * Karakter unicode adalah karakter yang lebih kompleks seperti emoji, simbol, dan karakter khusus
 * Karakter hanya bisa digunakan 1 karakter saja, tidak bisa lebih dari 1 karakter
 *
 * let character: char = 'A';
 * println!("Character: {}", character); // A
 */
#[test]
fn test_char() {
    let character: char = 'A';
    println!("Character: {}", character);

    let character: char = '😀';
    println!("Character: {}", character);
}

/**
 * Tuple
 * Rust memiliki tipe data tuple yang digunakan untuk menyimpan beberapa nilai dengan tipe data yang berbeda
 * Jumlah data di tuple sudah ditentukan saat deklarasi dan tidak bisa diubah, karena secara default tuple bersifat immutable
 * Untuk menggunakan tuple kita bisa menggunakan tanda kurung ()
 */
#[test]
fn test_tuple() {
    let person: (&str, i32, bool) = ("Azka", 25, false); // Tuple dengan 3 data. nama, umur, dan status menikah
    println!("Name: {}", person.0); // Mengambil data dari tuple dengan index 0
    println!("Age: {}", person.1); // Mengambil data dari tuple dengan index 1
    println!("Is Married: {}", person.2); // Mengambil data dari tuple dengan index 2

    // :? digunakan untuk menampilkan tuple, struct, enum, dan data structure lainnya
    // :? digunakan untuk debugging
    println!("Person: {:?}", person); // Person: ("Azka", 25, false)

    // cara untuk mendestructuring tuple, yaitu dengan cara mendeklarasikan variable baru dan mengambil data dari tuple
    let (name, age, is_married) = person; // Destructuring tuple
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Married: {}", is_married);
}

/**
 * Tuple Mutable
 * Rust memiliki tipe data tuple yang bersifat mutable, artinya kita bisa mengubah data di dalam tuple
 * Untuk membuat tuple mutable kita perlu menambahkan keyword mut sebelum deklarasi tuple
 */
#[test]
fn test_tuple_mutable() {
    let mut person: (&str, i32, bool) = ("Azka", 25, false); // Tuple dengan 3 data. nama, umur, dan status menikah
    person.2 = true; // Mengubah data tuple dengan index 2
    println!("Person: {:?}", person); // Person: ("Azka", 25, true)
}

/**
 * Tuple Unit atau Tuple Kosong
 * Rust memiliki tipe data tuple unit yang digunakan untuk menyimpan data tanpa nilai, mirip seperti void di bahasa pemrograman lain
 * Tuple unit digunakan ketika kita ingin mengembalikan data dari function yang tidak memiliki nilai
 */
#[test]
fn test_tuple_unit() {
    fn say_hello() -> () {
        println!("Hello!");
    }

    fn process_data(x: i32) -> () {
        if x > 0 {
            println!("Positif");
        } else {
            println!("Negatif atau nol");
        }
    }

    let result = say_hello(); // memanggil function say_hello
    println!("Result: {:?}", result); // tapi result berisi (), karena function tidak mengembalikan nilai

    let result = process_data(5); // memanggil function process_data
    println!("Result: {:?}", result); // tapi result berisi (), karena function tidak mengembalikan nilai
}

/**
 * Array
 * Rust memiliki tipe data array yang digunakan untuk menyimpan beberapa nilai dengan tipe data yang sama, dan jumlah data sudah ditentukan saat deklarasi
 * Array bersifat fixed size, artinya jumlah data di array tidak bisa diubah setelah diinisialisasi
 */
#[test]
fn test_array() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array dengan 5 data bertipe i32
    println!("Numbers: {:?}", numbers); // Numbers: [1, 2, 3, 4, 5]

    let names: [&str; 3] = ["Azka", "Fathurrahman", "Rahman"]; // Array dengan 3 data bertipe &str
    println!("Names: {:?}", names); // Names: ["Azka", "Fathurrahman", "Rahman"]

    // Mengakses data di array
    let first_number: i32 = numbers[0];
    let second_name: &str = names[1];
    println!("First Number: {}", first_number); // 1
    println!("Second Name: {}", second_name); // Fathurrahman
}

/**
 * Array Mutable
 * Rust memiliki tipe data array yang bersifat mutable, artinya kita bisa mengubah data di dalam array
 * Untuk membuat array mutable kita perlu menambahkan keyword mut sebelum deklarasi array
 */
#[test]
fn test_array_mutable() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array dengan 5 data bertipe i32
    numbers[0] = 10; // Mengubah data array dengan index 0
    println!("Numbers: {:?}", numbers); // Numbers: [10, 2, 3, 4, 5]
}

/**
 * Array Len
 * Rust memiliki method len() yang digunakan untuk mendapatkan panjang array
 * Tipe data usize, yang digunakan untuk menyimpan panjang data, akan mengikuti panjang data di komputer kita apa 32 bit atau 64 bit
 */
#[test]
fn test_array_len() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array dengan 5 data bertipe i32
    let len: usize = numbers.len(); // Mendapatkan panjang array
    println!("Len: {}", len); // 5
}

/**
 * Two Dimensional Array
 * Rust memiliki tipe data array dua dimensi yang digunakan untuk menyimpan data dalam bentuk baris dan kolom
 * Array dua dimensi digunakan untuk menyimpan data yang lebih kompleks seperti matriks, papan catur, dan data yang membutuhkan baris dan kolom
 */
#[test]
fn test_two_dimensional_array() {
    let matrix: [[i32; 3]; 4] = [
        [1, 2, 3],    // Baris 1
        [4, 5, 6],    // Baris 2
        [7, 8, 9],    // Baris 3
        [10, 11, 12], // Baris 4
    ]; // Array dua dimensi dengan 3 baris dan 3 kolom bertipe i32
    println!("Matrix: {:?}", matrix); // Matrix: [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]]

    // Mengakses data di array dua dimensi
    let first_row: [i32; 3] = matrix[0]; // Mengambil baris pertama
    let first_number: i32 = matrix[0][0]; // Mengambil data di baris pertama dan kolom pertama
    println!("First Row: {:?}", first_row); // [1, 2, 3]
    println!("First Number: {}", first_number); // 1
}

/**
 * Constant
 * Rust memiliki keyword const yang digunakan untuk membuat constant, artinya nilai dari constant tidak bisa diubah setelah diinisialisasi
 * Constant harus memiliki tipe data, dan tidak bisa menggunakan let
 * Constant biasanya digunakan untuk menyimpan nilai yang tidak berubah seperti PI, PHI, dan nilai lain yang tetap
 * Constant menggunakan huruf besar dan menggunakan underscore (_) untuk memisahkan kata
 */
#[test]
fn test_constant() {
    const PI: f32 = 3.14; // Constant PI dengan tipe data f32
    const RADIUS: i32 = 10; // Constant RADIUS dengan tipe data i32

    const CIRCLE_AREA: f32 = PI * (RADIUS * RADIUS) as f32; // Menghitung luas lingkaran
    const CIRCLE_CIRCUMFERENCE: f32 = 2.0 * PI * RADIUS as f32; // Menghitung keliling lingkaran

    println!("Luas Lingkaran: {}", CIRCLE_AREA); // 314.0
    println!("Keliling Lingkaran: {}", CIRCLE_CIRCUMFERENCE); // 62.8

    const NAME: &str = "Azka Fathurrahman"; // Constant NAME dengan tipe data &str
    println!("Name: {}", NAME); // Azka Fathurrahman
}

/**
 * Scope
 * Rust memiliki konsep scope yang digunakan untuk menentukan dimana variable bisa diakses
 * Variable yang dideklarasikan di dalam scope tertentu hanya bisa diakses di dalam scope tersebut
 * Rust menggunakan curly braces {} untuk menandai scope
 * Variable yang dideklarasikan di luar scope bisa diakses di dalam scope
 */
#[test]
fn test_scope() {
    let name: &str = "Azka"; // Variable name di luar scope

    {
        // Inner scope
        let age: i32 = 25; // Variable age di dalam scope
        println!("Name: {}", name); // Variable name bisa diakses di dalam scope
        println!("Age: {}", age); // Variable age bisa diakses di dalam scope
    }

    // println!("Age: {}", age); // Variable age tidak bisa diakses di luar scope
}
