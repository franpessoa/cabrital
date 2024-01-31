use crate::{cabrito::Cabrito, matriz::Matriz};

#[derive(Debug, PartialEq)]
pub struct Sim {
    pub matrizes: Vec<Matriz>,
    pub cabritos: Vec<Cabrito>,
    pub config: SimConfig,

    pub delta_t: usize,
    pub current_step: SimStep,
}

#[derive(serde_derive::Deserialize, Debug, PartialEq)]
pub struct SimConfig {
    pub filhos_por_100_partos: usize,
    pub tempo_prenhez_meses: usize,
    pub tempo_amamentando_meses: usize,
    pub idade_abate_cabrito: usize,
    pub tempo_crescimento_matriz: usize,
    pub tempo_vida_matriz: usize,
    pub teto_matriz: usize,
    pub init_matrizes: usize,
    pub init_cabritos: usize,
    pub init_matrizes_idade: usize,
    pub rt_meses: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimEvento {
    Parto(Vec<Cabrito>),
    Abate(Cabrito),
    MorteMatriz(Matriz),
    NewMatriz(Matriz, Cabrito),
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct SimStep {
    pub mes: usize,
    pub matrizes: usize,
    pub cabritos: usize,
    pub idade_média_matrizes: f32,
    pub imediato: SimStepImediato,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SimStepImediato {
    pub partos: usize,
    pub abates: usize,
    pub abates_macho: usize,
    pub abates_femea: usize,
    pub novas_matrizes: usize,
    pub mortes_matriz: usize,
}

#[derive(Debug)]
pub struct Ambiente<'a> {
    pub config: &'a SimConfig,
    pub n_matrizes: usize,
    pub n_cabritos: usize,
    pub delta_t: usize,
}

impl Sim {
    fn evento(&mut self, e: &SimEvento) {
        match e {
            SimEvento::Parto(c) => {
                self.cabritos.append(&mut c.clone());
                self.current_step.imediato.partos += c.len()
            }

            SimEvento::Abate(c) => {
                self.cabritos
                    .remove(self.cabritos.iter().position(|x| x == c).unwrap());
                self.current_step.imediato.abates += 1;

                match c.genero {
                    crate::cabrito::CabritoGenero::Femea => {
                        self.current_step.imediato.abates_femea += 1
                    }
                    crate::cabrito::CabritoGenero::Macho => {
                        self.current_step.imediato.abates_macho += 1
                    }
                }
            }
            SimEvento::NewMatriz(m, c) => {
                self.matrizes.push(m.clone());
                self.current_step.imediato.novas_matrizes += 1;
                self.cabritos
                    .remove(self.cabritos.iter().position(|x| x == c).unwrap());
            }

            SimEvento::MorteMatriz(m) => {
                self.matrizes
                    .remove(self.matrizes.iter().position(|x| x == m).unwrap());
                self.current_step.imediato.mortes_matriz += 1
            }
        }
    }

    pub fn step(&mut self) -> SimStep {
        let mut event_register = Vec::with_capacity(self.cabritos.len() + self.matrizes.len());

        self.current_step = SimStep::default();
        self.delta_t += 1;

        let amb = Ambiente {
            config: &self.config,
            n_matrizes: self.matrizes.len(),
            n_cabritos: self.cabritos.len(),
            delta_t: self.delta_t,
        };

        for cabrito in &mut self.cabritos {
            if let Some(e) = cabrito.step(&amb) {
                event_register.push(e)
            }
        }

        for matriz in &mut self.matrizes {
            if let Some(e) = matriz.step(&amb) {
                event_register.push(e)
            }
        }

        for event in event_register {
            self.evento(&event)
        }

        self.atualliza_imediato();
        self.current_step
    }

    fn atualliza_imediato(&mut self) {
        self.current_step.mes = self.delta_t;
        self.current_step.matrizes = self.matrizes.len();
        self.current_step.cabritos = self.cabritos.len();
        self.current_step.idade_média_matrizes =
            self.matrizes.iter().map(|x| x.idade).sum::<usize>() as f32
                / self.current_step.matrizes as f32
    }
}
