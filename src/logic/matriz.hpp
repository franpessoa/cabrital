#pragma once
#include <stdint.h>
#include <spdlog/spdlog.h>
#include "cabrito.hpp"
#include "rng.hpp"

namespace Matriz
{
    enum class MatrizStep
    {
        MATRIZ_STEP_PARTO,
        MATRIZ_STEP_PARTO_DUPLO,
        MATRIZ_STEP_NADA
    };

    struct Matriz
    {
        uint32_t idade {6};

        std::vector<Cabrito::Cabrito> step(const double_t parto_duplo_frac)
        {   
            idade++;
            if (idade % 6 == 0)
            {
                return RNG::Parto(parto_duplo_frac);
            } else {
                return {};
            };
        }
    };
};