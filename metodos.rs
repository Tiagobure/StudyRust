

fn main(){
    let minha_casa = Casa{
            largura: 6,
            comprimento: 35,
    };
    println!("a minha casa tem {} metros quadrados",minha_casa.area());
}
impl Casa{
    fn area(&self) -> u32{
        self.largura * self.comprimento
    }
}
struct Casa{
    largura: u32,
    comprimento: u32,
}