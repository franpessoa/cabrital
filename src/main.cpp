#include "logic/cabrito.hpp"
#include "logic/matriz.hpp"
#include "logic/rng.hpp"
#include "logic/simulation.hpp"
#include "config.hpp"
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
    CLI::App app{"Simulação de cabritos"};

    auto now = std::chrono::system_clock::now();
    std::time_t now_string = std::chrono::system_clock::to_time_t(now);
    std::stringstream ss;
    ss << now_string;

    std::string cfg_file = "config.toml";
    std::string out_file = ss.str();

    app.add_option("-f,--file", cfg_file, "entrada de dados");
    app.add_option("-o,--output", out_file, "saída");

    CLI11_PARSE(app, argc, argv);

    SimulationEntities entities = read_entities(cfg_file);
    SimulationConfig config = read_cfg(cfg_file);

    Simulation simulation = {
        .cfg = config,
        .entities = entities
    };

    spdlog::info("Built simulation struct");
    spdlog::info("Starting simulation");

    std::ofstream csv_out;


    csv_out.open(out_file);
    spdlog::warn("Writing to {}\n", out_file);
    csv_out << "Meses,Matrizes,Machos,Femeas,Saída,Total,Abates Macho,Abates Femea\n";

    for (int i = 0; i < simulation.cfg.tempo_simulacao; i++)
    {
        auto step = simulation.step();

        std::vector<Cabrito::Cabrito> machos;
        std::vector<Cabrito::Cabrito> femeas;

        std::copy_if(
            step.cabritos.begin(),
            step.cabritos.end(),
            std::back_inserter(machos),
            [](Cabrito::Cabrito c){ return c.genero == Cabrito::CabritoGenero::MACHO; }
        );

        std::copy_if(
            step.cabritos.begin(),
            step.cabritos.end(),
            std::back_inserter(femeas),
            [](Cabrito::Cabrito c){ return c.genero == Cabrito::CabritoGenero::FEMEA; }
        );

        csv_out << fmt::format("{},{},{},{},{},{},{},{}\n", 
            step.delta_t_meses, 
            step.matrizes.size(), 
            machos.size(), 
            femeas.size(), 
            step.saída_mensal, 
            step.cabritos.size(),
            step.abates_macho,
            step.abates_femea);
    }

    csv_out.close();
    spdlog::info("Finished simulation");
}