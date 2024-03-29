use crate::{cabrito::Cabrito, matriz::Matriz};
use rayon::prelude::*;

/// Estrutura que representa a simulação completa
#[derive(Debug, PartialEq)]
pub struct Sim {
    pub matrizes: Vec<Matriz>,
    pub cabritos: Vec<Cabrito>,
    pub config: SimConfig,

    pub delta_t: usize,
    pub current_step: SimStep,
}

/// Configurações da simulação que serão lidas do arquivo TOML de configuração
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

/// Representa um evento que muda o estado da simulação
/// Qualquer tipo de mudança no objeto `Sim` que uma entidade quiser fazer deve ser na forma de um evento
/// Isso garante que todas as mudanças passem por um controle central
#[derive(Clone, Debug, PartialEq)]
pub enum SimEvento {
    Parto(Vec<Cabrito>),
    Abate(Cabrito),
    MorteMatriz(Matriz),
    NovaMatriz(Matriz, Cabrito),
}

/// Representa a saída de um mês
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SimStep {
    pub mes: usize,
    pub matrizes: usize,
    pub cabritos: usize,
    pub idade_média_matrizes: f32,
    pub imediato: SimStepImediato,
}

/// Representa os dados de saída que são atualizados a cada evento novo
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SimStepImediato {
    pub partos: usize,
    pub abates: usize,
    pub abates_macho: usize,
    pub abates_femea: usize,
    pub novas_matrizes: usize,
    pub mortes_matriz: usize,
}

/// Representa a referência que um cabrito/matriz tem da simulação completa
#[derive(Debug)]
pub struct Ambiente<'a> {
    pub config: &'a SimConfig,
    pub n_matrizes: usize,
    pub n_cabritos: usize,
    pub delta_t: usize,
}

impl Sim {
    /// Registra um evento
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
            SimEvento::NovaMatriz(m, c) => {
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

    /// Avança a simulação em um passo
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

        event_register.append(
            &mut self
                .cabritos
                .par_iter_mut()
                .map(|x| x.step(&amb))
                .flatten()
                .collect::<Vec<_>>(),
        );

        event_register.append(
            &mut self
                .matrizes
                .par_iter_mut()
                .map(|x| x.step(&amb))
                .flatten()
                .collect::<Vec<_>>(),
        );

        for event in event_register {
            self.evento(&event)
        }

        self.atualiza_saida();
        self.current_step
    }

    /// Atualiza os dados da saída
    fn atualiza_saida(&mut self) {
        self.current_step.mes = self.delta_t;
        self.current_step.matrizes = self.matrizes.len();
        self.current_step.cabritos = self.cabritos.len();
        self.current_step.idade_média_matrizes =
            self.matrizes.iter().map(|x| x.idade).sum::<usize>() as f32
                / self.current_step.matrizes as f32
    }
}
