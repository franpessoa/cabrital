find_path(SPDLOG_INCLUDE_DIR spdlog/spdlog.h
  PATHS
    ENV SPDLOG_ROOT
    ENV SPDLOG_INCLUDE_DIR
    ${SPDLOG_ROOT}
    /usr
    /usr/local
    $ENV{HOME}/.local
  PATH_SUFFIXES
    include
  )
mark_as_advanced(SPDLOG_INCLUDE_DIR)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(Spdlog
  REQUIRED_VARS SPDLOG_INCLUDE_DIR
  )

if(Spdlog_FOUND AND NOT TARGET Spdlog::Spdlog)
  add_library(Spdlog::Spdlog INTERFACE IMPORTED)
  set_target_properties(Spdlog::Spdlog PROPERTIES
    INTERFACE_INCLUDE_DIRECTORIES "${SPDLOG_INCLUDE_DIR}"
    )
endif()