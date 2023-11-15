use std::io;

pub fn handle_input() -> usize {
    let mut index: String = String::new(); //store the input string in the variable index

    io::stdin()
        .read_line(&mut index) //get input
        .expect("🚨 讀行失敗"); //result,是一種列舉(enums),目的是要編碼錯誤處理資訊,有 Ok 和 Err

    let index: usize = index.trim().parse().expect("🚨 Invalid Input");
    return index;
}
