cmake_minimum_required(VERSION 3.8)
set(This Serialization)

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

add_library(${This} STATIC ${Sources} ${Headers})
set_target_properties(${This} PROPERTIES
    FOLDER Libraries
)

target_include_directories(${This} PUBLIC include)

target_link_libraries(${This}
    SystemAbstractions
)
