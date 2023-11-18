#pragma once
#include <random>
#include <stdint.h>
#include "cabrito.hpp"

namespace RNG
{
    double GenRng();
    bool CheckAlive(const double_t morte_mensal_frac);
    Cabrito::CabritoGenero Genero();
    std::vector<Cabrito::Cabrito> Parto(const double_t parto_duplo_frac);
}