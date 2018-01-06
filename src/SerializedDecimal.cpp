/**
 * @file SerializedDecimal.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedDecimal class.
 *
 * Copyright (c) 2013-2017 by Richard Walters
 */

#include <Serialization/SerializedDecimal.hpp>
#include <stdio.h>
#include <SystemAbstractions/StringExtensions.hpp>

namespace Serialization {

    SerializedDecimal::SerializedDecimal(double initialValue)
        : value_(initialValue)
    {
    }

    SerializedDecimal::operator double() const {
        return value_;
    }

    bool SerializedDecimal::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        return file->Write(&value_, sizeof(value_)) == sizeof(value_);
    }

    bool SerializedDecimal::Deserialize(SystemAbstractions::IFile* file) {
        return file->Read(&value_, sizeof(value_)) == sizeof(value_);
    }

    std::string SerializedDecimal::Render() const {
        return SystemAbstractions::sprintf("%lf", value_);
    }

    bool SerializedDecimal::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (rendering.find_first_of(" \r\n\t") != std::string::npos) {
            return false;
        }
        return (sscanf(rendering.c_str(), "%lf", &value_) == 1);
    }

    bool SerializedDecimal::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedDecimal*)other;
        return value_ == otherImpl->value_;
    }

}
