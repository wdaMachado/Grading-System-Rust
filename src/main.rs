use std::{io, vec}; 

fn main() {
    funcional();
}

fn funcional() {

    let alunos_string;
    let alunos_numero: u8;
    let mut notas_matematica: Vec<f32> = vec![];
    let mut notas_portugues: Vec<f32> = vec![];
    let mut notas_historia: Vec<f32> = vec![];
    let mut notas_ciencias: Vec<f32> = vec![];
    let mut nome_vetor: Vec<String> = vec![];
    let vec_materia: Vec<String> = vec!["Matemática".to_string(),"Português".to_string(), "História".to_string(), "Ciências".to_string()];


    println!("Olá, bem vindo ao sistema de notas!\n");
    println!("Para começar preciso que me diga quantos alunos existem na turma.\n");

    alunos_string = leitura_dados();
    alunos_numero = to_int(alunos_string);

    println!("Vou precisar que me diga os nomes e quais foram as notas de cada um deles.");

    let mut i = 1;
    let mut nota_string: String;
    let mut nota_float: f32;
    let mut nome_estudante: String = String::new();

    while i <= alunos_numero{
        println!("\nFavor digite o nome do {i}° estudante: ");
        nome_estudante = leitura_dados();
        nome_vetor.push(nome_estudante);
        for materia in vec_materia.iter(){
            println!("Favor digite a nota do {i}° estudante em {materia}: ");
            nota_string = leitura_dados();
            nota_float = to_float(nota_string);
            if materia == "Matemática"{ 
                notas_matematica.push(nota_float);
            } else if materia == "Português"{
                notas_portugues.push(nota_float);
            } else if materia == "História"{
                notas_historia.push(nota_float);
            } else if materia == "Ciências"{
                notas_ciencias.push(nota_float);
            } else {
                println!("Matéria não cadastrada!")
            }
        }


        i+=1;
    }

    let media_matematica: f32 = media(notas_matematica.clone(), vec_materia.clone(),"Matemática".to_string());
    let media_portugues : f32 = media(notas_portugues.clone(), vec_materia.clone(),"Português".to_string());
    let media_historia  : f32 = media(notas_historia.clone(), vec_materia.clone(),"História".to_string());
    let media_ciencias  : f32 = media(notas_ciencias.clone(), vec_materia.clone(),"Ciências".to_string());

    println!("\nA média das notas de todos os alunos em {:<9} é: {media_matematica:.2}",vec_materia[0]);
    println!("A média das notas de todos os alunos em {:<9} é: {media_portugues:.2}",vec_materia[1]);
    println!("A média das notas de todos os alunos em {:<9} é: {media_historia:.2}",vec_materia[2]);
    println!("A média das notas de todos os alunos em {:<9} é: {media_ciencias:.2}",vec_materia[3]);
    println!("-------------------------------------------------------");


    menor_nota(notas_matematica.clone(), nome_vetor.clone(), vec_materia.clone(), "Matemática".to_string());
    menor_nota(notas_portugues.clone(), nome_vetor.clone(), vec_materia.clone(), "Português".to_string());
    menor_nota(notas_historia.clone(), nome_vetor.clone(), vec_materia.clone(), "História".to_string());
    menor_nota(notas_ciencias.clone(), nome_vetor.clone(), vec_materia.clone(), "Ciências".to_string());
    println!("-------------------------------------------------------");

    maior_nota(notas_matematica.clone(), nome_vetor.clone(), vec_materia.clone(), "Matemática".to_string());
    maior_nota(notas_portugues.clone(), nome_vetor.clone(), vec_materia.clone(), "Português".to_string());
    maior_nota(notas_historia.clone(), nome_vetor.clone(), vec_materia.clone(), "História".to_string());
    maior_nota(notas_ciencias.clone(), nome_vetor.clone(), vec_materia.clone(), "Ciências".to_string());
    println!("-------------------------------------------------------");

    situacao_estudante(notas_matematica.clone(),notas_portugues.clone(),notas_historia.clone(),notas_ciencias.clone(), nome_vetor, vec_materia);

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

fn media(vetor: Vec<f32>, vetor_nome_materias: Vec<String>, materia: String)-> f32{
    let mut soma:f32 = 0.0;
    let mut media:f32 = 0.0;
    let mut iterador = 0;
    let mut nome_materia: String = "".to_string();
    for i in vetor_nome_materias{
        if i == materia{
            nome_materia = i;
        } else {
            continue;
        }
    }
    for i in vetor.iter(){
        soma += vetor[iterador];
        iterador +=1;
    }
    media = soma/(vetor.len() as f32);
    media
}

fn menor_nota(vetor_notas: Vec<f32>, vetor_nome: Vec<String>, vetor_nome_materias: Vec<String>,materia: String){
    let mut menor: f32 = 100.00;
    let mut iterador = 0;
    let mut index: usize = 100;
    let mut nome_materia: String = "Não encontrada matéria".to_string();
    for i in vetor_nome_materias{
        if i == materia{
            nome_materia = i;
        } else {
            continue;
        }
    }
    for nota in vetor_notas.iter(){
        if menor > vetor_notas[iterador] {
            menor = vetor_notas[iterador];
            index = iterador; 
        }
        iterador +=1;
    }
    println!("O aluno {:<10} tem a menor nota em {:<10}: {:.2}", vetor_nome[index], nome_materia, menor);
}

fn maior_nota(vetor: Vec<f32>, vetor_nome: Vec<String>, vetor_nome_materias: Vec<String>,materia: String){
    let mut maior: f32 = 0.0;
    let mut iterador = 0;
    let mut index: usize = 0;
    let mut nome_materia: String = "Não encontrada matéria".to_string();
    for i in vetor_nome_materias{
        if i == materia{
            nome_materia = i;
        } else {
            continue;
        }
    }
    for i in vetor.iter(){
        if maior < vetor[iterador] {
            maior = vetor[iterador];
            index = iterador;
        }
        iterador +=1;
    }
    println!("O aluno {:<10} tem a maior nota em {:<10}: {:.2}", vetor_nome[index] , nome_materia,maior);
}
fn situacao_estudante(notas_materia1: Vec<f32>, notas_materia2: Vec<f32>, notas_materia3: Vec<f32>, notas_materia4: Vec<f32>, nome_vetor: Vec<String>, vetor_nome_materias: Vec<String>){ 
    let mut index = 0;
    let situacao_materia1: Vec<String> = situacao(notas_materia1.clone());
    let situacao_materia2: Vec<String> = situacao(notas_materia2.clone());
    let situacao_materia3: Vec<String> = situacao(notas_materia3.clone());
    let situacao_materia4: Vec<String> = situacao(notas_materia4.clone());
    println!("{:>10} |  Notas: {:>10}  |  {:>10} |  {:>10} |  {:>10} |","Alunos",vetor_nome_materias[0],vetor_nome_materias[1],vetor_nome_materias[2],vetor_nome_materias[3]);
    for nome in nome_vetor.iter(){
        println!("{:>10} |         {:<5.3} {:>5.2} | {:>5.3} {:>5.2} | {:>5.3} {:>5.2} | {:>5.3} {:>5.2} |", nome, situacao_materia1[index], notas_materia1[index], situacao_materia2[index], notas_materia2[index], situacao_materia3[index], notas_materia3[index], situacao_materia4[index], notas_materia4[index] );
        index+=1;
    }
}

fn situacao(vetor_notas: Vec<f32>)-> Vec<String>{
    let mut vetor_situacao: Vec<String> = vec![];
    for nota in vetor_notas{
        if nota>= 7.0{
            vetor_situacao.push("Aprovado".to_string())
        } else if nota < 7.0 && nota >= 4.0 {
            vetor_situacao.push("Recuperação".to_string())
        } else if nota< 4.0 {
            vetor_situacao.push("Reprovado".to_string())
        }
    }
    vetor_situacao
}
