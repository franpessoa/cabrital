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
#include <CLI11.hpp>

int main(int argc, char** argv)
{
    CLI::App app{"App description"};
    
    double_t parto_duplo_frac {1/4};
    double_t morte_mensal_frac {1/100};
    uint32_t tempo_prenhez_meses {6};
    uint32_t tempo_abate_meses {6};
    uint32_t max_matrizes {500};
    uint32_t tempo_vida_matriz_meses {12*3};
    double_t morte_mensal_matrizes_frac {1/100};

    app.add_option("--parto-duplo", parto_duplo_frac, "número de partos duplos");
    app.add_option("--morte-mensal", morte_mensal_frac, "fração de mortes mensais");
    app.add_option("--prenhez", tempo_prenhez_meses, "tempo de prenhez");
    app.add_option("--tempo-abate", tempo_abate_meses, "tempo para abate");
    app.add_option("--max-matrizes", max_matrizes, "máximo de matrizes");
    app.add_option("--vida-matriz", tempo_vida_matriz_meses, "tempo de vida das matrizes");
    app.add_option("--frac-morte-matriz", morte_mensal_matrizes_frac, "mortes mensais de matrizes (fração)");
    CLI11_PARSE(app, argc, argv);

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
        .parto_duplo_frac = parto_duplo_frac,
        .morte_mensal_frac = morte_mensal_frac,
        .tempo_prenhez_meses = tempo_prenhez_meses,
        .tempo_abate_meses = tempo_abate_meses,
        .max_matrizes = max_matrizes,
        .tempo_vida_matriz_meses = tempo_vida_matriz_meses,
        .morte_mensal_matrizes_frac = morte_mensal_matrizes_frac
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