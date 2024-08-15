fn main() {
    // args を取得する
    let args: Vec<String> = std::env::args().collect();
    // https://api.notion.com/v1/${args[1]} を出力する
    println!("https://api.notion.com/v1/{}", args[1]);
}
