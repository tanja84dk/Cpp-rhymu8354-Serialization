/**
 * @file SerializedString.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedString class.
 *
 * Copyright (c) 2013-2018 by Richard Walters
 */

#include <Serialization/SerializedUnsignedInteger.hpp>
#include <Serialization/SerializedString.hpp>
#include <StringExtensions/StringExtensions.hpp>

namespace Serialization {

    SerializedString::SerializedString(std::string initialValue)
        : value_(initialValue)
    {
    }

    SerializedString::operator std::string() const {
        return value_;
    }

    bool SerializedString::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        if (!SerializedUnsignedInteger((unsigned int)value_.length()).Serialize(file)) {
            return false;
        }
        return file->Write(value_.c_str(), value_.length()) == value_.length();
    }

    bool SerializedString::Deserialize(SystemAbstractions::IFile* file) {
        SerializedUnsignedInteger length;
        if (!length.Deserialize(file)) {
            return false;
        }
        if (length == 0) {
            value_.clear();
            return true;
        }
        SystemAbstractions::IFile::Buffer buffer(length);
        if (file->Read(buffer) != length) {
            return false;
        }
        value_.assign(reinterpret_cast< const char* >(&buffer[0]), length);
        return true;
    }

    std::string SerializedString::Render() const {
        std::string rendering = "\"";
        rendering += value_;
        rendering += "\"";
        return rendering;
    }

    bool SerializedString::Parse(std::string rendering) {
        rendering = StringExtensions::Trim(rendering);
        if (
            (rendering.length() < 2)
            || (rendering[0] != '"')
            || (rendering[rendering.length() - 1] != '"')
        ) {
            return false;
        }
        value_ = rendering.substr(1, rendering.length() - 2);
        return true;
    }

    bool SerializedString::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedString*)other;
        return value_ == otherImpl->value_;
    }

}
