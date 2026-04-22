# 📝 Todo CLI em Rust

Um projeto simples de linha de comando feito em Rust para gerenciar tarefas (to-do list).
Permite adicionar tarefas, listar e marcar como concluídas.

---

## 🚀 Funcionalidades

* ➕ Adicionar tarefas
* 📋 Listar tarefas
* ✅ Marcar tarefas como concluídas

---

## 📦 Estrutura do Projeto

```bash
todo_cli/
├── Cargo.toml
└── src/
    └── main.rs
```

---

## 🧠 Como funciona

O programa roda em loop e espera que o usuário escolha uma opção digitando no terminal.

### Menu principal:

```text
Qual opção?
adicionar
listar
retirar
```

⚠️ **Observação:** No código atual, você usa `"1"`, `"2"` e `"3"` no `match`, mas mostra texto (`adicionar`, `listar`, `retirar`). Isso pode confundir — ideal alinhar depois.

---

## 📌 Estrutura de dados

```rust
struct Tarefa {
    titulo: String,
    descricao: String,
    feito: bool,
}
```

Cada tarefa possui:

* `titulo` → nome da tarefa
* `descricao` → detalhes
* `feito` → status (true = concluída)

---

## 🔧 Funções principais

### ➕ `adicionar_tarefa`

* Solicita título e descrição ao usuário
* Cria uma nova tarefa
* Adiciona na lista

```rust
lista.push(tarefa);
```

---

### 📋 `listar_tarefas`

* Percorre a lista de tarefas
* Exibe título, descrição e status

```rust
for (i) in lista2 {
    println!("{}, {} {}", i.titulo, i.descricao, i.feito)
}
```

⚠️ Pode ser melhorado usando `.iter().enumerate()` para mostrar índice.

---

### ✅ `adicionar_feito`

* Mostra todas as tarefas com índice
* Usuário escolhe uma
* Marca como concluída

```rust
if let Some(tarefa) = lista.get_mut(indice_escolhido) {
    tarefa.feito = true;
}
```

---

## ▶️ Como executar

Na raiz do projeto:

```bash
cargo run
```

---

## 💡 Melhorias futuras

Algumas ideias para evoluir o projeto:

* 🗑️ Remover tarefas
* 💾 Salvar em arquivo (persistência)
* 🎨 Melhorar interface (cores no terminal)
* 🔢 Padronizar menu (usar números ou texto)
* ⚠️ Melhor tratamento de erros

---

## 🛠️ Tecnologias usadas

* Rust (CLI)
* Biblioteca padrão (`std::io`)

---

## 📚 Objetivo

Este projeto foi desenvolvido para praticar:

* Manipulação de `Vec`
* Entrada de dados (`stdin`)
* Estruturas (`struct`)
* Controle de fluxo (`match`, `loop`)
* Mutabilidade e referências em Rust

---

## 👨‍💻 Autor

Feito como projeto de estudo em Rust 🦀
