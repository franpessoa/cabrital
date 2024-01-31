use crate::{
    cabrito::{Cabrito, CabritoGenero},
    simulation::{Ambiente, SimEvento},
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

impl Matriz {
    /// Cria uma nova matriz com dada idade
    pub fn new(idade: usize) -> Self {
        Self {
            state: MatrizEstado::default(),
            idade,
        }
    }

    /// Avança a matriz em um passo
    pub fn step(&mut self, env: &Ambiente) -> Option<SimEvento> {
        self.idade += 1;

        // Mata a matriz caso necessário
        if self.idade >= env.config.tempo_vida_matriz {
            return Some(SimEvento::MorteMatriz(self.clone()));
        }

        match self.state {
            MatrizEstado::Amamentando(x) => {
                if x + 1 > env.config.tempo_amamentando_meses {
                    self.state = MatrizEstado::Prenha(0);
                    return None;
                }

                self.state = MatrizEstado::Amamentando(x + 1);
            }
            MatrizEstado::Prenha(x) => {
                if x + 1 >= env.config.tempo_prenhez_meses {
                    return Some(SimEvento::Parto(self.parto(env)));
                }

                self.state = MatrizEstado::Prenha(x + 1)
            }
        }

        None
    }

    /// Faz um parto
    fn parto(&self, env: &Ambiente) -> Vec<Cabrito> {
        let n_filhos = crate::rng_logic::parto(&env.config.filhos_por_100_partos);
        let mut filhos = vec![];
        for _ in 0..n_filhos {
            filhos.push(Cabrito {
                age: 0,
                genero: CabritoGenero::from(thread_rng().gen::<bool>()),
            })
        }

        filhos
    }
}
