/**
 * @file SerializedObject.cpp
 *
 * This module contains the implementation of the
 * Serialization::SerializedObject class.
 *
 * Copyright (c) 2013-2018 by Richard Walters
 */

#include <Serialization/SerializedObject.hpp>
#include <Serialization/SerializedBoolean.hpp>
#include <Serialization/SerializedCollection.hpp>
#include <Serialization/SerializedDecimal.hpp>
#include <Serialization/SerializedInteger.hpp>
#include <Serialization/SerializedIntegerVector.hpp>
#include <Serialization/SerializedIpAddress.hpp>
#include <Serialization/SerializedString.hpp>
#include <Serialization/SerializedUnsignedInteger.hpp>
#include <Serialization/SerializedUnsignedIntegerVector.hpp>
#include <Serialization/SerializedVector.hpp>
#include <stddef.h>
#include <SystemAbstractions/StringExtensions.hpp>
#include <SystemAbstractions/StringFile.hpp>

namespace {

    // The following are identifier strings for supported object types.
    //
    // IMPORTANT: Changing these will almost certainly break
    //            compatibility with other serialization versions!

    static const std::string TYPE_ID_EMPTY("e");
    static const std::string TYPE_ID_BOOLEAN("b");
    static const std::string TYPE_ID_INTEGER("n");
    static const std::string TYPE_ID_INTEGER_VECTOR("nv");
    static const std::string TYPE_ID_IP_ADDRESS("ip");
    static const std::string TYPE_ID_UNSIGNED_INTEGER("i");
    static const std::string TYPE_ID_UNSIGNED_INTEGER_VECTOR("iv");
    static const std::string TYPE_ID_STRING("s");
    static const std::string TYPE_ID_DECIMAL("d");
    static const std::string TYPE_ID_VECTOR("v");
    static const std::string TYPE_ID_COLLECTION("c");

}

namespace Serialization {

    SerializedObject::~SerializedObject() = default;
    SerializedObject::SerializedObject(const SerializedObject& other) = default;
    SerializedObject::SerializedObject(SerializedObject&& other) = default;
    SerializedObject& SerializedObject::operator=(const SerializedObject& other) = default;
    SerializedObject& SerializedObject::operator=(SerializedObject&& other) = default;

    SerializedObject::SerializedObject() = default;

    SerializedObject::Type SerializedObject::GetType() const {
        return type_;
    }

    bool SerializedObject::operator==(const SerializedObject& other) const {
        if (type_ != other.type_) {
            return false;
        }
        if (impl_ == nullptr) {
            if (other.impl_ != nullptr) {
                return false;
            }
        } else if (other.impl_ == nullptr) {
            if (impl_ == nullptr) {
                return false;
            }
        }
        if (type_ == Type::Empty) {
            return true;
        } else {
            return impl_->IsEqualTo(other.impl_.get());
        }
    }

    bool SerializedObject::operator!=(const SerializedObject& other) const {
        return !(*this == other);
    }

    bool SerializedObject::Serialize(
        SystemAbstractions::IFile* file,
        unsigned int serializationVersion
    ) const {
        // Get starting position in the file, in case we need to back up
        // in case of error.
        const uint64_t startingPosition = file->GetPosition();

        // Select the type ID string corresponding to the object type.
        SerializedString typeId;
        switch (type_) {
        case Type::Empty:
            typeId = TYPE_ID_EMPTY;
            break;

        case Type::Boolean:
            typeId = TYPE_ID_BOOLEAN;
            break;

        case Type::IpAddress:
            typeId = TYPE_ID_IP_ADDRESS;
            break;

        case Type::UnsignedInteger:
            typeId = TYPE_ID_UNSIGNED_INTEGER;
            break;

        case Type::UnsignedIntegerVector:
            typeId = TYPE_ID_UNSIGNED_INTEGER_VECTOR;
            break;

        case Type::Integer:
            typeId = TYPE_ID_INTEGER;
            break;

        case Type::IntegerVector:
            typeId = TYPE_ID_INTEGER_VECTOR;
            break;

        case Type::String:
            typeId = TYPE_ID_STRING;
            break;

        case Type::Decimal:
            typeId = TYPE_ID_DECIMAL;
            break;

        case Type::Vector:
            typeId = TYPE_ID_VECTOR;
            break;

        case Type::Collection:
            typeId = TYPE_ID_COLLECTION;
            break;

        default:
            return false;
        }

        // Serialize the object type and data to a string.
        SystemAbstractions::StringFile objectStringFile;
        if (!typeId.Serialize(&objectStringFile)) {
            return false;
        }
        if (impl_ != nullptr) {
            if (!impl_->Serialize(&objectStringFile)) {
                return false;
            }
        }
        std::string objectString(objectStringFile);

        // Serialize the object string length followed by the object string.
        SerializedUnsignedInteger length((unsigned int)objectString.length());
        if (!length.Serialize(file)) {
            file->SetPosition(startingPosition);
            return false;
        }
        if (file->Write(objectString.data(), length) != length) {
            file->SetPosition(startingPosition);
            return false;
        }
        return true;
    }

