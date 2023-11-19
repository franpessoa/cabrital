dependencies = []
dependencies += dependency('fmt', required : true, static : true, native: true)
dependencies += dependency('spdlog', required : true, static : true, native : true)

exe = executable(
  'cabrito', 
  ['main.cpp', 'logic/rng.cpp', 'logic/simulation.cpp'],
  dependencies : dependencies,
  include_directories : incdirs
)