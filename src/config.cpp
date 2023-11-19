#include "config.hpp"
#include "logic/simulation.hpp"
#include "logic/cabrito.hpp"
#include "logic/matriz.hpp"
#include "logic/rng.hpp"
#include <toml.hpp>
#include <string>
#include <vector>


SimulationConfig read_cfg(std::string fname)
{
    const auto data = toml::parse(fname);
    const auto& cfg = toml::find(data, "config");
    const double_t parto_duplo_frac = toml::find_or(cfg, "parto_duplo", 1/20);
    const double_t morte_mensal_frac = toml::find_or(cfg, "morte_mensal", 1/100);
    const uint32_t tempo_prenhez_meses = toml::find_or(cfg, "tempo_prenhez", 6);
    const uint32_t tempo_abate_meses = toml::find_or(cfg, "tempo_abate", 6);
    const uint32_t max_matrizes = toml::find_or(cfg, "teto_matrizes", 1000);
    const uint32_t tempo_vida_matriz_meses = toml::find_or(cfg, "tempo_vida_matriz", 3*12);
    const double_t morte_mensal_matrizes_frac = toml::find_or(cfg, "morte_mensal_matrizes", 1/100);
    const uint32_t tempo_total = toml::find_or(cfg, "tempo_total", 12*50);

    const SimulationConfig result = {
        .parto_duplo_frac = parto_duplo_frac,
        .morte_mensal_frac = morte_mensal_frac,
        .tempo_prenhez_meses = tempo_prenhez_meses,
        .tempo_abate_meses = tempo_abate_meses,
        .max_matrizes = max_matrizes,
        .tempo_vida_matriz_meses = tempo_vida_matriz_meses,
        .morte_mensal_matrizes_frac = morte_mensal_matrizes_frac,
        .tempo_simulacao = tempo_total
    };

    return result;
};

SimulationEntities read_entities(std::string fname)
{
    const auto data = toml::parse(fname);
    const auto& cfg = toml::find(data, "entidades");
    const uint32_t machos = toml::find_or(cfg, "machos", 30);
    const uint32_t femeas = toml::find_or(cfg, "femeas", 30);
    const uint32_t matrizes = toml::find_or(cfg, "machos", 30);

    std::vector<Cabrito::Cabrito> v_cabritos = {};
    std::vector<Matriz::Matriz> v_matrizes = {};

    for (uint32_t i = 0; i < femeas; i++) { v_cabritos.push_back({.idade_meses = static_cast<uint32_t>(RNG::GenRng() * 6), .genero = Cabrito::CabritoGenero::FEMEA, }); };
    for (uint32_t i = 0; i < machos; i++) { v_cabritos.push_back({.idade_meses = static_cast<uint32_t>(RNG::GenRng() * 6), .genero = Cabrito::CabritoGenero::MACHO, }); };
    for (uint32_t i = 0; i < matrizes; i++) { v_matrizes.push_back({.idade = static_cast<uint32_t>(RNG::GenRng() * 36)}); };

    SimulationEntities entities = {
        .cabritos = v_cabritos,
        .matrizes = v_matrizes
    };

    return entities;
};
