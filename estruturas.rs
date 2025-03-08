fn main(){
        let meu_carro = Carro{modelo: String::from("Celta"),
        cor: String::from("Preto"),
        valor: 10_000,
    };
    println!("Meu carro é um {} {} e custou {} reais",
    meu_carro.modelo,
    meu_carro.cor,
    meu_carro.valor
    );

}
struct Carro{
    modelo: String,
    cor: String,
    valor:u32,
}