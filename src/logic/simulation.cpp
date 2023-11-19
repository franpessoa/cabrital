#include "simulation.hpp"
#include "cabrito.hpp"
#include "rng.hpp"
#include <vector>
#include <iterator>
#include "spdlog/spdlog.h"
#include "fmt/core.h"

SimulationStep Simulation::step()
{
    uint32_t saida_macho = 0;
    uint32_t saida_femea = 0;

    delta_t_meses++;
    spdlog::info(fmt::format("Reached step {}", delta_t_meses));

    uint32_t idx = 0;

    for (uint32_t i = 0; i < entities.matrizes.size(); i++)
    {
        Matriz::Matriz& matriz = entities.matrizes[i];
        matriz.idade++;
        auto step = matriz.step(cfg.parto_duplo_frac, cfg.tempo_prenhez_meses);

        entities.cabritos.insert(
            entities.cabritos.end(),
            std::make_move_iterator(step.begin()),
            std::make_move_iterator(step.end())
        );

        if (matriz.idade >= cfg.tempo_vida_matriz_meses || RNG::CheckAlive(cfg.morte_mensal_matrizes_frac))
        {
            entities.matrizes.erase(entities.matrizes.begin() + i);
        }
    }

    for (auto& cabrito: entities.cabritos)
    {
        cabrito.idade_meses++;
        if (cabrito.idade_meses >= cfg.tempo_abate_meses)
        {
            if (cabrito.genero == Cabrito::CabritoGenero::FEMEA)
            {
                saida_femea++;

                if (entities.matrizes.size() < cfg.max_matrizes)
                {
                    entities.matrizes.push_back(Matriz::Matriz());
                }
                
            } else {
                saida_macho++;
            }

            // auto init = entities.cabritos.size();
            entities.cabritos.erase(entities.cabritos.begin() + idx);
            // auto end = entities.cabritos.size();
        } else if (RNG::CheckAlive(cfg.morte_mensal_frac)) {
            entities.cabritos.erase(entities.cabritos.begin() + idx);
        };

        idx++;
    };

    SimulationStep step = {
        .cabritos = entities.cabritos, 
        .matrizes = entities.matrizes, 
        .saída_mensal = saida_femea + saida_macho, 
        .delta_t_meses = delta_t_meses,
        .abates_macho = saida_macho,
        .abates_femea = saida_femea
    };
    steps.push_back(step);

    return step;
};