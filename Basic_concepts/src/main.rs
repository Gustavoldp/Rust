fn main() {
    {
        /*Erro de compilação, variaveis são imutaveis, adicionando mut se resolve
        mut pode ser útil em casos onde sua variável lida com uma grande estrutura de dados
        onde alterar seu conteudo seja mais rápido que reescreve-lo*/
        let mut x = 5;
        println!("The value of x is {}", x);
        x = 6;
        println!("The value of x is {}", x);
    }

    {
        /*constantes
        valores maximos e minimos são escritos com letras maiusculas
        o _ não faz diferença se não na hora de ler o código
        constantes sempre são imutaveis, não é possível usa mut nelas
        e elas não podem receber valores que dependem de algo em tempo de execução*/
        const MAX_POINTS: u32 = 100_000;
        println!("{}", MAX_POINTS);
    }

    {
        /*Shadowing
        shadowing permite a criação de varias variaveis com o mesmo nome
        de tipos intercambiaveis*/
        let x = 5;
        let x = x + 1;
        let x = x * 2;

        println!("{}", x);

        let spaces = "    ";
        let spaces = spaces.len();
        println!("{}", spaces);
    }
    {
        /*Tuplas 
        Seu tamanho é fixo após declaração, cada posição tem seu tipo
        e deve ser declarado entre parêntesis*/
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let (x, y, z) = tup;
        //podemos desestruturar a tupla para acessar um valor específico
        println!("{}", y);
        //acessar o valor a partir de index é feito com um '.'
        println!("{}", tup.0)
    }
    {
        /*Arrays 
        Seu tamanho também é fixo, todos os elementos devem ser do mesmo tipo
        e são declarados entre cochetes*/
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        //Cria um vetor com 5 números 3
        let b: [i32;5] = [3;5];
        //Pega o primeiro item do Array
        let first = a[0];

    }
    //Funções
    another_function(5);
    plus_one(4);

    //If Else
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {}", number);

    {
        //Retorno de loop pode vir após o break da condição.
        let mut counter = 0;

        let result = loop{
            counter +=1;

            if counter == 10{
                break counter * 2;
            }
        };
        println!("{}", result);
    }

    {
        //um loop com condição pode ser atingido utilizando while
        let mut number = 3;

        while number != 0 {
            println!("{}", number);

            number = number - 1;
        }
        println!("LIFTOFF!!!");
    }

    {
        //Iterando um Array
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("{}", a[index]);

            index = index + 1;
        }
    }

    {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("{}", element);
        }
    }

    {
        //.rev inverte a ordem
        for number in (1..4).rev(){
            println!("{}", number);
        }
        println!("LIFTOFF!!!");
    }



}

fn another_function(x: i32){
    println!("The value of x is: {}", x);
}
//Retorno implícito, se colocar o ; dps do x+1 da erro, deixa de ser uma expressão
fn plus_one (x: i32) -> i32{
    x + 1
}
