/**
 * @file SerializedInteger.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedInteger class.
 *
 * Copyright (c) 2013-2017 by Richard Walters
 */

#include <Serialization/SerializedInteger.hpp>
#include <stack>
#include <stdio.h>
#include <SystemAbstractions/StringExtensions.hpp>

namespace Serialization {

    SerializedInteger::SerializedInteger(int initialValue)
        : value_(initialValue)
    {
    }

    SerializedInteger::operator int() const {
        return value_;
    }

    bool SerializedInteger::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        int value = (value_ < 0 ? -value_ : value_);
        std::stack< uint8_t > bytes;
        while ((value & 0xFFFFFFC0) != 0) {
            uint8_t lsb = (uint8_t)(value & 0x7F);
            value = ((value >> 7) & 0x01FFFFFF);
            bytes.push(lsb);
        }
        uint8_t lsb = (uint8_t)(value & 0x3F);
        if (value_ < 0) {
            lsb |= 0x40;
        }
        bytes.push(lsb);
        SystemAbstractions::IFile::Buffer serialization;
        while (!bytes.empty()) {
            uint8_t nextByte = bytes.top();
            bytes.pop();
            if (!bytes.empty()) {
                nextByte |= 0x80;
            }
            serialization.push_back(nextByte);
        }
        return file->Write(serialization) == serialization.size();
    }

    bool SerializedInteger::Deserialize(SystemAbstractions::IFile* file) {
        SystemAbstractions::IFile::Buffer nextByte(1);
        value_ = 0;
        if (file->Read(nextByte, 1) != 1) {
            return false;
        }
        bool more = ((nextByte[0] & 0x80) != 0);
        bool negative = ((nextByte[0] & 0x40) != 0);
        value_ = (nextByte[0] & 0x3F);
        while (more) {
            if (file->Read(nextByte, 1) != 1) {
                return false;
            }
            more = ((nextByte[0] & 0x80) != 0);
            value_ <<= 7;
            value_ += (nextByte[0] & 0x7F);
        }
        if (negative) {
            value_ = -value_;
        }
        return true;
    }

    std::string SerializedInteger::Render() const {
        return SystemAbstractions::sprintf("%+d", value_);
    }

    bool SerializedInteger::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (rendering.find_first_of(" \r\n\t") != std::string::npos) {
            return false;
        }
        return (sscanf(rendering.c_str(), "%d", &value_) == 1);
    }

    bool SerializedInteger::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedInteger*)other;
        return value_ == otherImpl->value_;
    }

}
