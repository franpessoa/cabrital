use crate::simulation::Simulável;
use rand::{thread_rng, Rng};

use crate::simulation::{Ambiente, SimEvento};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cabrito {
    pub idade: usize,
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

impl Default for Cabrito {
    fn default() -> Self {
        Self {
            genero: CabritoGenero::from(thread_rng().gen::<bool>()),
            idade: 0,
        }
    }
}

impl Cabrito {
    /// Checks if the individual can become a matriz imediatly
    fn can_matriz(&self, env: &Ambiente) -> bool {
        self.genero == CabritoGenero::Femea && env.n_matrizes.get() < env.config.teto_matriz
    }
}

impl Simulável for Cabrito {
    fn step(&mut self, amb: &Ambiente) -> Option<SimEvento> {
        self.idade += 1;

        if self.idade >= amb.config.idade_abate_cabrito {
            if self.can_matriz(amb) {
                return Some(SimEvento::MaybeMatriz(*self));
            } else {
                return Some(SimEvento::Abate(*self));
            }
        }

        None
    }

    fn parto_rng(max_idade: usize) -> Self {
        let mut rng = thread_rng();
        Self {
            idade: rng.gen_range(0..max_idade),
            genero: CabritoGenero::from(thread_rng().gen::<bool>()),
        }
    }

    fn new_por_idade(idade: usize) -> Self {
        Self {
            genero: thread_rng().gen_bool(0.5).into(),
            idade,
        }
    }
}
