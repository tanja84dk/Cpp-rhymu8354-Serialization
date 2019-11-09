/**
 * @file SerializedBoolean.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedBoolean class.
 *
 * Copyright (c) 2014-2018 by Richard Walters
 */

#include <Serialization/SerializedBoolean.hpp>
#include <StringExtensions/StringExtensions.hpp>

namespace Serialization {

    SerializedBoolean::SerializedBoolean(bool initialValue)
        : value_(initialValue)
    {
    }

    SerializedBoolean::operator bool() const {
        return value_;
    }

    bool SerializedBoolean::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        uint8_t byte = (value_ ? 1 : 0);
        return file->Write(&byte, 1) == 1;
    }

    bool SerializedBoolean::Deserialize(SystemAbstractions::IFile* file) {
        uint8_t byte;
        if (file->Read(&byte, 1) != 1) {
            return false;
        }
        value_ = (byte != 0);
        return true;
    }

    std::string SerializedBoolean::Render() const {
        if (value_) {
            return "True";
        } else {
            return "False";
        }
    }

    bool SerializedBoolean::Parse(std::string rendering) {
        rendering = StringExtensions::Trim(rendering);
        if (
            (rendering == "true")
            || (rendering == "True")
            || (rendering == "TRUE")
        ) {
            value_ = true;
            return true;
        }
        if (
            (rendering == "false")
            || (rendering == "False")
            || (rendering == "FALSE")
        ) {
            value_ = false;
            return true;
        }
        return false;
    }

    bool SerializedBoolean::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedBoolean*)other;
        return value_ == otherImpl->value_;
    }

}
