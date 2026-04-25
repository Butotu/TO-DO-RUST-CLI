use rusqlite::Connection;
use std::io;

fn listar_tarefas(conn: &Connection) {
    let mut stmt = conn
        .prepare("SELECT id, titulo, descricao, feito FROM tarefas")
        .expect("erro em pegar tarefas no banco");

    let tarefas = stmt
        .query_map([], |row| {
            Ok(Tarefa {
                id: Some(row.get(0)?),
                titulo: row.get(1)?,
                descricao: row.get(2)?,
                feito: row.get::<_, i32>(3)? != 0,
            })
        })
        .expect("erro ao listar tarefas no banco");

    for tarefa in tarefas {
        let t = tarefa.expect("erro");

        println!("\n LISTA DE TAREFAS\n");

        println!("ID: {}", t.id.unwrap());
        println!("Título: {}", t.titulo);
        println!("Descrição: {}", t.descricao);
        println!(
            "Status: {}",
            if t.feito { " Concluída" } else { " Pendente" }
        );
        println!("------------------------------");
    }
}
fn retirar_tarefa(conn: &Connection) {
    let mut input = String::new();

    println!("Digite o ID da tarefa que deseja remover:");

    io::stdin().read_line(&mut input).unwrap();

    let id: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID inválido!");
            return println!("auau");
        }
    };

    conn.execute("DELETE FROM tarefas WHERE id = ?1", [id])
        .expect("erro");

    println!("Tarefa removida com sucesso!");
}
fn adicionar_tarefa(conn: &Connection) {
    let mut tarefa = Tarefa {
        id: None,
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

    println!("\n ADICIONAR NOVA TAREFA");

    println!("Digite o título:");
    println!("Digite a descrição:");

    println!(
        "\n Tarefa adicionada:\n {}\n {}\nPendente",
        tarefa.titulo, tarefa.descricao
    );

    conn.execute(
        "INSERT INTO tarefas (titulo, descricao, feito) VALUES (?1, ?2, ?3)",
        (&tarefa.titulo, &tarefa.descricao, tarefa.feito),
    )
    .expect("Erro ao inserir no banco");
}
struct Tarefa {
    id: Option<i32>,
    titulo: String,
    descricao: String,
    feito: bool,
}
fn marcar_como_concluida(conn: &Connection) {
    let mut input = String::new();

    println!("Digite o ID da tarefa que deseja marcar como concluída:");

    io::stdin().read_line(&mut input).unwrap();

    let id: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID inválido!");
            return println!("auau");
        }
    };

    let linhas = conn
        .execute("UPDATE tarefas SET feito = ?1 WHERE id = ?2", (true, id))
        .expect("erro");

    if linhas == 0 {
        println!("Nenhuma tarefa encontrada com esse ID.");
    } else {
        println!("Tarefa marcada como concluída!");
    }
}

fn main() {
    let conn = Connection::open("banco.db").expect("erro ao encontrar o banco");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tarefas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    titulo TEXT NOT NULL,
    descricao TEXT NOT NULL,
    feito BOOLEAN NOT NULL
)",
        [],
    );

    let mut listaTarefas: Vec<Tarefa> = Vec::new();

    loop {
        println!("\n=== GERENCIADOR DE TAREFAS ===");
        println!("1 → Adicionar tarefa");
        println!("2 → Listar tarefas");
        println!("3 → Remover tarefa");
        println!("4 → Marcar como concluída");
        println!("Escolha uma opção:");

        let mut variavel_input = String::new();

        io::stdin().read_line(&mut variavel_input).unwrap();

        match variavel_input.trim() {
            "1" => adicionar_tarefa(&conn),
            "2" => listar_tarefas(&conn),
            "3" => retirar_tarefa(&conn),
            "4" => marcar_como_concluida(&conn),

            _ => println!("outro valor"),
        }
    }
}
