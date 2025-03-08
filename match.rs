fn main(){
    //key word match
    let fruta: &str = "melancia";
    match fruta {
        "manga" => println!("Sua fruta é uma manga"),
        "banana" => println!("Sua fruta é uma banana"),
        "goiaba" => println!("Sua fruta é uma goiaba"),
        _=> println!("Não conhecço sua fruta"),
    }
}