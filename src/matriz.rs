use crate::{
    cabrito::{Cabrito, CabritoGenero},
    simulation::{Environment, SimulationEvent},
};
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Clone)]
pub enum MatrizState {
    Prenha(usize),
    Amamentando(usize),
}

impl Default for MatrizState {
    fn default() -> Self {
        Self::Amamentando(0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matriz {
    pub state: MatrizState,
    pub age: usize,
}

impl Matriz {
    pub fn new(age: usize) -> Self {
        Self {
            state: MatrizState::default(),
            age,
        }
    }

    pub fn step(&mut self, env: &Environment) -> Option<SimulationEvent> {
        self.age += 1;

        match self.state {
            MatrizState::Amamentando(x) => {
                if x + 1 > env.config.tempo_amamentando_meses {
                    self.state = MatrizState::Prenha(0);
                    return None;
                }

                self.state = MatrizState::Amamentando(x + 1);
            }
            MatrizState::Prenha(x) => {
                if x + 1 >= env.config.tempo_prenhez_meses {
                    return Some(SimulationEvent::Parto(self.parto(env)));
                }

                self.state = MatrizState::Prenha(x + 1)
            }
        }

        if self.age >= env.config.tempo_vida_matriz {
            return Some(SimulationEvent::MorteMatriz(self.clone()));
        }

        None
    }

    fn parto(&self, env: &Environment) -> Vec<Cabrito> {
        let n_filhos = crate::rng_logic::parto(&env.config.filhos_por_100_partos);
        let mut filhos = vec![];
        for _ in 0..n_filhos {
            filhos.push(Cabrito {
                age: 0,
                gender: CabritoGenero::from(thread_rng().gen::<bool>()),
            })
        }

        filhos
    }
}