    bool SerializedObject::Deserialize(SystemAbstractions::IFile* file) {
        // Deserialize object data length.
        SerializedUnsignedInteger length;
        if (!length.Deserialize(file)) {
            return false;
        }
        const uint64_t startingPosition = file->GetPosition();

        // Deserialize object type.
        SerializedString typeId;
        if (!typeId.Deserialize(file)) {
            return false;
        }
        const std::string typeIdString(typeId);

        // If the object type is known, deserialize it.
        ISerializedObject* newObject = NULL;
        Type type = Type::Empty;
        if (typeIdString == TYPE_ID_EMPTY) {
            type_ = Type::Empty;
            impl_ = nullptr;
            return true;
        } else if (typeIdString == TYPE_ID_BOOLEAN) {
            SerializedBoolean* newBooleanObject = new SerializedBoolean();
            if (newBooleanObject->Deserialize(file)) {
                newObject = newBooleanObject;
                type = Type::Boolean;
            } else {
                delete newBooleanObject;
            }
        } else if (typeIdString == TYPE_ID_IP_ADDRESS) {
            SerializedIpAddress* newIpAddressObject = new SerializedIpAddress();
            if (newIpAddressObject->Deserialize(file)) {
                newObject = newIpAddressObject;
                type = Type::IpAddress;
            } else {
                delete newIpAddressObject;
            }
        } else if (typeIdString == TYPE_ID_UNSIGNED_INTEGER) {
            SerializedUnsignedInteger* newUnsignedIntegerObject = new SerializedUnsignedInteger();
            if (newUnsignedIntegerObject->Deserialize(file)) {
                newObject = newUnsignedIntegerObject;
                type = Type::UnsignedInteger;
            } else {
                delete newUnsignedIntegerObject;
            }
        } else if (typeIdString == TYPE_ID_UNSIGNED_INTEGER_VECTOR) {
            SerializedUnsignedIntegerVector* newUnsignedIntegerVectorObject = new SerializedUnsignedIntegerVector();
            if (newUnsignedIntegerVectorObject->Deserialize(file)) {
                newObject = newUnsignedIntegerVectorObject;
                type = Type::UnsignedIntegerVector;
            } else {
                delete newUnsignedIntegerVectorObject;
            }
        } else if (typeIdString == TYPE_ID_INTEGER) {
            SerializedInteger* newIntegerObject = new SerializedInteger();
            if (newIntegerObject->Deserialize(file)) {
                newObject = newIntegerObject;
                type = Type::Integer;
            } else {
                delete newIntegerObject;
            }
        } else if (typeIdString == TYPE_ID_INTEGER_VECTOR) {
            SerializedIntegerVector* newIntegerVectorObject = new SerializedIntegerVector();
            if (newIntegerVectorObject->Deserialize(file)) {
                newObject = newIntegerVectorObject;
                type = Type::IntegerVector;
            } else {
                delete newIntegerVectorObject;
            }
        } else if (typeIdString == TYPE_ID_STRING) {
            SerializedString* newStringObject = new SerializedString();
            if (newStringObject->Deserialize(file)) {
                newObject = newStringObject;
                type = Type::String;
            } else {
                delete newStringObject;
            }
        } else if (typeIdString == TYPE_ID_DECIMAL) {
            SerializedDecimal* newDecimalObject = new SerializedDecimal();
            if (newDecimalObject->Deserialize(file)) {
                newObject = newDecimalObject;
                type = Type::Decimal;
            } else {
                delete newDecimalObject;
            }
        } else if (typeIdString == TYPE_ID_VECTOR) {
            SerializedVector* newVectorObject = new SerializedVector();
            if (newVectorObject->Deserialize(file)) {
                newObject = newVectorObject;
                type = Type::Vector;
            } else {
                delete newVectorObject;
            }
        } else if (typeIdString == TYPE_ID_COLLECTION) {
            SerializedCollection* newCollectionObject = new SerializedCollection();
            if (newCollectionObject->Deserialize(file)) {
                newObject = newCollectionObject;
                type = Type::Collection;
            } else {
                delete newCollectionObject;
            }
        }

        // For known types, verify the length was correct.
        if (type != Type::Empty) {
            if (file->GetPosition() != startingPosition + length) {
                delete newObject;
                type = Type::Empty;
            }
        }

        // If unsuccessful, use the object data length to skip beyond
        // this object, to aid in recovering data beyond the object.
        if (type == Type::Empty) {
            file->SetPosition(startingPosition + length);
            return false;
        }

        // Release any old object we had, and hold onto the new object instead.
        type_ = type;
        impl_ = std::shared_ptr< ISerializedObject >(newObject);
        return true;
    }

