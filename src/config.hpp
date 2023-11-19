#pragma once
#include "logic/simulation.hpp"
#include <toml.hpp>
#include <string>

SimulationConfig read_cfg(std::string fname);
SimulationEntities read_entities(std::string fname);
