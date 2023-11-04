fn main() {
    println!("🎯 This shows some operations in array type");
    let error_codes: [i32; 3] = [400, 404, 500];
    let not_found: i32 = error_codes[1];
    println!("💡 Not found status code: {not_found}");

    let byte: [i32; 8] = [0; 8];
    println!("💡 Last index of [0;8] is: {}", byte[7]);

    let [_, second, ..] = error_codes;
    println!("💡 The 2nd entry is: {second}");
}
