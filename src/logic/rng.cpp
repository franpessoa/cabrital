#include "rng.hpp"
#include <random>
#include <stdint.h>
#include "cabrito.hpp"

namespace RNG {
    double GenRng() {
        std::random_device rng_device;
        std::mt19937_64 eng { rng_device() };
        std::uniform_real_distribution<double> rng(0, 1);

        return rng(eng);
    }

    bool CheckAlive(double_t morte_mensal_frac) {
        return (GenRng() <= morte_mensal_frac);
    }

    Cabrito::CabritoGenero Genero()
    {
        if (GenRng() < 0.5)
        {
            return Cabrito::CabritoGenero::FEMEA;
        };

        return Cabrito::CabritoGenero::MACHO;
    };

    std::vector<Cabrito::Cabrito> Parto(const double_t parto_duplo_frac)
    {
        // Em caso de parto duplo
        if (GenRng() <= parto_duplo_frac)
        {
            return {
                Cabrito::Cabrito { .genero = Genero() },
                Cabrito::Cabrito { .genero = Genero() },
            };
        };

        // Senão,
        return {
            Cabrito::Cabrito { .genero = Genero() }
        };
    };
}

