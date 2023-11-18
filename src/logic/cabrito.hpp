#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <random>
#include <chrono>

namespace Cabrito 
{
    enum class CabritoGenero 
    {
        MACHO, 
        FEMEA
    };

    struct Cabrito
    {
        uint32_t idade_meses {0};
        CabritoGenero genero;
    };
}

