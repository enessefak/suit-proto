// @generated
impl serde::Serialize for Address {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.street.is_empty() {
            len += 1;
        }
        if !self.city.is_empty() {
            len += 1;
        }
        if !self.postal_code.is_empty() {
            len += 1;
        }
        if !self.country_code.is_empty() {
            len += 1;
        }
        if !self.district.is_empty() {
            len += 1;
        }
        if !self.address_line_1.is_empty() {
            len += 1;
        }
        if !self.address_line_2.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Address", len)?;
        if !self.street.is_empty() {
            struct_ser.serialize_field("street", &self.street)?;
        }
        if !self.city.is_empty() {
            struct_ser.serialize_field("city", &self.city)?;
        }
        if !self.postal_code.is_empty() {
            struct_ser.serialize_field("postalCode", &self.postal_code)?;
        }
        if !self.country_code.is_empty() {
            struct_ser.serialize_field("countryCode", &self.country_code)?;
        }
        if !self.district.is_empty() {
            struct_ser.serialize_field("district", &self.district)?;
        }
        if !self.address_line_1.is_empty() {
            struct_ser.serialize_field("addressLine1", &self.address_line_1)?;
        }
        if !self.address_line_2.is_empty() {
            struct_ser.serialize_field("addressLine2", &self.address_line_2)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Address {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "street",
            "city",
            "postal_code",
            "postalCode",
            "country_code",
            "countryCode",
            "district",
            "address_line_1",
            "addressLine1",
            "address_line_2",
            "addressLine2",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Street,
            City,
            PostalCode,
            CountryCode,
            District,
            AddressLine1,
            AddressLine2,
            Description,
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
                            "street" => Ok(GeneratedField::Street),
                            "city" => Ok(GeneratedField::City),
                            "postalCode" | "postal_code" => Ok(GeneratedField::PostalCode),
                            "countryCode" | "country_code" => Ok(GeneratedField::CountryCode),
                            "district" => Ok(GeneratedField::District),
                            "addressLine1" | "address_line_1" => Ok(GeneratedField::AddressLine1),
                            "addressLine2" | "address_line_2" => Ok(GeneratedField::AddressLine2),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Address")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Address, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut street__ = None;
                let mut city__ = None;
                let mut postal_code__ = None;
                let mut country_code__ = None;
                let mut district__ = None;
                let mut address_line_1__ = None;
                let mut address_line_2__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Street => {
                            if street__.is_some() {
                                return Err(serde::de::Error::duplicate_field("street"));
                            }
                            street__ = Some(map_.next_value()?);
                        }
                        GeneratedField::City => {
                            if city__.is_some() {
                                return Err(serde::de::Error::duplicate_field("city"));
                            }
                            city__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PostalCode => {
                            if postal_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postalCode"));
                            }
                            postal_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CountryCode => {
                            if country_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryCode"));
                            }
                            country_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::District => {
                            if district__.is_some() {
                                return Err(serde::de::Error::duplicate_field("district"));
                            }
                            district__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressLine1 => {
                            if address_line_1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressLine1"));
                            }
                            address_line_1__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressLine2 => {
                            if address_line_2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressLine2"));
                            }
                            address_line_2__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Address {
                    street: street__.unwrap_or_default(),
                    city: city__.unwrap_or_default(),
                    postal_code: postal_code__.unwrap_or_default(),
                    country_code: country_code__.unwrap_or_default(),
                    district: district__.unwrap_or_default(),
                    address_line_1: address_line_1__.unwrap_or_default(),
                    address_line_2: address_line_2__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Address", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DateRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.DateRange", len)?;
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DateRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_date",
            "startDate",
            "end_date",
            "endDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartDate,
            EndDate,
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
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DateRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.DateRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DateRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_date__ = None;
                let mut end_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map_.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DateRange {
                    start_date: start_date__,
                    end_date: end_date__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.DateRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteByIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.DeleteByIdRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteByIdRequest {
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
            type Value = DeleteByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.DeleteByIdRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteByIdRequest, V::Error>
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
                Ok(DeleteByIdRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.DeleteByIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Filter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.operator.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Filter", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.operator.is_empty() {
            struct_ser.serialize_field("operator", &self.operator)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Filter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "operator",
            "value",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Operator,
            Value,
            Values,
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
                            "field" => Ok(GeneratedField::Field),
                            "operator" => Ok(GeneratedField::Operator),
                            "value" => Ok(GeneratedField::Value),
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Filter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Filter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Filter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut operator__ = None;
                let mut value__ = None;
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Filter {
                    field: field__.unwrap_or_default(),
                    operator: operator__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Filter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetByIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.GetByIdRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for GetByIdRequest {
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
            type Value = GetByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.GetByIdRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetByIdRequest, V::Error>
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
                Ok(GetByIdRequest {
                    id: id__.unwrap_or_default(),
                    locale_input: locale_input__,
                    include_relations: include_relations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.GetByIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.IdResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdResponse {
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
            type Value = IdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.IdResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdResponse, V::Error>
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
                Ok(IdResponse {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.IdResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if !self.sorts.is_empty() {
            len += 1;
        }
        if self.locale_input.is_some() {
            len += 1;
        }
        if self.search_term.is_some() {
            len += 1;
        }
        if !self.include_fields.is_empty() {
            len += 1;
        }
        if !self.include_relations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.ListRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if !self.sorts.is_empty() {
            struct_ser.serialize_field("sorts", &self.sorts)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        if let Some(v) = self.search_term.as_ref() {
            struct_ser.serialize_field("searchTerm", v)?;
        }
        if !self.include_fields.is_empty() {
            struct_ser.serialize_field("includeFields", &self.include_fields)?;
        }
        if !self.include_relations.is_empty() {
            struct_ser.serialize_field("includeRelations", &self.include_relations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "filters",
            "sorts",
            "locale_input",
            "localeInput",
            "search_term",
            "searchTerm",
            "include_fields",
            "includeFields",
            "include_relations",
            "includeRelations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            Filters,
            Sorts,
            LocaleInput,
            SearchTerm,
            IncludeFields,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            "filters" => Ok(GeneratedField::Filters),
                            "sorts" => Ok(GeneratedField::Sorts),
                            "localeInput" | "locale_input" => Ok(GeneratedField::LocaleInput),
                            "searchTerm" | "search_term" => Ok(GeneratedField::SearchTerm),
                            "includeFields" | "include_fields" => Ok(GeneratedField::IncludeFields),
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
            type Value = ListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.ListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut filters__ = None;
                let mut sorts__ = None;
                let mut locale_input__ = None;
                let mut search_term__ = None;
                let mut include_fields__ = None;
                let mut include_relations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sorts => {
                            if sorts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sorts"));
                            }
                            sorts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                        GeneratedField::SearchTerm => {
                            if search_term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchTerm"));
                            }
                            search_term__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeFields => {
                            if include_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeFields"));
                            }
                            include_fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeRelations => {
                            if include_relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeRelations"));
                            }
                            include_relations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListRequest {
                    pagination: pagination__,
                    filters: filters__.unwrap_or_default(),
                    sorts: sorts__.unwrap_or_default(),
                    locale_input: locale_input__,
                    search_term: search_term__,
                    include_fields: include_fields__.unwrap_or_default(),
                    include_relations: include_relations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.ListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocaleInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locale.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.LocaleInput", len)?;
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocaleInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locale,
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
                            "locale" => Ok(GeneratedField::Locale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocaleInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.LocaleInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocaleInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locale__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LocaleInput {
                    locale: locale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.LocaleInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Money {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.currency_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Money", len)?;
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.currency_code.is_empty() {
            struct_ser.serialize_field("currencyCode", &self.currency_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Money {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "currency_code",
            "currencyCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            CurrencyCode,
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
                            "amount" => Ok(GeneratedField::Amount),
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Money;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Money")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Money, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut currency_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Money {
                    amount: amount__.unwrap_or_default(),
                    currency_code: currency_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Money", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.MutateResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "id",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            Id,
            Message,
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
                            "success" => Ok(GeneratedField::Success),
                            "id" => Ok(GeneratedField::Id),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.MutateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut id__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MutateResponse {
                    success: success__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.MutateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaginationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_number != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.PaginationRequest", len)?;
        if self.page_number != 0 {
            struct_ser.serialize_field("pageNumber", &self.page_number)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaginationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_number",
            "pageNumber",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageNumber,
            PageSize,
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
                            "pageNumber" | "page_number" => Ok(GeneratedField::PageNumber),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaginationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.PaginationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PaginationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_number__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageNumber => {
                            if page_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageNumber"));
                            }
                            page_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PaginationRequest {
                    page_number: page_number__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.PaginationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaginationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_number != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if self.total_records != 0 {
            len += 1;
        }
        if self.total_pages != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.PaginationResponse", len)?;
        if self.page_number != 0 {
            struct_ser.serialize_field("pageNumber", &self.page_number)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if self.total_records != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalRecords", ToString::to_string(&self.total_records).as_str())?;
        }
        if self.total_pages != 0 {
            struct_ser.serialize_field("totalPages", &self.total_pages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaginationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_number",
            "pageNumber",
            "page_size",
            "pageSize",
            "total_records",
            "totalRecords",
            "total_pages",
            "totalPages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageNumber,
            PageSize,
            TotalRecords,
            TotalPages,
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
                            "pageNumber" | "page_number" => Ok(GeneratedField::PageNumber),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "totalRecords" | "total_records" => Ok(GeneratedField::TotalRecords),
                            "totalPages" | "total_pages" => Ok(GeneratedField::TotalPages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaginationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.PaginationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PaginationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_number__ = None;
                let mut page_size__ = None;
                let mut total_records__ = None;
                let mut total_pages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageNumber => {
                            if page_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageNumber"));
                            }
                            page_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalRecords => {
                            if total_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRecords"));
                            }
                            total_records__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalPages => {
                            if total_pages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPages"));
                            }
                            total_pages__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PaginationResponse {
                    page_number: page_number__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    total_records: total_records__.unwrap_or_default(),
                    total_pages: total_pages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.PaginationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortBy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.SortBy", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if self.direction != 0 {
            let v = sort_by::SortDirection::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SortBy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Direction,
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
                            "field" => Ok(GeneratedField::Field),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.SortBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SortBy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<sort_by::SortDirection>()? as i32);
                        }
                    }
                }
                Ok(SortBy {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.SortBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for sort_by::SortDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for sort_by::SortDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ASC",
            "DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = sort_by::SortDirection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ASC" => Ok(sort_by::SortDirection::Asc),
                    "DESC" => Ok(sort_by::SortDirection::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TranslationInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locale.is_empty() {
            len += 1;
        }
        if !self.field_name.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.TranslationInput", len)?;
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        if !self.field_name.is_empty() {
            struct_ser.serialize_field("fieldName", &self.field_name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranslationInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locale",
            "field_name",
            "fieldName",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locale,
            FieldName,
            Value,
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
                            "locale" => Ok(GeneratedField::Locale),
                            "fieldName" | "field_name" => Ok(GeneratedField::FieldName),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranslationInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.TranslationInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranslationInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locale__ = None;
                let mut field_name__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldName => {
                            if field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldName"));
                            }
                            field_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TranslationInput {
                    locale: locale__.unwrap_or_default(),
                    field_name: field_name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.TranslationInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranslationOutput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locale.is_empty() {
            len += 1;
        }
        if !self.field_name.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.TranslationOutput", len)?;
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        if !self.field_name.is_empty() {
            struct_ser.serialize_field("fieldName", &self.field_name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
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
impl<'de> serde::Deserialize<'de> for TranslationOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locale",
            "field_name",
            "fieldName",
            "value",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locale,
            FieldName,
            Value,
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
                            "locale" => Ok(GeneratedField::Locale),
                            "fieldName" | "field_name" => Ok(GeneratedField::FieldName),
                            "value" => Ok(GeneratedField::Value),
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
            type Value = TranslationOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.TranslationOutput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranslationOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locale__ = None;
                let mut field_name__ = None;
                let mut value__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldName => {
                            if field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldName"));
                            }
                            field_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
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
                Ok(TranslationOutput {
                    locale: locale__.unwrap_or_default(),
                    field_name: field_name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.TranslationOutput", FIELDS, GeneratedVisitor)
    }
}
