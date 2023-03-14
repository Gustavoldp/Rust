fn main() {
    {
        let mut s = String::from("Hello");

        s.push_str(", world!"); // push uma String na String, que é diferente de string literal

        println!("{}", s); //Vai printar `Hello, world!`
    }

    {
        let s1 = String::from("Hello");
        let s2 = s1;
        //s1 sai do escopo para evitar deslocamento duplo do espaço de memória
        println!("{}", s2);
    }

    {
        //Clonamos o conteudo, tamanho e capacidade
        let s1 = String::from("Hello");
        let s2 = s1.clone();

        println!("{}, {}", s1, s2);
    }

    {
        let s = String::from("Hello"); //s está no scopo

        takes_ownership(s); //s pertence a função
        // s não é mais valido a partir daqui

        let x: i32 = 5; // x entra no scopo
        makes_copy(x);//x é copiado para a função
        //então podemos usar x aqui
    }

    {
        let s1 = gives_ownership(); //entra seu retorno para variavel

        let s2 = String::from("Hello");

        let s3 = takes_and_gives_back(s2); // pega o valor do parâmetro e devolve para quem chamou
    }

    {
        //Função não tem ownership do valor, ele apenas calcula a partir do endereço do parâmetro
        let s1 = String::from("Hello");
        let len = calculate_length(&s1);

        println!("{}, {}", s1, len);

    }

    {
        //Só podemos ter uma referência mutável por vez
        let mut s = String::from("Hello");
        {
            let r1 = &mut s;
        } //Aqui r1 sai do scopo então podemos declarar r2
        let r2 = &mut s;
    }

    {
        //Slice
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate(){
                if item == b' '{
                    return i;
                }
            }
            s.len()
        }
    }
}

//scopo
fn func() {
    //s não é valido aqui
    let s = "Hello"; //s é valido daqui pra frente
} 

fn takes_ownership(s: String){
    println!("{}", s)
} 

fn makes_copy(x: i32){
    println!("{}", x)
}

//A função da sua string de retorno para quem chama a função.
fn gives_ownership() -> String{
    let some_string = String::from("Hello");

    some_string
}

//A função pega a string de parametro e devolve para quem à chamou.
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len() //len calcula o tamanho de uma String
}