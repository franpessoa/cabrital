use rand::{thread_rng, Rng};

use crate::{
    matriz::Matriz,
    simulation::{Environment, SimulationEvent},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cabrito {
    pub age: usize,
    pub gender: CabritoGenero,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CabritoGenero {
    Femea,
    Macho,
}

impl From<bool> for CabritoGenero {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Femea,
            false => Self::Macho,
        }
    }
}

impl Cabrito {
    /// Checks if the individual can become a matriz imediatly
    fn can_matriz(&self, env: &Environment) -> bool {
        self.gender == CabritoGenero::Femea && env.n_matrizes < env.config.teto_matriz
    }

    pub fn step(&mut self, env: &Environment) -> Option<SimulationEvent> {
        self.age += 1;

        if self.age >= env.config.idade_abate_cabrito {
            if self.can_matriz(env) {
                return Some(SimulationEvent::NewMatriz(Matriz::new(self.age), *self));
            } else {
                return Some(SimulationEvent::Abate(*self));
            }
        }

        None
    }

    pub fn parto_rng() -> Self {
        Self {
            age: 0,
            gender: CabritoGenero::from(thread_rng().gen::<bool>()),
        }
    }
}
