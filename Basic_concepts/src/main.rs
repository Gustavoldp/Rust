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

    }

}
