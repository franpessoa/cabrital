#include "logic/cabrito.hpp"
#include "logic/matriz.hpp"
#include "logic/rng.hpp"
#include "logic/simulation.hpp"
#include <iostream>
#include <spdlog/spdlog.h>
#include <fstream>
#include <chrono>
#include <ctime>
#include <string>
#include <sstream>

int main()
{
    std::vector<Cabrito::Cabrito> cabritos_iniciais = {
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::MACHO},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
        {0, Cabrito::CabritoGenero::FEMEA},
    };

    Simulation simulation = {
        .cabritos = cabritos_iniciais,
        .parto_duplo_frac = 1/10,
        .morte_mensal_frac = 1/100,
        .tempo_prenhez_meses = 6,
        .tempo_abate_meses = 6,
        .max_matrizes = 8000,
        .tempo_vida_matriz_meses = 12 * 3,
        .morte_mensal_matrizes_frac = 1/200
    };

    spdlog::info("Built simulation struct");
    spdlog::info("Starting simulation");

    std::ofstream csv_out;

    // Get the NAME FOR THE FILE, fuck C++
    auto now = std::chrono::system_clock::now();
    std::time_t now_string = std::chrono::system_clock::to_time_t(now);
    std::stringstream ss;
    ss << now_string;
    csv_out.open(fmt::format("{}.csv", ss.str()));

    csv_out << "Meses,Matrizes,Cabritos\n";

    for (int i = 0; i < 12*30; i++)
    {
        auto step = simulation.step();
        csv_out << fmt::format("{},{},{}\n", step.delta_t_meses, step.matrizes.size(), step.cabritos.size());
    }

    csv_out.close();
}