use rand::{thread_rng, Rng};

use crate::{
    matriz::Matriz,
    simulation::{Ambiente, SimEvento},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cabrito {
    pub age: usize,
    pub genero: CabritoGenero,
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
    fn can_matriz(&self, env: &Ambiente) -> bool {
        self.genero == CabritoGenero::Femea && env.n_matrizes.get() < env.config.teto_matriz
    }

    pub fn step(&mut self, env: &Ambiente) -> Option<SimEvento> {
        self.age += 1;

        if self.age >= env.config.idade_abate_cabrito {
            if self.can_matriz(env) {
                return Some(SimEvento::NovaMatriz(Matriz::new(self.age), *self));
            } else {
                return Some(SimEvento::Abate(*self));
            }
        }

        None
    }

    pub fn parto_rng() -> Self {
        Self {
            age: 0,
            genero: CabritoGenero::from(thread_rng().gen::<bool>()),
        }
    }
}
