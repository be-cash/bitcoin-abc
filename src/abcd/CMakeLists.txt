set(CMAKE_CXX_STANDARD 17)

include_directories(..)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

set(ABCD_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libabcd.a")
set(ABCD_CXX "${CMAKE_CURRENT_BINARY_DIR}/abcdgen.cpp")
set(ABCD_BRIDGE_CXX "${CMAKE_CURRENT_BINARY_DIR}/abcdgen_bridge.cpp")

file(
    GLOB
    ABCD_RUST_FILES
    CONFIGURE_DEPENDS
    "${PROJECT_SOURCE_DIR}/abcd/abcd/src/*.rs"
    "${PROJECT_SOURCE_DIR}/abcd/abcd-address-indexer/src/*.rs"
    "${PROJECT_SOURCE_DIR}/abcd/abcd-base/src/*.rs"
    "${PROJECT_SOURCE_DIR}/abcd/abcd-bridge/src/*.rs"
)

add_library(abcd STATIC ${ABCD_CXX} ${ABCD_BRIDGE_CXX})
add_custom_command(
    OUTPUT ${ABCD_CXX} ${ABCD_BRIDGE_CXX} ${ABCD_LIB}
    DEPENDS ${ABCD_RUST_FILES}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/abcd/src/lib.rs.cc ${ABCD_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/abcd/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/abcdgen.h
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/abcd-bridge/src/lib.rs.cc ${ABCD_BRIDGE_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/abcd-bridge/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/abcdgen_bridge.h
    COMMAND cp -r ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust ${CMAKE_CURRENT_BINARY_DIR}/
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/abcd
)
target_link_libraries(abcd ${ABCD_LIB})

include(NativeExecutable)
macro(link_boost TARGET)
	non_native_target_link_libraries(${TARGET} Boost 1.59 ${ARGN})
endmacro()
link_boost(abcd filesystem)
