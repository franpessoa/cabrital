use cabritos::{
    cabrito::Cabrito,
    matriz::Matriz,
    output::{write_head, write_record},
    simulation::{Simulation, SimulationConfig, SimulationStep},
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

#[derive(Deserialize)]
struct ConfigFile {
    config: SimulationConfig,
}

fn main() {
    //tracing_subscriber::fmt::init();

    let args = Args::parse();
    let contents = fs::read_to_string(args.cfg).unwrap();
    let config: ConfigFile = toml::from_str(&contents).unwrap();

    let matrizes = (0..100)
        .map(|_| Matriz::new(config.config.init_matrizes_idade))
        .collect::<Vec<Matriz>>();

    let cabritos = (0..100)
        .map(|_| Cabrito::parto_rng())
        .collect::<Vec<Cabrito>>();

    let mut simulation = Simulation {
        matrizes,
        cabritos,
        config: config.config,
        delta_t: 0,
        current_step: SimulationStep::default(),
    };

    let mut steps = Vec::with_capacity(simulation.config.rt_meses);

    for s in 0..simulation.config.rt_meses {
        println!("Calculando step {}", s);
        steps.push(simulation.step())
    }

    println!("Simulação finalizada");

    let out_path = args.out.unwrap_or(Local::now().to_string());
    let mut writer = csv::Writer::from_path(out_path).unwrap();
    write_head(&mut writer).unwrap();

    println!("Escrevendo resultados");

    for dp in steps {
        write_record(&mut writer, dp).unwrap()
    }
}
