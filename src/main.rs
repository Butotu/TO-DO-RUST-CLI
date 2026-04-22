use std::io;
fn listar_tarefas(lista: &mut Vec<Tarefa>) {
    let mut lista2 = lista;

    for (i) in lista2 {
        println!("{}, {} {}", i.titulo, i.descricao, i.feito)
    }
}
fn retirar_tarefa(lista: &mut Vec<Tarefa>) {
    let mut lista2 = lista;

    let mut contador = 0;

    for (i) in lista2.iter() {
        contador = contador + 1;
        println!("{}, {} {}", i.titulo, i.descricao, i.feito)
    }
    println!("Digite o titulo que você deseja retirar:");
    let mut resposta1 = String::new();
    io::stdin().read_line(&mut resposta1).unwrap();

    if let Some(i) = lista2.iter().position(|i| i.titulo == resposta1.trim()) {
        lista2.remove(i);
    }
}
fn adicionar_tarefa(lista: &mut Vec<Tarefa>) {
    let mut tarefa = Tarefa {
        titulo: String::new(),
        descricao: String::new(),
        feito: false,
    };

    println!("Titulo:");
    let mut resposta1 = String::new();
    io::stdin().read_line(&mut resposta1).unwrap();

    println!("Descrição:");
    let mut resposta2 = String::new();
    io::stdin().read_line(&mut resposta2).unwrap();

    tarefa.titulo = resposta1.trim().to_string();
    tarefa.descricao = resposta2.trim().to_string();

    println!(
        "Adicionado: {} - {} - {}",
        tarefa.titulo, tarefa.descricao, tarefa.feito
    );

    lista.push(tarefa);
}
struct Tarefa {
    titulo: String,
    descricao: String,
    feito: bool,
}
fn adicionar_feito(lista: &mut Vec<Tarefa>) {
    println!("Qual tarefa quer marcar o feito?");

    for (i, tarefa) in lista.iter().enumerate() {
        println!("{}: {}, feito: {}", i, tarefa.titulo, tarefa.feito);
    }

    let mut valor = String::new();
    io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler linha");

    let indice_escolhido: usize = match valor.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número válido!");
            return;
        }
    };

    if let Some(tarefa) = lista.get_mut(indice_escolhido) {
        tarefa.feito = true;
        println!("Tarefa '{}' marcada como concluída!", tarefa.titulo);
    } else {
        println!("Índice {} não encontrado na lista.", indice_escolhido);
    }
}
fn main() {
    let mut listaTarefas: Vec<Tarefa> = Vec::new();

    loop {
        println!("Qual opção?");
        println!("adicionar");
        println!("listar");
        println!("retirar");

        let mut variavel_input = String::new();

        io::stdin().read_line(&mut variavel_input).unwrap();

        match variavel_input.trim() {
            "1" => adicionar_tarefa(&mut listaTarefas),
            "2" => listar_tarefas(&mut listaTarefas),
            "3" => retirar_tarefa(&mut listaTarefas),

            _ => println!("outro valor"),
        }
    }
}
