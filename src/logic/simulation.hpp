#pragma once
#include "cabrito.hpp"
#include "matriz.hpp"

struct SimulationStep
{
    std::vector<Cabrito::Cabrito> cabritos;
    std::vector<Matriz::Matriz> matrizes;
    uint32_t saída_mensal;
    uint32_t delta_t_meses;
    uint32_t abates_macho;
    uint32_t abates_femea;
};

struct SimulationConfig
{
    const double_t parto_duplo_frac;
    const double_t morte_mensal_frac;
    const uint32_t tempo_prenhez_meses;
    const uint32_t tempo_abate_meses;
    const uint32_t max_matrizes;
    const uint32_t tempo_vida_matriz_meses;
    const double_t morte_mensal_matrizes_frac;
    const uint32_t tempo_simulacao;
};

struct SimulationEntities
{
    std::vector<Cabrito::Cabrito> cabritos = {};
    std::vector<Matriz::Matriz> matrizes = {};
};

struct Simulation
{
    std::vector<SimulationStep> steps = {};

    uint32_t delta_t_meses = {0};

    SimulationConfig cfg;
    SimulationEntities entities;
    SimulationStep step();
};