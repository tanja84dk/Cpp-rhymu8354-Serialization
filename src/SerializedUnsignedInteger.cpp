/**
 * @file SerializedUnsignedInteger.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedUnsignedInteger class.
 *
 * Copyright (c) 2013-2017 by Richard Walters
 */

#include <inttypes.h>
#include <Serialization/SerializedUnsignedInteger.hpp>
#include <stack>
#include <SystemAbstractions/StringExtensions.hpp>

namespace Serialization {

    SerializedUnsignedInteger::SerializedUnsignedInteger(uint64_t initialValue)
        : value_(initialValue)
    {
    }

    SerializedUnsignedInteger::operator uint64_t() const {
        return value_;
    }

    bool SerializedUnsignedInteger::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        uint64_t value = value_;
        std::stack< uint8_t > bytes;
        do {
            uint8_t lsb = (uint8_t)(value & 0x7F);
            value = ((value >> 7) & 0x01FFFFFFFFFFFFFFLL);
            bytes.push(lsb);
        } while (value > 0);
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

    bool SerializedUnsignedInteger::Deserialize(SystemAbstractions::IFile* file) {
        SystemAbstractions::IFile::Buffer nextByte(1);
        value_ = 0;
        bool more = true;
        while (more) {
            if (file->Read(nextByte, 1) != 1) {
                return false;
            }
            more = ((nextByte[0] & 0x80) != 0);
            value_ <<= 7;
            value_ += (nextByte[0] & 0x7F);
        }
        return true;
    }

    std::string SerializedUnsignedInteger::Render() const {
        return SystemAbstractions::sprintf("%" PRIu64, value_);
    }

    bool SerializedUnsignedInteger::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (rendering.find_first_of(" \r\n\t") != std::string::npos) {
            return false;
        }
        return (sscanf(rendering.c_str(), "%" SCNu64, &value_) == 1);
    }

    bool SerializedUnsignedInteger::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedUnsignedInteger*)other;
        return value_ == otherImpl->value_;
    }

}
