# Serialization

This is a library providing facilities to convert objects to and from a sequence of bytes for the purpose of storing them in files or conveying them between programs.

## Supported platforms / recommended toolchains

This library should build for any platform and with any toolchain which supports C++11.

## Building

[CMake](https://cmake.org/) is used to generate the build system for this library.  This library is intended to be included as part of a larger project, such as a program which uses the library.  The parent project should include this library's CMakeLists.txt via the *add_subdirectory* command.

### Prerequisites

* [CMake](https://cmake.org/) version 3.8 or newer
* C++ toolchain compatible with CMake for your development platform (e.g. [Visual Studio](https://www.visualstudio.com/) on Windows)

### Dependencies

* [SystemAbstractions](https://bitbucket.org/digitalstirling/systemabstractions)
