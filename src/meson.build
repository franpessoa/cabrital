dependencies = []

dependencies += dependency('spdlog', required : true, static : true, native : true)
dependencies += dependency('fmt', required : true, native: true, static: true)

executable(
  'cabrito', 
  ['main.cpp', 'logic/rng.cpp', 'logic/simulation.cpp', 'config.cpp'],
  dependencies : dependencies,
  include_directories : incdirs
)