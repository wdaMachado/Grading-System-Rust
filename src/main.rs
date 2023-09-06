use std::{io, vec}; 

fn main() {
    funcional();
}

fn funcional() {

    let alunos_string;
    let alunos_numero: u8;
    //let mut alunos_vetor = Vec::new();
    let mut alunos_vetor: Vec<f32> = vec![];
    //let mut nome_vetor = Vec::new();
    let mut nome_vetor: Vec<String> = vec![];
    let vec_materia: Vec<String> = vec!["Matemática".to_string(),"Português".to_string(), "História".to_string(), "Ciências".to_string()];


    println!("Olá, bem vindo ao sistema de notas!\n");
    println!("Para começar preciso que me diga quantos alunos existem na turma.\n");

    alunos_string = leitura_dados();
    alunos_numero = to_int(alunos_string);

    println!("Agora vou precisar que me diga quais foram as notas de cada um deles.");

    let mut i = 1;
    let mut nota_string: String;
    let mut nota_float: f32;
    let mut nome_estudante: String = String::new();

    while i <= alunos_numero{
        println!("\nFavor digite o nome do {i}° estudante: ");

        nome_estudante = leitura_dados();
        nome_vetor.push(nome_estudante);
        for j in vec_materia.iter(){
            println!("Favor digite a nota do {i}° estudante em {j}: ");
            nota_string = leitura_dados();
            nota_float = to_float(nota_string);
            alunos_vetor.push(nota_float);}
        i+=1;
    }

    /*i = 1;
    while i <= alunos_numero{
 
        i+=1;
    }*/

    
    let media: f32 = media(alunos_vetor.clone());
    println!("\nA média das notas de todos os alunos é: {media:.2}");

    menor_nota(alunos_vetor.clone(), nome_vetor.clone());
    //println!("menor nota: {menor_nt}");

    maior_nota(alunos_vetor.clone(), nome_vetor.clone());
    //println!("maior nota: {maior_nt}");

    situacao_estudante(alunos_vetor.clone(), nome_vetor.clone());

}

fn leitura_dados()-> String {
    let mut dado: String = String::new();
    io::stdin().read_line(&mut dado).unwrap();
    return dado.trim().to_string();
}

fn to_int(dado_string: String)-> u8 {
    let dado_int: u8 = dado_string.trim().parse().unwrap();
    return dado_int;
}

fn to_float(dado_string: String)-> f32 {
    let dado_float: f32 = dado_string.trim().parse().unwrap();
    return dado_float;
}

fn media(vetor: Vec<f32>)-> f32{
    let mut soma:f32 = 0.0;
    let mut media:f32 = 0.0;
    let mut iterador = 0;
    for i in vetor.iter(){
        soma += vetor[iterador];
        iterador +=1;
    }
    media = soma/(vetor.len() as f32);
    return media;
}

fn menor_nota(vetor: Vec<f32>, vetor_nome: Vec<String>){
    let mut menor: f32 = 100.00;
    let mut iterador = 0;
    let mut index: usize = 100;
    for i in vetor.iter(){
        if menor > vetor[iterador] {
            menor = vetor[iterador];
            index = iterador; 
        }
        iterador +=1;
    }
    println!("O aluno {} tem a menor nota: {:.2}", vetor_nome[index] , menor);
}

fn maior_nota(vetor: Vec<f32>, vetor_nome: Vec<String>){
    let mut maior: f32 = 0.0;
    let mut iterador = 0;
    let mut index: usize = 0;
    for i in vetor.iter(){
        if maior < vetor[iterador] {
            maior = vetor[iterador];
            index = iterador;
        }
        iterador +=1;
    }
    println!("O aluno {} tem a maior nota: {:.2}", vetor_nome[index] , maior);
}

fn situacao_estudante(vetor: Vec<f32>, nome_vetor: Vec<String>){
    let mut iterador = 0;    
    for i in vetor.iter(){
        if vetor[iterador] >= 7.0 {
            println!("Aluno: {:>10} | nota: {:>3.2} | Status: Aprovado    |", nome_vetor[iterador], vetor[iterador]);
          } else if vetor[iterador] >= 4.0 && vetor[iterador] < 7.0 {
            println!("Aluno: {:>10} | nota: {:>3.2} | Status: Recuperação |", nome_vetor[iterador], vetor[iterador]);
          } else if vetor[iterador] < 4.0 {
            println!("Aluno: {:>10} | nota: {:>3.2} | Status: Reprovado   |", nome_vetor[iterador], vetor[iterador]);
          }
        iterador+=1;        
    }
}
