add_executable(
    cabrito
    main.cpp
    logic/rng.cpp
    logic/simulation.cpp
)

find_package(spdlog REQUIRED)

target_link_libraries(cabrito fmt)
target_link_libraries(cabrito spdlog)
target_compile_definitions(cabrito PRIVATE SPDLOG_FMT_EXTERNAL)