//importação das bibliotecas
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{process, vec};
use std::thread;

fn backup_caracter_especial(vetor_caracter: &Vec<char>){
    let nome_arquivo_backup_caracter_especial = "backup_caracter_especial.txt";

    let mut arquivo_backup_caracter_especial = match File::create(nome_arquivo_backup_caracter_especial) {
        Ok(f) => {
            println!("Arquivo criado com sucesso");
            f
        }

        Err(e) => {
            println!("Deu ruim na hora de criar o arquivo: {}", e);
            process::exit(1)
        }
    };

    for caractere in vetor_caracter { 
        if let Err(e) = write!(arquivo_backup_caracter_especial, "{}", caractere) {
            println!("deu BO mano: {}", e);
        }
    }

    println!("Fim da thread 03")
}

fn backup_numero_linha(numero_linhas: usize) {
    let nome_arquivo_backup_numeros_linhas = "backup_numeros_linhas.txt";

    let mut arquivo_backup_numeros_linhas = match File::create(nome_arquivo_backup_numeros_linhas) {
        Ok(f) => {
            println!("Arquivo criado com sucesso");
            f
        }

        Err(_e) => {
            println!("Deu ruim na hora de criar o arquivo");
            process::exit(1)
        }
    };

    if let Err(e) = write!(arquivo_backup_numeros_linhas, "{}", numero_linhas) {
        println!("deu ruim: {}", e);
    }

    println!("Fim da thread 04");
}

fn backup_numeros(numeros: &Vec<char>) {
    let nome_arquivo_backup_numeros = "backup_numeros.txt";

    let mut arquivo_backup_numeros = match File::create(nome_arquivo_backup_numeros) {
        Ok(f) => {
            println!("Arquivo criado com sucesso");
            f
        }

        Err(e) => {
            println!("Deu ruim na hora de criar o arquivo: {}", e);
            process::exit(1)
        }
    };

    for numero in numeros { 
        if let Err(e) = write!(arquivo_backup_numeros, "{}", numero) {
            println!("deu BO mano: {}", e);
        }
    }

    println!("Fim da thread 01");
}

fn backup_letras(vetor_letra: &Vec<char>) {
    let nome_arquivo_backup_letras = "backup_letras.txt";

    let mut arquivo_backup_letras = match File::create(nome_arquivo_backup_letras) {
        Ok(f) => {
            println!("Arquivo criado com sucesso");
            f
        }

        Err(e) => {
            println!("Deu ruim na hora de criar o arquivo: {}", e);
            process::exit(1)
        }
    };

    for letra in vetor_letra {
        if let Err(e) = write!(arquivo_backup_letras, "{}", letra) {
            println!("deu BO mano: {}", e);        
        }
    }

    println!("Fim da thread 02");
}

fn ler_arquivo(nome_arquivo: &str) {
    let file = match File::open(nome_arquivo) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Erro ao abrir o arquivo {}: {}", nome_arquivo, e);
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
 
    let mut numeros: Vec<char> = vec![];

    let mut letras: Vec<char> = vec![];
    
    let mut caracteres_especiais = vec![];

    let mut numero_linhas = 0;

    for linha in reader.lines() {
        match linha {
            Ok(l) => {
                numero_linhas += 1;
                for caractere in l.chars() {
                    if caractere.is_ascii_digit() {
                        numeros.push(caractere);
                    } else if caractere.is_alphabetic() {
                        letras.push(caractere);
                    } else {
                       caracteres_especiais.push(caractere);
                    }
                }
            }
            Err(e) => println!("deu erro ai: {}", e),
        }
    }

    let retorno_scope = thread::scope(|scope| {
        let mut handles = Vec::new();

        handles.push(scope.spawn(|| {
            println!("thread 01 - numeros");
            backup_numeros(&numeros);
        }));

        handles.push(scope.spawn(|| {
            println!("thread 02 - letras");
            backup_letras(&letras);
        }));

        handles.push(scope.spawn(|| {
            println!("thread 03 - caracter especial");
            backup_caracter_especial(&caracteres_especiais);
        }));

        handles.push(scope.spawn(|| {
            println!("thread 04 - numeros de linhas");
            backup_numero_linha(numero_linhas);
        }));

        let mut total_thread_finalizada = 0;

        for thread_num in handles.into_iter() {
            if let Ok(_thread_finalizada) = thread_num.join() {
                total_thread_finalizada += 1;
            }
        }

        total_thread_finalizada
    });

    println!("total de theads: {}", retorno_scope);
}

fn main() {
    let nome_arquivo: &'static str = "texto.txt";
    println!("nome do arquivo: {}", nome_arquivo);

    let handle = thread::spawn(move || {
        ler_arquivo(nome_arquivo);
    });

    // Aguarda a thread terminar
    handle.join().expect("Thread principal falhou");
}
