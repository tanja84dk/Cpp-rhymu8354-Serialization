/**
 * @file SerializedVector.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedVector class.
 *
 * Copyright (c) 2014-2018 by Richard Walters
 */

#include <Serialization/SerializedUnsignedInteger.hpp>
#include <Serialization/SerializedVector.hpp>
#include <SystemAbstractions/StringExtensions.hpp>
#include <SystemAbstractions/StringFile.hpp>

namespace Serialization {

    SerializedVector::SerializedVector(std::vector< SerializedObject > initialValue)
        : value_(initialValue)
    {
    }

    SerializedVector::operator std::vector< SerializedObject >() const {
        return value_;
    }

    size_t SerializedVector::GetLength() const {
        return value_.size();
    }

    const SerializedObject& SerializedVector::operator[](size_t index) const {
        return value_[index];
    }

    SerializedObject& SerializedVector::operator[](size_t index) {
        return value_[index];
    }

    void SerializedVector::Insert(SerializedObject value, size_t index) {
        (void)value_.insert(value_.begin() + index, value);
    }

    void SerializedVector::Append(SerializedObject value) {
        value_.push_back(value);
    }

    void SerializedVector::Delete(size_t index) {
        (void)value_.erase(value_.begin() + index);
    }

    void SerializedVector::Add(SerializedObject value) {
        auto valueIterator = value_.begin();
        SystemAbstractions::StringFile valueSerialized;
        if (!value.Serialize(&valueSerialized)) {
            return;
        }
        const std::string valueString(valueSerialized);
        while (valueIterator != value_.end()) {
            SystemAbstractions::StringFile otherValueSerialized;
            if (!valueIterator->Serialize(&otherValueSerialized)) {
                continue;
            }
            if (std::string(otherValueSerialized) == valueString) {
                break;
            }
            ++valueIterator;
        }
        if (valueIterator == value_.end()) {
            value_.push_back(value);
        }
    }

    void SerializedVector::Remove(SerializedObject value) {
        auto valueIterator = value_.begin();
        SystemAbstractions::StringFile valueSerialized;
        if (!value.Serialize(&valueSerialized)) {
            return;
        }
        const std::string valueString(valueSerialized);
        while (valueIterator != value_.end()) {
            SystemAbstractions::StringFile otherValueSerialized;
            if (!valueIterator->Serialize(&otherValueSerialized)) {
                continue;
            }
            if (std::string(otherValueSerialized) == valueString) {
                valueIterator = value_.erase(valueIterator);
            } else {
                ++valueIterator;
            }
        }
    }

    void SerializedVector::Replace(size_t index, SerializedObject value) {
        value_[index] = value;
    }

    bool SerializedVector::Serialize(
        SystemAbstractions::IFile* file,
        unsigned int serializationVersion
    ) const {
        SerializedUnsignedInteger numSerializedObjects((unsigned int)value_.size());
        if (!numSerializedObjects.Serialize(file)) {
            return false;
        }
        for (auto serializedObject: value_) {
            if (!serializedObject.Serialize(file)) {
                return false;
            }
        }
        return true;
    }

    bool SerializedVector::Deserialize(SystemAbstractions::IFile* file) {
        SerializedUnsignedInteger numSerializedObjects;
        if (!numSerializedObjects.Deserialize(file)) {
            return false;
        }
        value_.resize(numSerializedObjects);
        for (unsigned int i = 0; i < numSerializedObjects; ++i) {
            if (!value_[i].Deserialize(file)) {
                return false;
            }
        }
        return true;
    }

    std::string SerializedVector::Render() const {
        std::string rendering = "[";
        std::vector< std::string > componentRenderings;
        size_t totalComponentRenderings = 0;
        for (auto component: value_) {
            const std::string componentRendering = component.Render();
            auto componentRenderingLength = componentRendering.length();
            if (totalComponentRenderings > 0) {
                totalComponentRenderings += 2;
            }
            totalComponentRenderings += componentRenderingLength;
            componentRenderings.push_back(componentRendering);
        }
        if (totalComponentRenderings >= 70) {
            bool first = true;
            for (const auto& componentRendering: componentRenderings) {
                if (!first) {
                    rendering += ",";
                }
                rendering += "\r\n    " + SystemAbstractions::Indent(componentRendering, 4);
                first = false;
            }
            rendering += "\r\n";
        } else {
            bool first = true;
            for (const auto& componentRendering: componentRenderings) {
                if (!first) {
                    rendering += ", ";
                }
                rendering += componentRendering;
                first = false;
            }
        }
        rendering += "]";
        return rendering;
    }

    bool SerializedVector::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (
            (rendering.length() < 2)
            || (rendering[0] != '[')
            || (rendering[rendering.length() - 1] != ']')
        ) {
            return false;
        }
        value_.clear();
        size_t i = 1;
        const size_t end = rendering.length() - 1;
        while (i < end) {
            const std::string element = SystemAbstractions::ParseElement(rendering, i, end);
            SerializedObject obj;
            if (obj.Parse(SystemAbstractions::Trim(element))) {
                value_.push_back(obj);
            } else {
                return false;
            }
            const size_t j = i + element.length();
            if (rendering[j] == ',') {
                i = j + 1;
            } else {
                i = j;
            }
        }
        return true;
    }

    bool SerializedVector::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedVector*)other;
        return value_ == otherImpl->value_;
    }

}
