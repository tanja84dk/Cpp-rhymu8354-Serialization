/**
 * @file SerializedCollection.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedCollection class.
 *
 * Copyright (c) 2015-2017 by Richard Walters
 */

#include <Serialization/SerializedCollection.hpp>
#include <Serialization/SerializedObject.hpp>
#include <Serialization/SerializedString.hpp>
#include <Serialization/SerializedUnsignedInteger.hpp>
#include <SystemAbstractions/StringExtensions.hpp>

namespace Serialization {

    SerializedCollection::SerializedCollection(std::map< std::string, SerializedObject > initialValue)
        : collection_(initialValue)
    {
    }

    SerializedCollection::operator std::map< std::string, SerializedObject >() const {
        return collection_;
    }

    bool SerializedCollection::HasObject(const std::string& name) const {
        return (collection_.find(name) != collection_.end());
    }

    SerializedObject SerializedCollection::GetObjectWrapper(const std::string& name) const {
        auto serializedObject(collection_.find(name));
        if (serializedObject == collection_.end()) {
            return SerializedObject();
        } else {
            return serializedObject->second;
        }
    }

    void SerializedCollection::SetObject(const std::string& name, SerializedObject value) {
        collection_[name] = value;
    }

    void SerializedCollection::RemoveObject(const std::string& name) {
        auto wrapper(collection_.find(name));
        if (wrapper == collection_.end()) {
            return;
        }
        collection_.erase(wrapper);
    }

    bool SerializedCollection::GetFirstObject(std::string& name, SerializedObject& value) const {
        auto it = collection_.begin();
        if (it == collection_.end()) {
            return false;
        } else {
            name = it->first;
            value = it->second;
            return true;
        }
    }

    bool SerializedCollection::GetNextObject(std::string& name, SerializedObject& value) const {
        auto it = collection_.find(name);
        if (it == collection_.end()) {
            return false;
        }
        ++it;
        if (it == collection_.end()) {
            return false;
        } else {
            name = it->first;
            value = it->second;
            return true;
        }
    }

    size_t SerializedCollection::GetSize() const {
        return collection_.size();
    }

    bool SerializedCollection::Serialize(
        SystemAbstractions::IFile* file,
        unsigned int serializationVersion
    ) const {
        SerializedUnsignedInteger numSerializedObjects((unsigned int)collection_.size());
        if (!numSerializedObjects.Serialize(file)) {
            return false;
        }
        for (auto serializedObject: collection_) {
            SerializedString serializedKey(serializedObject.first);
            if (!serializedKey.Serialize(file)) {
                return false;
            }
            if (!serializedObject.second.Serialize(file)) {
                return false;
            }
        }
        return true;
    }

    bool SerializedCollection::Deserialize(SystemAbstractions::IFile* file) {
        collection_.clear();
        SerializedUnsignedInteger numSerializedObjects;
        if (!numSerializedObjects.Deserialize(file)) {
            return false;
        }
        for (unsigned int i = 0; i < numSerializedObjects; ++i) {
            SerializedString serializedKey;
            if (!serializedKey.Deserialize(file)) {
                return false;
            }
            SerializedObject serializedObject;
            if (!serializedObject.Deserialize(file)) {
                return false;
            }
            SetObject(serializedKey, serializedObject);
        }
        return true;
    }

    std::string SerializedCollection::Render() const {
        if (collection_.empty()) {
            return "{}";
        }
        std::string rendering = "{\r\n";
        bool first = true;
        for (auto serializedObject: collection_) {
            if (!first) {
                rendering += ",\r\n";
            }
            rendering += "    " + serializedObject.first;
            rendering += ": ";
            rendering += SystemAbstractions::Indent(serializedObject.second.Render(), 4);
            first = false;
        }
        rendering += "\r\n}";
        return rendering;
    }

    bool SerializedCollection::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (
            (rendering.length() < 2)
            || (rendering[0] != '{')
            || (rendering[rendering.length() - 1] != '}')
        ) {
            return false;
        }
        collection_.clear();
        size_t i = 1;
        const size_t end = rendering.length() - 1;
        while (i < end) {
            size_t j = i;
            while (
                (j < rendering.length() - 1)
                && (rendering[j] != ':')
            ) {
                ++j;
            }
            const std::string key(SystemAbstractions::Trim(rendering.substr(i, j - i)));
            if (rendering[j] == ':') {
                i = j + 1;
            } else {
                i = j;
                if (!key.empty()) {
                    return false;
                }
            }
            if (key.empty()) {
                continue;
            }
            const std::string element = SystemAbstractions::ParseElement(rendering, i, end);
            SerializedObject obj;
            if (obj.Parse(SystemAbstractions::Trim(element))) {
                collection_[key] = obj;
            } else {
                return false;
            }
            j = i + element.length();
            if (rendering[j] == ',') {
                i = j + 1;
            } else {
                i = j;
            }
        }
        return true;
    }

    bool SerializedCollection::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedCollection*)other;
        return collection_ == otherImpl->collection_;
    }

}
