// @generated
impl serde::Serialize for CreatePricingRuleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreatePricingRuleRequest", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePricingRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePricingRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreatePricingRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePricingRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePricingRuleRequest {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreatePricingRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePricingRuleResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreatePricingRuleResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePricingRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePricingRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreatePricingRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePricingRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreatePricingRuleResponse {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreatePricingRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPricingRuleRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.GetPricingRuleRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPricingRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPricingRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.GetPricingRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPricingRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPricingRuleRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.GetPricingRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPricingRuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.GetPricingRuleResponse", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPricingRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPricingRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.GetPricingRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPricingRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPricingRuleResponse {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.GetPricingRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PricingRule {
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
        if !self.product_id.is_empty() {
            len += 1;
        }
        if !self.region.is_empty() {
            len += 1;
        }
        if self.discounted_price.is_some() {
            len += 1;
        }
        if self.start_time != 0 {
            len += 1;
        }
        if self.end_time != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.PricingRule", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.region.is_empty() {
            struct_ser.serialize_field("region", &self.region)?;
        }
        if let Some(v) = self.discounted_price.as_ref() {
            struct_ser.serialize_field("discountedPrice", v)?;
        }
        if self.start_time != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("startTime", ToString::to_string(&self.start_time).as_str())?;
        }
        if self.end_time != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("endTime", ToString::to_string(&self.end_time).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PricingRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "product_id",
            "productId",
            "region",
            "discounted_price",
            "discountedPrice",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProductId,
            Region,
            DiscountedPrice,
            StartTime,
            EndTime,
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
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "region" => Ok(GeneratedField::Region),
                            "discountedPrice" | "discounted_price" => Ok(GeneratedField::DiscountedPrice),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PricingRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.PricingRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PricingRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_id__ = None;
                let mut region__ = None;
                let mut discounted_price__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Region => {
                            if region__.is_some() {
                                return Err(serde::de::Error::duplicate_field("region"));
                            }
                            region__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DiscountedPrice => {
                            if discounted_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discountedPrice"));
                            }
                            discounted_price__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PricingRule {
                    id: id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    region: region__.unwrap_or_default(),
                    discounted_price: discounted_price__,
                    start_time: start_time__.unwrap_or_default(),
                    end_time: end_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.PricingRule", FIELDS, GeneratedVisitor)
    }
}
