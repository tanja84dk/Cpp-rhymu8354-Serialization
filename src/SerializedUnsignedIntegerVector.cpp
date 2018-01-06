/**
 * @file SerializedUnsignedIntegerVector.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedUnsignedIntegerVector class.
 *
 * Copyright (c) 2013-2017 by Richard Walters
 */

#include <Serialization/SerializedUnsignedInteger.hpp>
#include <Serialization/SerializedUnsignedIntegerVector.hpp>
#include <stack>
#include <SystemAbstractions/StringExtensions.hpp>

namespace Serialization {

    SerializedUnsignedIntegerVector::SerializedUnsignedIntegerVector(std::vector< unsigned int > initialValue)
        : value_(initialValue)
    {
    }

    SerializedUnsignedIntegerVector::operator const std::vector< unsigned int >&() const {
        return value_;
    }

    SerializedUnsignedIntegerVector::operator std::vector< unsigned int >&() {
        return value_;
    }

    bool SerializedUnsignedIntegerVector::Serialize(SystemAbstractions::IFile* file, unsigned int serializationVersion) const {
        Serialization::SerializedUnsignedInteger sizeSerialized(value_.size());
        if (!sizeSerialized.Serialize(file)) {
            return false;
        }
        for (auto element: value_) {
            Serialization::SerializedUnsignedInteger elementSerialized(element);
            if (!elementSerialized.Serialize(file)) {
                return false;
            }
        }
        return true;
    }

    bool SerializedUnsignedIntegerVector::Deserialize(SystemAbstractions::IFile* file) {
        Serialization::SerializedUnsignedInteger sizeSerialized;
        if (!sizeSerialized.Deserialize(file)) {
            return false;
        }
        value_.resize(sizeSerialized);
        for (auto& element: value_) {
            Serialization::SerializedUnsignedInteger elementSerialized;
            if (!elementSerialized.Deserialize(file)) {
                return false;
            }
            element = (unsigned int)elementSerialized;
        }
        return true;
    }

    std::string SerializedUnsignedIntegerVector::Render() const {
        std::string rendering = "<";
        std::vector< std::string > elementRenderings;
        size_t totalElementRenderings = 0;
        for (auto element: value_) {
            Serialization::SerializedUnsignedInteger elementSerialized(element);
            const std::string elementRendering = elementSerialized.Render();
            auto elementRenderingLength = elementRendering.length();
            if (totalElementRenderings > 0) {
                totalElementRenderings += 2;
            }
            totalElementRenderings += elementRenderingLength;
            elementRenderings.push_back(elementRendering);
        }
        if (totalElementRenderings >= 70) {
            bool first = true;
            for (auto elementRendering: elementRenderings) {
                if (!first) {
                    rendering += ",";
                }
                rendering += "\r\n    ";
                rendering += elementRendering;
                first = false;
            }
            rendering += "\r\n";
        } else {
            bool first = true;
            for (auto elementRendering: elementRenderings) {
                if (!first) {
                    rendering += ", ";
                }
                rendering += elementRendering;
                first = false;
            }
        }
        rendering += ">";
        return rendering;
    }

    bool SerializedUnsignedIntegerVector::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (
            (rendering.length() < 2)
            || (rendering[0] != '<')
            || (rendering[rendering.length() - 1] != '>')
        ) {
            return false;
        }
        value_.clear();
        size_t i = 1;
        while (i < rendering.length() - 1) {
            size_t j = rendering.find_first_of(",>", i);
            unsigned int element = 0;
            std::string elementRendering = SystemAbstractions::Trim(rendering.substr(i, j - i));
            if (elementRendering.find_first_of(" \r\n\t") != std::string::npos) {
                return false;
            }
            if (
                sscanf(
                    elementRendering.c_str(),
                    "%u",
                    &element
                ) != 1
            ) {
                return false;
            }
            value_.push_back(element);
            if (rendering[j] == ',') {
                i = j + 1;
            } else {
                i = j;
            }
        }
        return true;
    }

    bool SerializedUnsignedIntegerVector::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedUnsignedIntegerVector*)other;
        return value_ == otherImpl->value_;
    }

}
