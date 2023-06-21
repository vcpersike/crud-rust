use crate::application::user_service::UserService;
use crate::infra::repository::UserRepository;
use std::io;

mod application;
mod infra;

fn cadastrar_usuario(user_service: &UserService) {
    println!("Digite o nome do usuário:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Falha ao ler a entrada");

    println!("Digite o e-mail do usuário:");
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Falha ao ler a entrada");

    let result = user_service.create_user(name.trim(), email.trim());
    match result {
        Ok(_) => println!("Usuário cadastrado com sucesso!"),
        Err(err) => eprintln!("Erro ao cadastrar usuário: {}", err),
    }
}

fn listar_usuarios(user_service: &UserService) {
    let users = user_service.get_all_users();
    match users {
        Ok(users) => {
            println!("Usuários cadastrados:");
            for user in users {
                println!("Nome: {}", user.name);
                println!("E-mail: {}\n", user.email);
            }
        }
        Err(err) => eprintln!("Erro ao obter usuários: {}", err),
    }
}