    std::string SerializedObject::Render() const {
        if (impl_ == nullptr) {
            return "Empty";
        } else {
            return impl_->Render();
        }
    }

    bool SerializedObject::Parse(std::string rendering) {
        rendering = SystemAbstractions::Trim(rendering);
        if (rendering.empty()) {
            return false;
        } else if (
            (rendering == "empty")
            || (rendering == "Empty")
            || (rendering == "EMPTY")
        ) {
            type_ = Type::Empty;
            impl_.reset();
            return true;
        } else if (rendering[0] == '"') {
            type_ = Type::String;
            impl_.reset(new SerializedString());
            return impl_->Parse(rendering);
        } else if (rendering[0] == '[') {
            type_ = Type::Vector;
            impl_.reset(new SerializedVector());
            return impl_->Parse(rendering);
        } else if (rendering[0] == '{') {
            type_ = Type::Collection;
            impl_.reset(new SerializedCollection());
            return impl_->Parse(rendering);
        } else if (rendering[0] == '(') {
            type_ = Type::IntegerVector;
            impl_.reset(new SerializedIntegerVector());
            return impl_->Parse(rendering);
        } else if (rendering[0] == '<') {
            type_ = Type::UnsignedIntegerVector;
            impl_.reset(new SerializedUnsignedIntegerVector());
            return impl_->Parse(rendering);
        } else if (
            (rendering[0] == '-')
            || (rendering[0] == '+')
        ) {
            type_ = Type::Integer;
            impl_.reset(new SerializedInteger());
            return impl_->Parse(rendering);
        } else if (rendering.find('.', 0) != std::string::npos) {
            auto pos1 = rendering.find('.', 0);
            auto pos2 = rendering.find('.', pos1 + 1);
            if (pos2 == std::string::npos) {
                type_ = Type::Decimal;
                impl_.reset(new SerializedDecimal());
                return impl_->Parse(rendering);
            } else {
                type_ = Type::IpAddress;
                impl_.reset(new SerializedIpAddress());
                return impl_->Parse(rendering);
            }
        } else if (rendering.find_first_not_of("0123456789") == std::string::npos) {
            type_ = Type::UnsignedInteger;
            impl_.reset(new SerializedUnsignedInteger());
            return impl_->Parse(rendering);
        } else {
            type_ = Type::Boolean;
            impl_.reset(new SerializedBoolean());
            return impl_->Parse(rendering);
        }
    }

    bool SerializedObject::IsEqualTo(const ISerializedObject* other) const {
        auto otherImpl = (const SerializedObject*)other;
        return (*this == *otherImpl);
    }

}
