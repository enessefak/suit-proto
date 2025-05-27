// @generated
impl serde::Serialize for AddTagToProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_id.is_empty() {
            len += 1;
        }
        if !self.tag_key.is_empty() {
            len += 1;
        }
        if !self.tag_value_code.is_empty() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.AddTagToProductRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.tag_key.is_empty() {
            struct_ser.serialize_field("tagKey", &self.tag_key)?;
        }
        if !self.tag_value_code.is_empty() {
            struct_ser.serialize_field("tagValueCode", &self.tag_value_code)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddTagToProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "tag_key",
            "tagKey",
            "tag_value_code",
            "tagValueCode",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            TagKey,
            TagValueCode,
            Translations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "tagKey" | "tag_key" => Ok(GeneratedField::TagKey),
                            "tagValueCode" | "tag_value_code" => Ok(GeneratedField::TagValueCode),
                            "translations" => Ok(GeneratedField::Translations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddTagToProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.AddTagToProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddTagToProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut tag_key__ = None;
                let mut tag_value_code__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagKey => {
                            if tag_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagKey"));
                            }
                            tag_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagValueCode => {
                            if tag_value_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagValueCode"));
                            }
                            tag_value_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddTagToProductRequest {
                    product_id: product_id__.unwrap_or_default(),
                    tag_key: tag_key__.unwrap_or_default(),
                    tag_value_code: tag_value_code__.unwrap_or_default(),
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.AddTagToProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignOptionValueToVariantRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.variant_id.is_empty() {
            len += 1;
        }
        if !self.option_id.is_empty() {
            len += 1;
        }
        if !self.value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.AssignOptionValueToVariantRequest", len)?;
        if !self.variant_id.is_empty() {
            struct_ser.serialize_field("variantId", &self.variant_id)?;
        }
        if !self.option_id.is_empty() {
            struct_ser.serialize_field("optionId", &self.option_id)?;
        }
        if !self.value_id.is_empty() {
            struct_ser.serialize_field("valueId", &self.value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignOptionValueToVariantRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "variant_id",
            "variantId",
            "option_id",
            "optionId",
            "value_id",
            "valueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VariantId,
            OptionId,
            ValueId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "optionId" | "option_id" => Ok(GeneratedField::OptionId),
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignOptionValueToVariantRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.AssignOptionValueToVariantRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignOptionValueToVariantRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut variant_id__ = None;
                let mut option_id__ = None;
                let mut value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionId => {
                            if option_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionId"));
                            }
                            option_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueId => {
                            if value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AssignOptionValueToVariantRequest {
                    variant_id: variant_id__.unwrap_or_default(),
                    option_id: option_id__.unwrap_or_default(),
                    value_id: value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.AssignOptionValueToVariantRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignedOptionValueInVariant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.option_id.is_empty() {
            len += 1;
        }
        if !self.option_code.is_empty() {
            len += 1;
        }
        if !self.option_name.is_empty() {
            len += 1;
        }
        if !self.value_id.is_empty() {
            len += 1;
        }
        if !self.value_code.is_empty() {
            len += 1;
        }
        if !self.value_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.AssignedOptionValueInVariant", len)?;
        if !self.option_id.is_empty() {
            struct_ser.serialize_field("optionId", &self.option_id)?;
        }
        if !self.option_code.is_empty() {
            struct_ser.serialize_field("optionCode", &self.option_code)?;
        }
        if !self.option_name.is_empty() {
            struct_ser.serialize_field("optionName", &self.option_name)?;
        }
        if !self.value_id.is_empty() {
            struct_ser.serialize_field("valueId", &self.value_id)?;
        }
        if !self.value_code.is_empty() {
            struct_ser.serialize_field("valueCode", &self.value_code)?;
        }
        if !self.value_name.is_empty() {
            struct_ser.serialize_field("valueName", &self.value_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignedOptionValueInVariant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "option_id",
            "optionId",
            "option_code",
            "optionCode",
            "option_name",
            "optionName",
            "value_id",
            "valueId",
            "value_code",
            "valueCode",
            "value_name",
            "valueName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionId,
            OptionCode,
            OptionName,
            ValueId,
            ValueCode,
            ValueName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "optionId" | "option_id" => Ok(GeneratedField::OptionId),
                            "optionCode" | "option_code" => Ok(GeneratedField::OptionCode),
                            "optionName" | "option_name" => Ok(GeneratedField::OptionName),
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            "valueCode" | "value_code" => Ok(GeneratedField::ValueCode),
                            "valueName" | "value_name" => Ok(GeneratedField::ValueName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignedOptionValueInVariant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.AssignedOptionValueInVariant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignedOptionValueInVariant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut option_id__ = None;
                let mut option_code__ = None;
                let mut option_name__ = None;
                let mut value_id__ = None;
                let mut value_code__ = None;
                let mut value_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OptionId => {
                            if option_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionId"));
                            }
                            option_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionCode => {
                            if option_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionCode"));
                            }
                            option_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionName => {
                            if option_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionName"));
                            }
                            option_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueId => {
                            if value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueCode => {
                            if value_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCode"));
                            }
                            value_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueName => {
                            if value_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueName"));
                            }
                            value_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AssignedOptionValueInVariant {
                    option_id: option_id__.unwrap_or_default(),
                    option_code: option_code__.unwrap_or_default(),
                    option_name: option_name__.unwrap_or_default(),
                    value_id: value_id__.unwrap_or_default(),
                    value_code: value_code__.unwrap_or_default(),
                    value_name: value_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.AssignedOptionValueInVariant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProductOptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_id.is_empty() {
            len += 1;
        }
        if !self.option_code.is_empty() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.CreateProductOptionRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.option_code.is_empty() {
            struct_ser.serialize_field("optionCode", &self.option_code)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProductOptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "option_code",
            "optionCode",
            "sort_order",
            "sortOrder",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            OptionCode,
            SortOrder,
            Translations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "optionCode" | "option_code" => Ok(GeneratedField::OptionCode),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "translations" => Ok(GeneratedField::Translations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProductOptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.CreateProductOptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProductOptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut option_code__ = None;
                let mut sort_order__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionCode => {
                            if option_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionCode"));
                            }
                            option_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProductOptionRequest {
                    product_id: product_id__.unwrap_or_default(),
                    option_code: option_code__.unwrap_or_default(),
                    sort_order: sort_order__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.CreateProductOptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProductOptionValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.option_id.is_empty() {
            len += 1;
        }
        if !self.value_code.is_empty() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        if self.additional_data.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.CreateProductOptionValueRequest", len)?;
        if !self.option_id.is_empty() {
            struct_ser.serialize_field("optionId", &self.option_id)?;
        }
        if !self.value_code.is_empty() {
            struct_ser.serialize_field("valueCode", &self.value_code)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        if let Some(v) = self.additional_data.as_ref() {
            struct_ser.serialize_field("additionalData", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProductOptionValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "option_id",
            "optionId",
            "value_code",
            "valueCode",
            "sort_order",
            "sortOrder",
            "additional_data",
            "additionalData",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionId,
            ValueCode,
            SortOrder,
            AdditionalData,
            Translations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "optionId" | "option_id" => Ok(GeneratedField::OptionId),
                            "valueCode" | "value_code" => Ok(GeneratedField::ValueCode),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "additionalData" | "additional_data" => Ok(GeneratedField::AdditionalData),
                            "translations" => Ok(GeneratedField::Translations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProductOptionValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.CreateProductOptionValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProductOptionValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut option_id__ = None;
                let mut value_code__ = None;
                let mut sort_order__ = None;
                let mut additional_data__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OptionId => {
                            if option_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionId"));
                            }
                            option_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueCode => {
                            if value_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCode"));
                            }
                            value_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AdditionalData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalData"));
                            }
                            additional_data__ = map_.next_value()?;
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProductOptionValueRequest {
                    option_id: option_id__.unwrap_or_default(),
                    value_code: value_code__.unwrap_or_default(),
                    sort_order: sort_order__,
                    additional_data: additional_data__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.CreateProductOptionValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.product_type.is_empty() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        if !self.initial_category_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.CreateProductRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.product_type.is_empty() {
            struct_ser.serialize_field("productType", &self.product_type)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        if !self.initial_category_ids.is_empty() {
            struct_ser.serialize_field("initialCategoryIds", &self.initial_category_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "product_type",
            "productType",
            "is_active",
            "isActive",
            "translations",
            "initial_category_ids",
            "initialCategoryIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            ProductType,
            IsActive,
            Translations,
            InitialCategoryIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "productType" | "product_type" => Ok(GeneratedField::ProductType),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "translations" => Ok(GeneratedField::Translations),
                            "initialCategoryIds" | "initial_category_ids" => Ok(GeneratedField::InitialCategoryIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.CreateProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut product_type__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                let mut initial_category_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductType => {
                            if product_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productType"));
                            }
                            product_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitialCategoryIds => {
                            if initial_category_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialCategoryIds"));
                            }
                            initial_category_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProductRequest {
                    code: code__.unwrap_or_default(),
                    product_type: product_type__.unwrap_or_default(),
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                    initial_category_ids: initial_category_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.CreateProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProductUnitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_id.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.barcode.is_some() {
            len += 1;
        }
        if self.is_base_unit {
            len += 1;
        }
        if !self.conversion_to_base_factor.is_empty() {
            len += 1;
        }
        if self.is_sellable.is_some() {
            len += 1;
        }
        if self.is_purchasable.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.CreateProductUnitRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.barcode.as_ref() {
            struct_ser.serialize_field("barcode", v)?;
        }
        if self.is_base_unit {
            struct_ser.serialize_field("isBaseUnit", &self.is_base_unit)?;
        }
        if !self.conversion_to_base_factor.is_empty() {
            struct_ser.serialize_field("conversionToBaseFactor", &self.conversion_to_base_factor)?;
        }
        if let Some(v) = self.is_sellable.as_ref() {
            struct_ser.serialize_field("isSellable", v)?;
        }
        if let Some(v) = self.is_purchasable.as_ref() {
            struct_ser.serialize_field("isPurchasable", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProductUnitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "unit_id",
            "unitId",
            "barcode",
            "is_base_unit",
            "isBaseUnit",
            "conversion_to_base_factor",
            "conversionToBaseFactor",
            "is_sellable",
            "isSellable",
            "is_purchasable",
            "isPurchasable",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            UnitId,
            Barcode,
            IsBaseUnit,
            ConversionToBaseFactor,
            IsSellable,
            IsPurchasable,
            SortOrder,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "barcode" => Ok(GeneratedField::Barcode),
                            "isBaseUnit" | "is_base_unit" => Ok(GeneratedField::IsBaseUnit),
                            "conversionToBaseFactor" | "conversion_to_base_factor" => Ok(GeneratedField::ConversionToBaseFactor),
                            "isSellable" | "is_sellable" => Ok(GeneratedField::IsSellable),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProductUnitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.CreateProductUnitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProductUnitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut unit_id__ = None;
                let mut barcode__ = None;
                let mut is_base_unit__ = None;
                let mut conversion_to_base_factor__ = None;
                let mut is_sellable__ = None;
                let mut is_purchasable__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Barcode => {
                            if barcode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("barcode"));
                            }
                            barcode__ = map_.next_value()?;
                        }
                        GeneratedField::IsBaseUnit => {
                            if is_base_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBaseUnit"));
                            }
                            is_base_unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversionToBaseFactor => {
                            if conversion_to_base_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversionToBaseFactor"));
                            }
                            conversion_to_base_factor__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSellable => {
                            if is_sellable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSellable"));
                            }
                            is_sellable__ = map_.next_value()?;
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = map_.next_value()?;
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(CreateProductUnitRequest {
                    product_id: product_id__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    barcode: barcode__,
                    is_base_unit: is_base_unit__.unwrap_or_default(),
                    conversion_to_base_factor: conversion_to_base_factor__.unwrap_or_default(),
                    is_sellable: is_sellable__,
                    is_purchasable: is_purchasable__,
                    sort_order: sort_order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.CreateProductUnitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProductVariantRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_id.is_empty() {
            len += 1;
        }
        if !self.sku.is_empty() {
            len += 1;
        }
        if self.barcode.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.initial_options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.CreateProductVariantRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.sku.is_empty() {
            struct_ser.serialize_field("sku", &self.sku)?;
        }
        if let Some(v) = self.barcode.as_ref() {
            struct_ser.serialize_field("barcode", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if !self.initial_options.is_empty() {
            struct_ser.serialize_field("initialOptions", &self.initial_options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProductVariantRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "sku",
            "barcode",
            "is_active",
            "isActive",
            "initial_options",
            "initialOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            Sku,
            Barcode,
            IsActive,
            InitialOptions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "sku" => Ok(GeneratedField::Sku),
                            "barcode" => Ok(GeneratedField::Barcode),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "initialOptions" | "initial_options" => Ok(GeneratedField::InitialOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProductVariantRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.CreateProductVariantRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProductVariantRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut sku__ = None;
                let mut barcode__ = None;
                let mut is_active__ = None;
                let mut initial_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sku => {
                            if sku__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sku"));
                            }
                            sku__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Barcode => {
                            if barcode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("barcode"));
                            }
                            barcode__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::InitialOptions => {
                            if initial_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialOptions"));
                            }
                            initial_options__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProductVariantRequest {
                    product_id: product_id__.unwrap_or_default(),
                    sku: sku__.unwrap_or_default(),
                    barcode: barcode__,
                    is_active: is_active__,
                    initial_options: initial_options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.CreateProductVariantRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.locale_input.is_some() {
            len += 1;
        }
        if !self.include_relations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.GetProductRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        if !self.include_relations.is_empty() {
            struct_ser.serialize_field("includeRelations", &self.include_relations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "locale_input",
            "localeInput",
            "include_relations",
            "includeRelations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            LocaleInput,
            IncludeRelations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "localeInput" | "locale_input" => Ok(GeneratedField::LocaleInput),
                            "includeRelations" | "include_relations" => Ok(GeneratedField::IncludeRelations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.GetProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut locale_input__ = None;
                let mut include_relations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeRelations => {
                            if include_relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeRelations"));
                            }
                            include_relations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetProductRequest {
                    id: id__.unwrap_or_default(),
                    locale_input: locale_input__,
                    include_relations: include_relations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.GetProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProductResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.product.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.GetProductResponse", len)?;
        if let Some(v) = self.product.as_ref() {
            struct_ser.serialize_field("product", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProductResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Product,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "product" => Ok(GeneratedField::Product),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProductResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.GetProductResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProductResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Product => {
                            if product__.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            product__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetProductResponse {
                    product: product__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.GetProductResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProductVariantResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.variant.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.GetProductVariantResponse", len)?;
        if let Some(v) = self.variant.as_ref() {
            struct_ser.serialize_field("variant", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProductVariantResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "variant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Variant,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "variant" => Ok(GeneratedField::Variant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProductVariantResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.GetProductVariantResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProductVariantResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut variant__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Variant => {
                            if variant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variant"));
                            }
                            variant__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetProductVariantResponse {
                    variant: variant__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.GetProductVariantResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProductVariantsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_request.is_some() {
            len += 1;
        }
        if self.filter_by_product_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ListProductVariantsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_product_id.as_ref() {
            struct_ser.serialize_field("filterByProductId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProductVariantsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_product_id",
            "filterByProductId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByProductId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "baseRequest" | "base_request" => Ok(GeneratedField::BaseRequest),
                            "filterByProductId" | "filter_by_product_id" => Ok(GeneratedField::FilterByProductId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProductVariantsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ListProductVariantsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProductVariantsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_product_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByProductId => {
                            if filter_by_product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByProductId"));
                            }
                            filter_by_product_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListProductVariantsRequest {
                    base_request: base_request__,
                    filter_by_product_id: filter_by_product_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ListProductVariantsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProductVariantsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.variants.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ListProductVariantsResponse", len)?;
        if !self.variants.is_empty() {
            struct_ser.serialize_field("variants", &self.variants)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProductVariantsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "variants",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Variants,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "variants" => Ok(GeneratedField::Variants),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProductVariantsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ListProductVariantsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProductVariantsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut variants__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Variants => {
                            if variants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variants"));
                            }
                            variants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListProductVariantsResponse {
                    variants: variants__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ListProductVariantsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProductsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_request.is_some() {
            len += 1;
        }
        if self.filter_by_category_id.is_some() {
            len += 1;
        }
        if self.filter_by_product_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ListProductsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_category_id.as_ref() {
            struct_ser.serialize_field("filterByCategoryId", v)?;
        }
        if let Some(v) = self.filter_by_product_type.as_ref() {
            struct_ser.serialize_field("filterByProductType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProductsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_category_id",
            "filterByCategoryId",
            "filter_by_product_type",
            "filterByProductType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByCategoryId,
            FilterByProductType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "baseRequest" | "base_request" => Ok(GeneratedField::BaseRequest),
                            "filterByCategoryId" | "filter_by_category_id" => Ok(GeneratedField::FilterByCategoryId),
                            "filterByProductType" | "filter_by_product_type" => Ok(GeneratedField::FilterByProductType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProductsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ListProductsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProductsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_category_id__ = None;
                let mut filter_by_product_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByCategoryId => {
                            if filter_by_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByCategoryId"));
                            }
                            filter_by_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByProductType => {
                            if filter_by_product_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByProductType"));
                            }
                            filter_by_product_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListProductsRequest {
                    base_request: base_request__,
                    filter_by_category_id: filter_by_category_id__,
                    filter_by_product_type: filter_by_product_type__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ListProductsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProductsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.products.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ListProductsResponse", len)?;
        if !self.products.is_empty() {
            struct_ser.serialize_field("products", &self.products)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProductsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "products",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Products,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "products" => Ok(GeneratedField::Products),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProductsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ListProductsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProductsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut products__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Products => {
                            if products__.is_some() {
                                return Err(serde::de::Error::duplicate_field("products"));
                            }
                            products__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListProductsResponse {
                    products: products__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ListProductsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Product {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.product_type.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.short_description.is_empty() {
            len += 1;
        }
        if !self.slug.is_empty() {
            len += 1;
        }
        if !self.meta_title.is_empty() {
            len += 1;
        }
        if !self.meta_description.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        if !self.categories.is_empty() {
            len += 1;
        }
        if !self.product_units.is_empty() {
            len += 1;
        }
        if !self.product_tags.is_empty() {
            len += 1;
        }
        if !self.product_options.is_empty() {
            len += 1;
        }
        if !self.product_variants.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.Product", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.product_type.is_empty() {
            struct_ser.serialize_field("productType", &self.product_type)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.short_description.is_empty() {
            struct_ser.serialize_field("shortDescription", &self.short_description)?;
        }
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if !self.meta_title.is_empty() {
            struct_ser.serialize_field("metaTitle", &self.meta_title)?;
        }
        if !self.meta_description.is_empty() {
            struct_ser.serialize_field("metaDescription", &self.meta_description)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        if !self.categories.is_empty() {
            struct_ser.serialize_field("categories", &self.categories)?;
        }
        if !self.product_units.is_empty() {
            struct_ser.serialize_field("productUnits", &self.product_units)?;
        }
        if !self.product_tags.is_empty() {
            struct_ser.serialize_field("productTags", &self.product_tags)?;
        }
        if !self.product_options.is_empty() {
            struct_ser.serialize_field("productOptions", &self.product_options)?;
        }
        if !self.product_variants.is_empty() {
            struct_ser.serialize_field("productVariants", &self.product_variants)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Product {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "product_type",
            "productType",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "description",
            "short_description",
            "shortDescription",
            "slug",
            "meta_title",
            "metaTitle",
            "meta_description",
            "metaDescription",
            "all_translations",
            "allTranslations",
            "categories",
            "product_units",
            "productUnits",
            "product_tags",
            "productTags",
            "product_options",
            "productOptions",
            "product_variants",
            "productVariants",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            ProductType,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            ShortDescription,
            Slug,
            MetaTitle,
            MetaDescription,
            AllTranslations,
            Categories,
            ProductUnits,
            ProductTags,
            ProductOptions,
            ProductVariants,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "code" => Ok(GeneratedField::Code),
                            "productType" | "product_type" => Ok(GeneratedField::ProductType),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "shortDescription" | "short_description" => Ok(GeneratedField::ShortDescription),
                            "slug" => Ok(GeneratedField::Slug),
                            "metaTitle" | "meta_title" => Ok(GeneratedField::MetaTitle),
                            "metaDescription" | "meta_description" => Ok(GeneratedField::MetaDescription),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "categories" => Ok(GeneratedField::Categories),
                            "productUnits" | "product_units" => Ok(GeneratedField::ProductUnits),
                            "productTags" | "product_tags" => Ok(GeneratedField::ProductTags),
                            "productOptions" | "product_options" => Ok(GeneratedField::ProductOptions),
                            "productVariants" | "product_variants" => Ok(GeneratedField::ProductVariants),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Product;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.Product")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Product, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut product_type__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut short_description__ = None;
                let mut slug__ = None;
                let mut meta_title__ = None;
                let mut meta_description__ = None;
                let mut all_translations__ = None;
                let mut categories__ = None;
                let mut product_units__ = None;
                let mut product_tags__ = None;
                let mut product_options__ = None;
                let mut product_variants__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductType => {
                            if product_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productType"));
                            }
                            product_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShortDescription => {
                            if short_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shortDescription"));
                            }
                            short_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetaTitle => {
                            if meta_title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metaTitle"));
                            }
                            meta_title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetaDescription => {
                            if meta_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metaDescription"));
                            }
                            meta_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Categories => {
                            if categories__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categories"));
                            }
                            categories__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductUnits => {
                            if product_units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productUnits"));
                            }
                            product_units__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductTags => {
                            if product_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productTags"));
                            }
                            product_tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductOptions => {
                            if product_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOptions"));
                            }
                            product_options__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductVariants => {
                            if product_variants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productVariants"));
                            }
                            product_variants__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Product {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    product_type: product_type__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    short_description: short_description__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    meta_title: meta_title__.unwrap_or_default(),
                    meta_description: meta_description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    categories: categories__.unwrap_or_default(),
                    product_units: product_units__.unwrap_or_default(),
                    product_tags: product_tags__.unwrap_or_default(),
                    product_options: product_options__.unwrap_or_default(),
                    product_variants: product_variants__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.Product", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductCategoryLinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_id.is_empty() {
            len += 1;
        }
        if !self.category_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ProductCategoryLinkRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.category_id.is_empty() {
            struct_ser.serialize_field("categoryId", &self.category_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductCategoryLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "category_id",
            "categoryId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            CategoryId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "categoryId" | "category_id" => Ok(GeneratedField::CategoryId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductCategoryLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ProductCategoryLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductCategoryLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut category_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CategoryId => {
                            if category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categoryId"));
                            }
                            category_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProductCategoryLinkRequest {
                    product_id: product_id__.unwrap_or_default(),
                    category_id: category_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ProductCategoryLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.option_code.is_empty() {
            len += 1;
        }
        if self.sort_order != 0 {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        if !self.option_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ProductOption", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.option_code.is_empty() {
            struct_ser.serialize_field("optionCode", &self.option_code)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        if !self.option_values.is_empty() {
            struct_ser.serialize_field("optionValues", &self.option_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "option_code",
            "optionCode",
            "sort_order",
            "sortOrder",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "all_translations",
            "allTranslations",
            "option_values",
            "optionValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            OptionCode,
            SortOrder,
            CreatedAt,
            UpdatedAt,
            Name,
            AllTranslations,
            OptionValues,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "optionCode" | "option_code" => Ok(GeneratedField::OptionCode),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "optionValues" | "option_values" => Ok(GeneratedField::OptionValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ProductOption")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductOption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut option_code__ = None;
                let mut sort_order__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut all_translations__ = None;
                let mut option_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionCode => {
                            if option_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionCode"));
                            }
                            option_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionValues => {
                            if option_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionValues"));
                            }
                            option_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProductOption {
                    id: id__.unwrap_or_default(),
                    option_code: option_code__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    option_values: option_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ProductOption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductOptionValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.value_code.is_empty() {
            len += 1;
        }
        if self.sort_order != 0 {
            len += 1;
        }
        if self.additional_data.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ProductOptionValue", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.value_code.is_empty() {
            struct_ser.serialize_field("valueCode", &self.value_code)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
        }
        if let Some(v) = self.additional_data.as_ref() {
            struct_ser.serialize_field("additionalData", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductOptionValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value_code",
            "valueCode",
            "sort_order",
            "sortOrder",
            "additional_data",
            "additionalData",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ValueCode,
            SortOrder,
            AdditionalData,
            CreatedAt,
            UpdatedAt,
            Name,
            AllTranslations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "valueCode" | "value_code" => Ok(GeneratedField::ValueCode),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "additionalData" | "additional_data" => Ok(GeneratedField::AdditionalData),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductOptionValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ProductOptionValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductOptionValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut value_code__ = None;
                let mut sort_order__ = None;
                let mut additional_data__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut all_translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueCode => {
                            if value_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCode"));
                            }
                            value_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AdditionalData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalData"));
                            }
                            additional_data__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProductOptionValue {
                    id: id__.unwrap_or_default(),
                    value_code: value_code__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                    additional_data: additional_data__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ProductOptionValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.tag_key.is_empty() {
            len += 1;
        }
        if !self.tag_value_code.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if !self.display_value.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ProductTag", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.tag_key.is_empty() {
            struct_ser.serialize_field("tagKey", &self.tag_key)?;
        }
        if !self.tag_value_code.is_empty() {
            struct_ser.serialize_field("tagValueCode", &self.tag_value_code)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if !self.display_value.is_empty() {
            struct_ser.serialize_field("displayValue", &self.display_value)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "tag_key",
            "tagKey",
            "tag_value_code",
            "tagValueCode",
            "created_at",
            "createdAt",
            "display_value",
            "displayValue",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TagKey,
            TagValueCode,
            CreatedAt,
            DisplayValue,
            AllTranslations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "tagKey" | "tag_key" => Ok(GeneratedField::TagKey),
                            "tagValueCode" | "tag_value_code" => Ok(GeneratedField::TagValueCode),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "displayValue" | "display_value" => Ok(GeneratedField::DisplayValue),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ProductTag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut tag_key__ = None;
                let mut tag_value_code__ = None;
                let mut created_at__ = None;
                let mut display_value__ = None;
                let mut all_translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagKey => {
                            if tag_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagKey"));
                            }
                            tag_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagValueCode => {
                            if tag_value_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagValueCode"));
                            }
                            tag_value_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::DisplayValue => {
                            if display_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayValue"));
                            }
                            display_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProductTag {
                    id: id__.unwrap_or_default(),
                    tag_key: tag_key__.unwrap_or_default(),
                    tag_value_code: tag_value_code__.unwrap_or_default(),
                    created_at: created_at__,
                    display_value: display_value__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ProductTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductUnit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.unit.is_some() {
            len += 1;
        }
        if self.barcode.is_some() {
            len += 1;
        }
        if self.is_base_unit {
            len += 1;
        }
        if !self.conversion_to_base_factor.is_empty() {
            len += 1;
        }
        if self.is_sellable {
            len += 1;
        }
        if self.is_purchasable {
            len += 1;
        }
        if self.sort_order != 0 {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ProductUnit", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.unit.as_ref() {
            struct_ser.serialize_field("unit", v)?;
        }
        if let Some(v) = self.barcode.as_ref() {
            struct_ser.serialize_field("barcode", v)?;
        }
        if self.is_base_unit {
            struct_ser.serialize_field("isBaseUnit", &self.is_base_unit)?;
        }
        if !self.conversion_to_base_factor.is_empty() {
            struct_ser.serialize_field("conversionToBaseFactor", &self.conversion_to_base_factor)?;
        }
        if self.is_sellable {
            struct_ser.serialize_field("isSellable", &self.is_sellable)?;
        }
        if self.is_purchasable {
            struct_ser.serialize_field("isPurchasable", &self.is_purchasable)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductUnit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "unit",
            "barcode",
            "is_base_unit",
            "isBaseUnit",
            "conversion_to_base_factor",
            "conversionToBaseFactor",
            "is_sellable",
            "isSellable",
            "is_purchasable",
            "isPurchasable",
            "sort_order",
            "sortOrder",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Unit,
            Barcode,
            IsBaseUnit,
            ConversionToBaseFactor,
            IsSellable,
            IsPurchasable,
            SortOrder,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "unit" => Ok(GeneratedField::Unit),
                            "barcode" => Ok(GeneratedField::Barcode),
                            "isBaseUnit" | "is_base_unit" => Ok(GeneratedField::IsBaseUnit),
                            "conversionToBaseFactor" | "conversion_to_base_factor" => Ok(GeneratedField::ConversionToBaseFactor),
                            "isSellable" | "is_sellable" => Ok(GeneratedField::IsSellable),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductUnit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ProductUnit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductUnit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut unit__ = None;
                let mut barcode__ = None;
                let mut is_base_unit__ = None;
                let mut conversion_to_base_factor__ = None;
                let mut is_sellable__ = None;
                let mut is_purchasable__ = None;
                let mut sort_order__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = map_.next_value()?;
                        }
                        GeneratedField::Barcode => {
                            if barcode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("barcode"));
                            }
                            barcode__ = map_.next_value()?;
                        }
                        GeneratedField::IsBaseUnit => {
                            if is_base_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBaseUnit"));
                            }
                            is_base_unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversionToBaseFactor => {
                            if conversion_to_base_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversionToBaseFactor"));
                            }
                            conversion_to_base_factor__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSellable => {
                            if is_sellable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSellable"));
                            }
                            is_sellable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProductUnit {
                    id: id__.unwrap_or_default(),
                    unit: unit__,
                    barcode: barcode__,
                    is_base_unit: is_base_unit__.unwrap_or_default(),
                    conversion_to_base_factor: conversion_to_base_factor__.unwrap_or_default(),
                    is_sellable: is_sellable__.unwrap_or_default(),
                    is_purchasable: is_purchasable__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ProductUnit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductVariant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.sku.is_empty() {
            len += 1;
        }
        if self.barcode.is_some() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.assigned_options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.ProductVariant", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.sku.is_empty() {
            struct_ser.serialize_field("sku", &self.sku)?;
        }
        if let Some(v) = self.barcode.as_ref() {
            struct_ser.serialize_field("barcode", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.assigned_options.is_empty() {
            struct_ser.serialize_field("assignedOptions", &self.assigned_options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductVariant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "sku",
            "barcode",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "display_name",
            "displayName",
            "assigned_options",
            "assignedOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Sku,
            Barcode,
            IsActive,
            CreatedAt,
            UpdatedAt,
            DisplayName,
            AssignedOptions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "sku" => Ok(GeneratedField::Sku),
                            "barcode" => Ok(GeneratedField::Barcode),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "assignedOptions" | "assigned_options" => Ok(GeneratedField::AssignedOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductVariant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.ProductVariant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductVariant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut sku__ = None;
                let mut barcode__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut display_name__ = None;
                let mut assigned_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sku => {
                            if sku__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sku"));
                            }
                            sku__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Barcode => {
                            if barcode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("barcode"));
                            }
                            barcode__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssignedOptions => {
                            if assigned_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedOptions"));
                            }
                            assigned_options__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProductVariant {
                    id: id__.unwrap_or_default(),
                    sku: sku__.unwrap_or_default(),
                    barcode: barcode__,
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    display_name: display_name__.unwrap_or_default(),
                    assigned_options: assigned_options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.ProductVariant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOptionValueFromVariantRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.variant_id.is_empty() {
            len += 1;
        }
        if !self.option_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.RemoveOptionValueFromVariantRequest", len)?;
        if !self.variant_id.is_empty() {
            struct_ser.serialize_field("variantId", &self.variant_id)?;
        }
        if !self.option_id.is_empty() {
            struct_ser.serialize_field("optionId", &self.option_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOptionValueFromVariantRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "variant_id",
            "variantId",
            "option_id",
            "optionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VariantId,
            OptionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "optionId" | "option_id" => Ok(GeneratedField::OptionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveOptionValueFromVariantRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.RemoveOptionValueFromVariantRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOptionValueFromVariantRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut variant_id__ = None;
                let mut option_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionId => {
                            if option_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionId"));
                            }
                            option_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveOptionValueFromVariantRequest {
                    variant_id: variant_id__.unwrap_or_default(),
                    option_id: option_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.RemoveOptionValueFromVariantRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveTagFromProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_tag_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.RemoveTagFromProductRequest", len)?;
        if !self.product_tag_id.is_empty() {
            struct_ser.serialize_field("productTagId", &self.product_tag_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveTagFromProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_tag_id",
            "productTagId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductTagId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productTagId" | "product_tag_id" => Ok(GeneratedField::ProductTagId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveTagFromProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.RemoveTagFromProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveTagFromProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_tag_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductTagId => {
                            if product_tag_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productTagId"));
                            }
                            product_tag_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveTagFromProductRequest {
                    product_tag_id: product_tag_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.RemoveTagFromProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProductOptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.option_id.is_empty() {
            len += 1;
        }
        if self.option_code.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.UpdateProductOptionRequest", len)?;
        if !self.option_id.is_empty() {
            struct_ser.serialize_field("optionId", &self.option_id)?;
        }
        if let Some(v) = self.option_code.as_ref() {
            struct_ser.serialize_field("optionCode", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProductOptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "option_id",
            "optionId",
            "option_code",
            "optionCode",
            "sort_order",
            "sortOrder",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionId,
            OptionCode,
            SortOrder,
            TranslationsToUpdate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "optionId" | "option_id" => Ok(GeneratedField::OptionId),
                            "optionCode" | "option_code" => Ok(GeneratedField::OptionCode),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "translationsToUpdate" | "translations_to_update" => Ok(GeneratedField::TranslationsToUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProductOptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.UpdateProductOptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProductOptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut option_id__ = None;
                let mut option_code__ = None;
                let mut sort_order__ = None;
                let mut translations_to_update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OptionId => {
                            if option_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionId"));
                            }
                            option_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionCode => {
                            if option_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionCode"));
                            }
                            option_code__ = map_.next_value()?;
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TranslationsToUpdate => {
                            if translations_to_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translationsToUpdate"));
                            }
                            translations_to_update__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateProductOptionRequest {
                    option_id: option_id__.unwrap_or_default(),
                    option_code: option_code__,
                    sort_order: sort_order__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.UpdateProductOptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProductOptionValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.option_value_id.is_empty() {
            len += 1;
        }
        if self.value_code.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        if self.additional_data.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.UpdateProductOptionValueRequest", len)?;
        if !self.option_value_id.is_empty() {
            struct_ser.serialize_field("optionValueId", &self.option_value_id)?;
        }
        if let Some(v) = self.value_code.as_ref() {
            struct_ser.serialize_field("valueCode", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        if let Some(v) = self.additional_data.as_ref() {
            struct_ser.serialize_field("additionalData", v)?;
        }
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProductOptionValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "option_value_id",
            "optionValueId",
            "value_code",
            "valueCode",
            "sort_order",
            "sortOrder",
            "additional_data",
            "additionalData",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionValueId,
            ValueCode,
            SortOrder,
            AdditionalData,
            TranslationsToUpdate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "optionValueId" | "option_value_id" => Ok(GeneratedField::OptionValueId),
                            "valueCode" | "value_code" => Ok(GeneratedField::ValueCode),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "additionalData" | "additional_data" => Ok(GeneratedField::AdditionalData),
                            "translationsToUpdate" | "translations_to_update" => Ok(GeneratedField::TranslationsToUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProductOptionValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.UpdateProductOptionValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProductOptionValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut option_value_id__ = None;
                let mut value_code__ = None;
                let mut sort_order__ = None;
                let mut additional_data__ = None;
                let mut translations_to_update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OptionValueId => {
                            if option_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionValueId"));
                            }
                            option_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueCode => {
                            if value_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCode"));
                            }
                            value_code__ = map_.next_value()?;
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AdditionalData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalData"));
                            }
                            additional_data__ = map_.next_value()?;
                        }
                        GeneratedField::TranslationsToUpdate => {
                            if translations_to_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translationsToUpdate"));
                            }
                            translations_to_update__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateProductOptionValueRequest {
                    option_value_id: option_value_id__.unwrap_or_default(),
                    value_code: value_code__,
                    sort_order: sort_order__,
                    additional_data: additional_data__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.UpdateProductOptionValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        if self.product_type.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.UpdateProductRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.product_type.as_ref() {
            struct_ser.serialize_field("productType", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "product_type",
            "productType",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            ProductType,
            IsActive,
            TranslationsToUpdate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "code" => Ok(GeneratedField::Code),
                            "productType" | "product_type" => Ok(GeneratedField::ProductType),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "translationsToUpdate" | "translations_to_update" => Ok(GeneratedField::TranslationsToUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.UpdateProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut product_type__ = None;
                let mut is_active__ = None;
                let mut translations_to_update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::ProductType => {
                            if product_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productType"));
                            }
                            product_type__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::TranslationsToUpdate => {
                            if translations_to_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translationsToUpdate"));
                            }
                            translations_to_update__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateProductRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    product_type: product_type__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.UpdateProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProductUnitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_unit_id.is_empty() {
            len += 1;
        }
        if self.barcode.is_some() {
            len += 1;
        }
        if self.is_base_unit.is_some() {
            len += 1;
        }
        if self.conversion_to_base_factor.is_some() {
            len += 1;
        }
        if self.is_sellable.is_some() {
            len += 1;
        }
        if self.is_purchasable.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.UpdateProductUnitRequest", len)?;
        if !self.product_unit_id.is_empty() {
            struct_ser.serialize_field("productUnitId", &self.product_unit_id)?;
        }
        if let Some(v) = self.barcode.as_ref() {
            struct_ser.serialize_field("barcode", v)?;
        }
        if let Some(v) = self.is_base_unit.as_ref() {
            struct_ser.serialize_field("isBaseUnit", v)?;
        }
        if let Some(v) = self.conversion_to_base_factor.as_ref() {
            struct_ser.serialize_field("conversionToBaseFactor", v)?;
        }
        if let Some(v) = self.is_sellable.as_ref() {
            struct_ser.serialize_field("isSellable", v)?;
        }
        if let Some(v) = self.is_purchasable.as_ref() {
            struct_ser.serialize_field("isPurchasable", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProductUnitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_unit_id",
            "productUnitId",
            "barcode",
            "is_base_unit",
            "isBaseUnit",
            "conversion_to_base_factor",
            "conversionToBaseFactor",
            "is_sellable",
            "isSellable",
            "is_purchasable",
            "isPurchasable",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductUnitId,
            Barcode,
            IsBaseUnit,
            ConversionToBaseFactor,
            IsSellable,
            IsPurchasable,
            SortOrder,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "productUnitId" | "product_unit_id" => Ok(GeneratedField::ProductUnitId),
                            "barcode" => Ok(GeneratedField::Barcode),
                            "isBaseUnit" | "is_base_unit" => Ok(GeneratedField::IsBaseUnit),
                            "conversionToBaseFactor" | "conversion_to_base_factor" => Ok(GeneratedField::ConversionToBaseFactor),
                            "isSellable" | "is_sellable" => Ok(GeneratedField::IsSellable),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProductUnitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.UpdateProductUnitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProductUnitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_unit_id__ = None;
                let mut barcode__ = None;
                let mut is_base_unit__ = None;
                let mut conversion_to_base_factor__ = None;
                let mut is_sellable__ = None;
                let mut is_purchasable__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductUnitId => {
                            if product_unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productUnitId"));
                            }
                            product_unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Barcode => {
                            if barcode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("barcode"));
                            }
                            barcode__ = map_.next_value()?;
                        }
                        GeneratedField::IsBaseUnit => {
                            if is_base_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBaseUnit"));
                            }
                            is_base_unit__ = map_.next_value()?;
                        }
                        GeneratedField::ConversionToBaseFactor => {
                            if conversion_to_base_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversionToBaseFactor"));
                            }
                            conversion_to_base_factor__ = map_.next_value()?;
                        }
                        GeneratedField::IsSellable => {
                            if is_sellable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSellable"));
                            }
                            is_sellable__ = map_.next_value()?;
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = map_.next_value()?;
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(UpdateProductUnitRequest {
                    product_unit_id: product_unit_id__.unwrap_or_default(),
                    barcode: barcode__,
                    is_base_unit: is_base_unit__,
                    conversion_to_base_factor: conversion_to_base_factor__,
                    is_sellable: is_sellable__,
                    is_purchasable: is_purchasable__,
                    sort_order: sort_order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.UpdateProductUnitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProductVariantRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.variant_id.is_empty() {
            len += 1;
        }
        if self.sku.is_some() {
            len += 1;
        }
        if self.barcode.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.UpdateProductVariantRequest", len)?;
        if !self.variant_id.is_empty() {
            struct_ser.serialize_field("variantId", &self.variant_id)?;
        }
        if let Some(v) = self.sku.as_ref() {
            struct_ser.serialize_field("sku", v)?;
        }
        if let Some(v) = self.barcode.as_ref() {
            struct_ser.serialize_field("barcode", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProductVariantRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "variant_id",
            "variantId",
            "sku",
            "barcode",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VariantId,
            Sku,
            Barcode,
            IsActive,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "sku" => Ok(GeneratedField::Sku),
                            "barcode" => Ok(GeneratedField::Barcode),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProductVariantRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.UpdateProductVariantRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProductVariantRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut variant_id__ = None;
                let mut sku__ = None;
                let mut barcode__ = None;
                let mut is_active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sku => {
                            if sku__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sku"));
                            }
                            sku__ = map_.next_value()?;
                        }
                        GeneratedField::Barcode => {
                            if barcode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("barcode"));
                            }
                            barcode__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateProductVariantRequest {
                    variant_id: variant_id__.unwrap_or_default(),
                    sku: sku__,
                    barcode: barcode__,
                    is_active: is_active__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.UpdateProductVariantRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VariantOptionAssignmentInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.option_id.is_empty() {
            len += 1;
        }
        if !self.value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.product.v1.VariantOptionAssignmentInput", len)?;
        if !self.option_id.is_empty() {
            struct_ser.serialize_field("optionId", &self.option_id)?;
        }
        if !self.value_id.is_empty() {
            struct_ser.serialize_field("valueId", &self.value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VariantOptionAssignmentInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "option_id",
            "optionId",
            "value_id",
            "valueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionId,
            ValueId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "optionId" | "option_id" => Ok(GeneratedField::OptionId),
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VariantOptionAssignmentInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.product.v1.VariantOptionAssignmentInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VariantOptionAssignmentInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut option_id__ = None;
                let mut value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OptionId => {
                            if option_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionId"));
                            }
                            option_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueId => {
                            if value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VariantOptionAssignmentInput {
                    option_id: option_id__.unwrap_or_default(),
                    value_id: value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.product.v1.VariantOptionAssignmentInput", FIELDS, GeneratedVisitor)
    }
}
