# CMakeLists.txt for Serialization
# Original © 2018 by Richard Walters https://github.com/tanja84dk/Cpp-rhymu8354-Utf8
# Recreated and updated 2022 by Tanja84dk
# © 2023 by Tanja84dk

cmake_minimum_required(VERSION 3.8)
project(Serialization)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
#include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/Tanja84dkWarnings.cmake)

find_package(CPM REQUIRED)
find_package(Cpp-rhymu8354-StringExtensions REQUIRED)
find_package(Cpp-rhymu8354-SystemAbstractions REQUIRED)

set(Headers
    include/Serialization/ISerializedObject.hpp
    include/Serialization/SerializedBoolean.hpp
    include/Serialization/SerializedCollection.hpp
    include/Serialization/SerializedDecimal.hpp
    include/Serialization/SerializedInteger.hpp
    include/Serialization/SerializedIntegerVector.hpp
    include/Serialization/SerializedIpAddress.hpp
    include/Serialization/SerializedObject.hpp
    include/Serialization/SerializedString.hpp
    include/Serialization/SerializedUnsignedInteger.hpp
    include/Serialization/SerializedUnsignedIntegerVector.hpp
    include/Serialization/SerializedVector.hpp
)

set(Sources
    src/SerializedBoolean.cpp
    src/SerializedCollection.cpp
    src/SerializedDecimal.cpp
    src/SerializedInteger.cpp
    src/SerializedIntegerVector.cpp
    src/SerializedIpAddress.cpp
    src/SerializedObject.cpp
    src/SerializedString.cpp
    src/SerializedUnsignedInteger.cpp
    src/SerializedUnsignedIntegerVector.cpp
    src/SerializedVector.cpp
)

set(${PROJECT_NAME}_Libraries
    rhymu8354::StringExtensions
    rhymu8354::SystemAbstractions
)

add_library(${PROJECT_NAME} STATIC ${Sources} ${Headers})

add_library(libs::${PROJECT_NAME} ALIAS ${PROJECT_NAME})
add_library(rhymu8354::${PROJECT_NAME} ALIAS ${PROJECT_NAME})

target_link_libraries(${PROJECT_NAME} ${Serialization_Libraries})

target_include_directories(${PROJECT_NAME}
	PUBLIC
		$<INSTALL_INTERFACE:include>
		$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
	PRIVATE
		${CMAKE_CURRENT_SOURCE_DIR}
)

target_compile_features(${PROJECT_NAME}
	PRIVATE
		cxx_std_14
)

set_target_properties(${PROJECT_NAME}
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
	VERSION ${PROJECT_VERSION}
	SOVERSION ${PROJECT_VERSION_MAJOR}
    FOLDER Libraries
)

if(BUILD_TESTING)
	add_subdirectory(test)
endif()
