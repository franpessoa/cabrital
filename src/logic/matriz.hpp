#pragma once
#include <stdint.h>
#include "spdlog/spdlog.h"
#include "cabrito.hpp"
#include "rng.hpp"

namespace Matriz
{
    struct Matriz
    {
        uint32_t idade {6};

        std::vector<Cabrito::Cabrito> step(const double_t parto_duplo_frac, const uint32_t tempo_prenhez)
        {   
            ++idade;
            if (idade % tempo_prenhez == 0)
            {
                return RNG::Parto(parto_duplo_frac);
            } else {
                return {};
            };
        }
    };
};