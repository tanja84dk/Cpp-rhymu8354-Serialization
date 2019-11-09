/**
 * @file SerializedIpAddress.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedIpAddress class.
 *
 * Copyright (c) 2013-2018 by Richard Walters
 */

#include <inttypes.h>
#include <Serialization/SerializedIpAddress.hpp>
#include <StringExtensions/StringExtensions.hpp>

namespace Serialization {

    SerializedIpAddress::SerializedIpAddress(uint32_t initialValue)
        : value_(initialValue)
    {
    }

    SerializedIpAddress::operator uint32_t() const {
        return value_;
    }

    bool SerializedIpAddress::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        return file->Write(&value_, sizeof(value_)) == sizeof(value_);
    }

    bool SerializedIpAddress::Deserialize(SystemAbstractions::IFile* file) {
        return file->Read(&value_, sizeof(value_)) == sizeof(value_);
    }

    std::string SerializedIpAddress::Render() const {
        return StringExtensions::sprintf(
            "%" PRIu8 ".%" PRIu8 ".%" PRIu8 ".%" PRIu8,
            (uint8_t)((value_ >> 24) & 0xFF),
            (uint8_t)((value_ >> 16) & 0xFF),
            (uint8_t)((value_ >> 8) & 0xFF),
            (uint8_t)(value_ & 0xFF)
        );
    }

    bool SerializedIpAddress::Parse(std::string rendering) {
        rendering = StringExtensions::Trim(rendering);
        if (rendering.find_first_of(" \r\n\t") != std::string::npos) {
            return false;
        }
        uint8_t valueBytes[4];
        if (
            sscanf(
                rendering.c_str(),
                "%" SCNu8 ".%" SCNu8 ".%" SCNu8 ".%" SCNu8,
                &valueBytes[0],
                &valueBytes[1],
                &valueBytes[2],
                &valueBytes[3]
            ) == 4
        ) {
            value_ = (
                (((uint32_t)valueBytes[0]) << 24)
                | (((uint32_t)valueBytes[1]) << 16)
                | (((uint32_t)valueBytes[2]) << 8)
                | ((uint32_t)valueBytes[3])
            );
            return true;
        } else {
            return false;
        }
    }

    bool SerializedIpAddress::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedIpAddress*)other;
        return value_ == otherImpl->value_;
    }

}
