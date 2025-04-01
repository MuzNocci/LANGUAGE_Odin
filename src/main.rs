mod lexer;
mod parser;
mod interpreter;
mod compiler;
mod vm;
mod stdlib;
mod repl;

use std::env;
use std::fs;
use std::io::{self};
use std::process;

use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(name = "odin")]
#[command(author = "Müller Nocciolli <muller@nocciolli.com.br>")]
#[command(version = "0.1.1")]
#[command(about = "Interpretador/Compilador da linguagem Odin", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Executa um arquivo Odin
    Run {
        /// Caminho para o arquivo Odin a ser executado
        #[arg(required = true)]
        file: String,
    },
    /// Compila um arquivo Odin para bytecode
    Compile {
        /// Caminho para o arquivo Odin a ser compilado
        #[arg(required = true)]
        file: String,
        
        /// Caminho para o arquivo de saída (bytecode)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Inicia o REPL (Read-Eval-Print Loop) interativo
    Repl {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file }) => {
            run_file(file)?;
        }
        Some(Commands::Compile { file, output }) => {
            let output_path = output.clone().unwrap_or_else(|| {
                let mut path = file.clone();
                if let Some(pos) = path.rfind('.') {
                    path.truncate(pos);
                }
                path + ".odin_bytecode"
            });
            compile_file(file, &output_path)?;
        }
        Some(Commands::Repl {}) | None => {
            run_repl()?;
        }
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Executando arquivo: {}", path);
    
    // Lê o conteúdo do arquivo
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Erro ao ler o arquivo '{}': {}", path, e))?;
    
    // Analisa e executa o código
    match run_code(&contents) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Erro ao executar o código: {}", e);
            process::exit(1);
        }
    }
}

fn compile_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Compilando {} para {}", input_path, output_path);
    
    // Lê o conteúdo do arquivo
    let contents = fs::read_to_string(input_path)
        .map_err(|e| format!("Erro ao ler o arquivo '{}': {}", input_path, e))?;
    
    // Implementar a compilação do código para bytecode
    let bytecode = compiler::compile(&contents)?;
    
    // Escreve o bytecode no arquivo de saída
    fs::write(output_path, bytecode)
        .map_err(|e| format!("Erro ao escrever o arquivo '{}': {}", output_path, e))?;
    
    println!("Compilação concluída com sucesso.");
    Ok(())
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("Odin Programming Language REPL v0.1.0");
    println!("Digite 'exit()' ou pressione Ctrl+C para sair");
    
    // Instancia o REPL
    let mut repl = repl::Repl::new();
    repl.run()?;
    
    Ok(())
}

// Função auxiliar para executar código Odin
fn run_code(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Tokenização
    let tokens = lexer::tokenize(code)?;
    
    // Parsing
    let ast = parser::parse(tokens)?;
    
    // Interpretação
    interpreter::evaluate(ast)?;
    
    Ok(())
}