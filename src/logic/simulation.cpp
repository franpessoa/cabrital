#include "simulation.hpp"
#include "cabrito.hpp"
#include "rng.hpp"
#include <vector>
#include <iterator>
#include <spdlog/spdlog.h>
#include <fmt/core.h>

SimulationStep Simulation::step()
{
    saída = 0;
    delta_t_meses++;
    spdlog::info(fmt::format("Reached step {}", delta_t_meses));

    int idx = 0;
    for (auto& cabrito: cabritos)
    {
        cabrito.idade_meses++;
        int idx = 0;
        if (cabrito.idade_meses == tempo_abate_meses)
        {
            if (cabrito.genero == Cabrito::CabritoGenero::FEMEA && matrizes.size() < max_matrizes)
            {
                spdlog::warn("creating matriz");
                matrizes.push_back(Matriz::Matriz());
            } else {
                saída++;
            }

            cabritos.erase(cabritos.begin() + idx);
        } else if (RNG::CheckAlive(morte_mensal_frac)) {
            cabritos.erase(cabritos.begin() + idx);
        };

        idx++;
    };

    for (int i = 0; i < matrizes.size(); i++)
    {
        Matriz::Matriz& matriz = matrizes[i];
        auto step = matriz.step(parto_duplo_frac);
        cabritos.insert(
            cabritos.end(),
            std::make_move_iterator(step.begin()),
            std::make_move_iterator(step.end())
        );
        if (matriz.idade >= tempo_vida_matriz_meses || RNG::CheckAlive(morte_mensal_matrizes_frac))
        {
            spdlog::warn("apagando matriz");
            matrizes.erase(matrizes.begin() + i);
        }
    }

    SimulationStep step = {.cabritos = cabritos, .matrizes = matrizes, .saída_mensal = saída, .delta_t_meses = delta_t_meses};
    steps.push_back(step);

    return step;
};