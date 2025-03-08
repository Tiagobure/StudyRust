fn main(){
        let meu_pet: Animal= Animal::Gato;
        match meu_pet{
            Animal::Cachorro => println!("O seu animal é um cachorro"),
            Animal::Gato => println!("O seu animal é um gato"),
            Animal::Calopsita => println!("O seu animal é um passaro"),
            _=> println!("O seu animal é desconhecido"),
        }
}

enum Animal{
    Cachorro,
    Gato,
    Calopsita,
}
// :: dentro do enum pegue um valor