use crate::{
    cabrito::{Cabrito, CabritoGenero},
    simulation::{Ambiente, SimEvento, Simulável},
};

use rand::{thread_rng, Rng};

/// Representa o estado da matriz, que pode ser prenha ou amamentando
#[derive(Debug, PartialEq, Clone)]
pub enum MatrizEstado {
    Prenha(usize),
    Amamentando(usize),
}

// O estado padrão de uma matriz é prenha a 0 meses
// Isso é usado no caso de novas matrizes serem criadas ao longo da simulação
impl Default for MatrizEstado {
    fn default() -> Self {
        Self::Prenha(0)
    }
}

/// Representa uma matriz, com idade e estado
#[derive(Debug, PartialEq, Clone)]
pub struct Matriz {
    pub state: MatrizEstado,
    pub idade: usize,
}

impl Default for Matriz {
    fn default() -> Self {
        Self {
            state: MatrizEstado::default(),
            idade: 0,
        }
    }
}

impl Matriz {
    /// Faz um parto
    fn parto(&self, env: &Ambiente) -> Vec<Cabrito> {
        let n_filhos = crate::rng_logic::parto(&env.config.filhos_por_100_partos);
        let mut filhos = vec![];
        for _ in 0..n_filhos {
            filhos.push(Cabrito {
                idade: 0,
                genero: CabritoGenero::from(thread_rng().gen::<bool>()),
            })
        }

        filhos
    }
}

impl Simulável for Matriz {
    fn step(&mut self, amb: &Ambiente) -> Option<SimEvento> {
        self.idade += 1;

        // Mata a matriz caso necessário
        if self.idade >= amb.config.tempo_vida_matriz {
            return Some(SimEvento::MorteMatriz(self.clone()));
        }

        match self.state {
            MatrizEstado::Amamentando(x) => {
                if x + 1 > amb.config.tempo_amamentando_meses {
                    self.state = MatrizEstado::Prenha(0);
                    return None;
                }

                self.state = MatrizEstado::Amamentando(x + 1);
            }
            MatrizEstado::Prenha(x) => {
                if x + 1 >= amb.config.tempo_prenhez_meses {
                    return Some(SimEvento::Parto(self.parto(amb)));
                }

                self.state = MatrizEstado::Prenha(x + 1)
            }
        }

        None
    }

    /// Cria uma nova matriz com dada idade
    fn parto_rng(idade_teto: usize) -> Self {
        Self {
            state: MatrizEstado::default(),
            idade: thread_rng().gen_range(0..idade_teto),
        }
    }

    fn new_por_idade(idade: usize) -> Self {
        Self {
            state: MatrizEstado::default(),
            idade,
        }
    }
}

impl From<Cabrito> for Matriz {
    fn from(value: Cabrito) -> Self {
        return Self {
            state: MatrizEstado::default(),
            idade: value.idade,
        };
    }
}
