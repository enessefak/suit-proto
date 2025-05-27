// @generated
impl serde::Serialize for Empty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("projectsuit.common.v1.Empty", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Empty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Empty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Empty")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Empty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Empty {
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Empty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Error {
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
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Error", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Error {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Metadata,
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
                            "message" => Ok(GeneratedField::Message),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Error;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Error")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Error, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Error {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Error", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorResponse {
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
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.ErrorResponse", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Details,
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
                            "message" => Ok(GeneratedField::Message),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.ErrorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ErrorResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.ErrorResponse", FIELDS, GeneratedVisitor)
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
        if !self.currency.is_empty() {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Money", len)?;
        if !self.currency.is_empty() {
            struct_ser.serialize_field("currency", &self.currency)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
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
            "currency",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Currency,
            Amount,
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
                            "currency" => Ok(GeneratedField::Currency),
                            "amount" => Ok(GeneratedField::Amount),
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
                let mut currency__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Money {
                    currency: currency__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Money", FIELDS, GeneratedVisitor)
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
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.sort_by.is_empty() {
            len += 1;
        }
        if self.sort_desc {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.PaginationRequest", len)?;
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.sort_by.is_empty() {
            struct_ser.serialize_field("sortBy", &self.sort_by)?;
        }
        if self.sort_desc {
            struct_ser.serialize_field("sortDesc", &self.sort_desc)?;
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
            "page",
            "page_size",
            "pageSize",
            "sort_by",
            "sortBy",
            "sort_desc",
            "sortDesc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
            PageSize,
            SortBy,
            SortDesc,
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
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "sortBy" | "sort_by" => Ok(GeneratedField::SortBy),
                            "sortDesc" | "sort_desc" => Ok(GeneratedField::SortDesc),
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
                let mut page__ = None;
                let mut page_size__ = None;
                let mut sort_by__ = None;
                let mut sort_desc__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
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
                        GeneratedField::SortBy => {
                            if sort_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortBy"));
                            }
                            sort_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortDesc => {
                            if sort_desc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortDesc"));
                            }
                            sort_desc__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PaginationRequest {
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    sort_by: sort_by__.unwrap_or_default(),
                    sort_desc: sort_desc__.unwrap_or_default(),
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
        if self.total_items != 0 {
            len += 1;
        }
        if self.total_pages != 0 {
            len += 1;
        }
        if self.current_page != 0 {
            len += 1;
        }
        if self.items_per_page != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.PaginationResponse", len)?;
        if self.total_items != 0 {
            struct_ser.serialize_field("totalItems", &self.total_items)?;
        }
        if self.total_pages != 0 {
            struct_ser.serialize_field("totalPages", &self.total_pages)?;
        }
        if self.current_page != 0 {
            struct_ser.serialize_field("currentPage", &self.current_page)?;
        }
        if self.items_per_page != 0 {
            struct_ser.serialize_field("itemsPerPage", &self.items_per_page)?;
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
            "total_items",
            "totalItems",
            "total_pages",
            "totalPages",
            "current_page",
            "currentPage",
            "items_per_page",
            "itemsPerPage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalItems,
            TotalPages,
            CurrentPage,
            ItemsPerPage,
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
                            "totalItems" | "total_items" => Ok(GeneratedField::TotalItems),
                            "totalPages" | "total_pages" => Ok(GeneratedField::TotalPages),
                            "currentPage" | "current_page" => Ok(GeneratedField::CurrentPage),
                            "itemsPerPage" | "items_per_page" => Ok(GeneratedField::ItemsPerPage),
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
                let mut total_items__ = None;
                let mut total_pages__ = None;
                let mut current_page__ = None;
                let mut items_per_page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalItems => {
                            if total_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalItems"));
                            }
                            total_items__ = 
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
                        GeneratedField::CurrentPage => {
                            if current_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPage"));
                            }
                            current_page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ItemsPerPage => {
                            if items_per_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemsPerPage"));
                            }
                            items_per_page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PaginationResponse {
                    total_items: total_items__.unwrap_or_default(),
                    total_pages: total_pages__.unwrap_or_default(),
                    current_page: current_page__.unwrap_or_default(),
                    items_per_page: items_per_page__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.PaginationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_DIRECTION_UNSPECIFIED",
            Self::Asc => "SORT_DIRECTION_ASC",
            Self::Desc => "SORT_DIRECTION_DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_DIRECTION_UNSPECIFIED",
            "SORT_DIRECTION_ASC",
            "SORT_DIRECTION_DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortDirection;

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
                    "SORT_DIRECTION_UNSPECIFIED" => Ok(SortDirection::Unspecified),
                    "SORT_DIRECTION_ASC" => Ok(SortDirection::Asc),
                    "SORT_DIRECTION_DESC" => Ok(SortDirection::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATUS_UNSPECIFIED",
            Self::Active => "STATUS_ACTIVE",
            Self::Inactive => "STATUS_INACTIVE",
            Self::Deleted => "STATUS_DELETED",
            Self::Campaign => "STATUS_CAMPAIGN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATUS_UNSPECIFIED",
            "STATUS_ACTIVE",
            "STATUS_INACTIVE",
            "STATUS_DELETED",
            "STATUS_CAMPAIGN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

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
                    "STATUS_UNSPECIFIED" => Ok(Status::Unspecified),
                    "STATUS_ACTIVE" => Ok(Status::Active),
                    "STATUS_INACTIVE" => Ok(Status::Inactive),
                    "STATUS_DELETED" => Ok(Status::Deleted),
                    "STATUS_CAMPAIGN" => Ok(Status::Campaign),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Timestamp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seconds != 0 {
            len += 1;
        }
        if self.nanos != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Timestamp", len)?;
        if self.seconds != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("seconds", ToString::to_string(&self.seconds).as_str())?;
        }
        if self.nanos != 0 {
            struct_ser.serialize_field("nanos", &self.nanos)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Timestamp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
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
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Timestamp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Timestamp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Timestamp {
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Timestamp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Translation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.common.v1.Translation", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Translation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = Translation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.common.v1.Translation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Translation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Translation {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.common.v1.Translation", FIELDS, GeneratedVisitor)
    }
}
