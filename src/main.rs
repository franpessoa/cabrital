use cabritos::{
    cabrito::Cabrito,
    matriz::Matriz,
    output::{registra_cabecalho, registra_record},
    simulation::{Sim, SimConfig, Simulável},
};
use chrono::prelude::*;
use clap::Parser;
use serde_derive::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("./config.toml"))]
    cfg: String,
    #[arg(short, long)]
    out: Option<String>,
}

/// Representa o formato usado no arquivo TOML de configuração
#[derive(Deserialize)]
struct ConfigFile {
    config: SimConfig,
}

fn eval_novo_ind<T: Simulável>(idade: Option<usize>, max_idade: usize) -> T {
    match idade {
        Some(i) => T::new_por_idade(i),
        None => T::parto_rng(max_idade),
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Iniciando simulação");

    let args = Args::parse();
    let contents = fs::read_to_string(args.cfg).unwrap();
    let config: ConfigFile = toml::from_str(&contents).unwrap();

    let matrizes = (0..config.config.init_matrizes)
        .map(|_| {
            eval_novo_ind::<Matriz>(
                config.config.init_matrizes_idade,
                config.config.tempo_vida_matriz,
            )
        })
        .collect::<Vec<Matriz>>();

    let cabritos = (0..config.config.init_cabritos)
        .map(|_| {
            eval_novo_ind::<Cabrito>(
                config.config.init_cabritos_idade,
                config.config.idade_abate_cabrito,
            )
        })
        .collect::<Vec<Cabrito>>();

    let mut simulation = Sim {
        matrizes,
        cabritos,
        config: config.config,
        delta_t: 0,
    };

    let mut steps = Vec::with_capacity(simulation.config.rt_meses);

    for n in 0..simulation.config.rt_meses {
        tracing::info!("Calculando step {}", n);
        steps.push(simulation.step())
    }

    tracing::info!("Simulação finalizada");

    let out_path = args.out.unwrap_or(Local::now().to_string());
    let mut writer = csv::Writer::from_path(out_path).unwrap();
    registra_cabecalho(&mut writer).unwrap();

    tracing::info!("Escrevendo resultados");

    for dp in steps {
        registra_record(&mut writer, dp).unwrap()
    }
}
