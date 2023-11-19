#include "simulation.hpp"
#include "cabrito.hpp"
#include "rng.hpp"
#include <vector>
#include <iterator>
#include "spdlog/spdlog.h"
#include "fmt/core.h"

SimulationStep Simulation::step()
{
    saída = 0;
    delta_t_meses++;
    spdlog::info(fmt::format("Reached step {}", delta_t_meses));

    int idx = 0;
    for (auto& cabrito: entities.cabritos)
    {
        cabrito.idade_meses++;
        int idx = 0;
        if (cabrito.idade_meses == cfg.tempo_abate_meses)
        {
            if (cabrito.genero == Cabrito::CabritoGenero::FEMEA && entities.matrizes.size() < cfg.max_matrizes)
            {
                entities.matrizes.push_back(Matriz::Matriz());
            } else {
                saída++;
            }

            entities.cabritos.erase(entities.cabritos.begin() + idx);
        } else if (RNG::CheckAlive(cfg.morte_mensal_frac)) {
            entities.cabritos.erase(entities.cabritos.begin() + idx);
        };

        idx++;
    };

    for (int i = 0; i < entities.matrizes.size(); i++)
    {
        Matriz::Matriz& matriz = entities.matrizes[i];
        auto step = matriz.step(cfg.parto_duplo_frac);
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

    SimulationStep step = {.cabritos = entities.cabritos, .matrizes = entities.matrizes, .saída_mensal = saída, .delta_t_meses = delta_t_meses};
    steps.push_back(step);

    return step;
};