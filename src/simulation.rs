use crate::{cabrito::Cabrito, matriz::Matriz};

#[derive(Debug, PartialEq)]
pub struct Simulation {
    pub matrizes: Vec<Matriz>,
    pub cabritos: Vec<Cabrito>,
    pub config: SimulationConfig,

    pub delta_t: usize,
    pub current_step: SimulationStep,
}

#[derive(serde_derive::Deserialize, Debug, PartialEq)]
pub struct SimulationConfig {
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
pub enum SimulationEvent {
    Parto(Vec<Cabrito>),
    Abate(Cabrito),
    MorteMatriz(Matriz),
    NewMatriz(Matriz, Cabrito),
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct SimulationStep {
    pub mes: usize,
    pub matrizes: usize,
    pub cabritos: usize,
    pub idade_média_matrizes: f32,
    pub immediate: ImmediateEventStep,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ImmediateEventStep {
    pub partos: usize,
    pub abates: usize,
    pub abates_macho: usize,
    pub abates_femea: usize,
    pub novas_matrizes: usize,
    pub mortes_matriz: usize,
}

#[derive(Debug)]
pub struct Environment<'a> {
    pub config: &'a SimulationConfig,
    pub n_matrizes: usize,
    pub n_cabritos: usize,
    pub delta_t: usize,
}

impl Simulation {
    fn event(&mut self, e: &SimulationEvent) {
        match e {
            SimulationEvent::Parto(c) => {
                self.cabritos.append(&mut c.clone());
                self.current_step.immediate.partos += c.len()
            }

            SimulationEvent::Abate(c) => {
                self.cabritos
                    .remove(self.cabritos.iter().position(|x| x == c).unwrap());
                self.current_step.immediate.abates += 1;

                match c.gender {
                    crate::cabrito::CabritoGenero::Femea => {
                        self.current_step.immediate.abates_femea += 1
                    }
                    crate::cabrito::CabritoGenero::Macho => {
                        self.current_step.immediate.abates_macho += 1
                    }
                }
            }
            SimulationEvent::NewMatriz(m, c) => {
                self.matrizes.push(m.clone());
                self.current_step.immediate.novas_matrizes += 1;
                self.cabritos
                    .remove(self.cabritos.iter().position(|x| x == c).unwrap());
            }

            SimulationEvent::MorteMatriz(m) => {
                self.matrizes
                    .remove(self.matrizes.iter().position(|x| x == m).unwrap());
                self.current_step.immediate.mortes_matriz += 1
            }
        }
    }

    pub fn step(&mut self) -> SimulationStep {
        let mut event_register = Vec::with_capacity(self.cabritos.len() + self.matrizes.len());

        self.current_step = SimulationStep::default();
        self.delta_t += 1;

        let env = Environment {
            config: &self.config,
            n_matrizes: self.matrizes.len(),
            n_cabritos: self.cabritos.len(),
            delta_t: self.delta_t,
        };

        for cabrito in &mut self.cabritos {
            if let Some(e) = cabrito.step(&env) {
                event_register.push(e)
            }
        }

        for matriz in &mut self.matrizes {
            if let Some(e) = matriz.step(&env) {
                event_register.push(e)
            }
        }

        for event in event_register {
            self.event(&event)
        }

        self.update_step_fields();
        self.current_step
    }

    fn update_step_fields(&mut self) {
        self.current_step.mes = self.delta_t;
        self.current_step.matrizes = self.matrizes.len();
        self.current_step.cabritos = self.cabritos.len();
        self.current_step.idade_média_matrizes = self.matrizes.iter().map(|x| x.age).sum::<usize>()
            as f32
            / self.current_step.matrizes as f32
    }
}
