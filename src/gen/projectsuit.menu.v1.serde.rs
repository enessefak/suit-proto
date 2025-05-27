// @generated
impl serde::Serialize for CreateMenuRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.menu.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.CreateMenuRequest", len)?;
        if let Some(v) = self.menu.as_ref() {
            struct_ser.serialize_field("menu", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateMenuRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "menu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Menu,
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
                            "menu" => Ok(GeneratedField::Menu),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.CreateMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMenuRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut menu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Menu => {
                            if menu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menu"));
                            }
                            menu__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateMenuRequest {
                    menu: menu__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.CreateMenuRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateMenuResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.CreateMenuResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateMenuResponse {
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
            type Value = CreateMenuResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.CreateMenuResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMenuResponse, V::Error>
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
                Ok(CreateMenuResponse {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.CreateMenuResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMenuRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.DeleteMenuRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMenuRequest {
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
            type Value = DeleteMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.DeleteMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMenuRequest, V::Error>
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
                Ok(DeleteMenuRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.DeleteMenuRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMenuResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.DeleteMenuResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMenuResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteMenuResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.DeleteMenuResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMenuResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteMenuResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.DeleteMenuResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMenuRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.GetMenuRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMenuRequest {
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
            type Value = GetMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.GetMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMenuRequest, V::Error>
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
                Ok(GetMenuRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.GetMenuRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMenuResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.menu.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.GetMenuResponse", len)?;
        if let Some(v) = self.menu.as_ref() {
            struct_ser.serialize_field("menu", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMenuResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "menu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Menu,
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
                            "menu" => Ok(GeneratedField::Menu),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMenuResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.GetMenuResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMenuResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut menu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Menu => {
                            if menu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menu"));
                            }
                            menu__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMenuResponse {
                    menu: menu__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.GetMenuResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMenusRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.ListMenusRequest", len)?;
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMenusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
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
                            "page" => Ok(GeneratedField::Page),
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
            type Value = ListMenusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.ListMenusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMenusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                let mut page_size__ = None;
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
                    }
                }
                Ok(ListMenusRequest {
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.ListMenusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMenusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.ListMenusResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMenusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            Total,
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
                            "items" => Ok(GeneratedField::Items),
                            "total" => Ok(GeneratedField::Total),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMenusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.ListMenusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMenusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListMenusResponse {
                    items: items__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.ListMenusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Menu {
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
        if self.name.is_some() {
            len += 1;
        }
        if !self.sections.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.Menu", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.sections.is_empty() {
            struct_ser.serialize_field("sections", &self.sections)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Menu {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "sections",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Sections,
            IsActive,
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
                            "name" => Ok(GeneratedField::Name),
                            "sections" => Ok(GeneratedField::Sections),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = Menu;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.Menu")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Menu, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut sections__ = None;
                let mut is_active__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Sections => {
                            if sections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sections"));
                            }
                            sections__ = Some(map_.next_value()?);
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
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Menu {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    sections: sections__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.Menu", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MenuInput {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.MenuInput", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MenuInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "name",
            "description",
            "items",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Description,
            Items,
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
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "items" => Ok(GeneratedField::Items),
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
            type Value = MenuInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.MenuInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MenuInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut items__ = None;
                let mut is_active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MenuInput {
                    code: code__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    items: items__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.MenuInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MenuItemInput {
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
        if !self.variant_id.is_empty() {
            len += 1;
        }
        if self.quantity != 0. {
            len += 1;
        }
        if self.optional {
            len += 1;
        }
        if self.sort_order != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.MenuItemInput", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.variant_id.is_empty() {
            struct_ser.serialize_field("variantId", &self.variant_id)?;
        }
        if self.quantity != 0. {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if self.optional {
            struct_ser.serialize_field("optional", &self.optional)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MenuItemInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "quantity",
            "optional",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            VariantId,
            Quantity,
            Optional,
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
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "optional" => Ok(GeneratedField::Optional),
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
            type Value = MenuItemInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.MenuItemInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MenuItemInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut quantity__ = None;
                let mut optional__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Optional => {
                            if optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optional"));
                            }
                            optional__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MenuItemInput {
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    optional: optional__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.MenuItemInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MenuItemView {
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
        if !self.variant_id.is_empty() {
            len += 1;
        }
        if self.quantity != 0. {
            len += 1;
        }
        if self.optional {
            len += 1;
        }
        if self.sort_order != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.MenuItemView", len)?;
        if let Some(v) = self.product.as_ref() {
            struct_ser.serialize_field("product", v)?;
        }
        if !self.variant_id.is_empty() {
            struct_ser.serialize_field("variantId", &self.variant_id)?;
        }
        if self.quantity != 0. {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if self.optional {
            struct_ser.serialize_field("optional", &self.optional)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MenuItemView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product",
            "variant_id",
            "variantId",
            "quantity",
            "optional",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Product,
            VariantId,
            Quantity,
            Optional,
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
                            "product" => Ok(GeneratedField::Product),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "optional" => Ok(GeneratedField::Optional),
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
            type Value = MenuItemView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.MenuItemView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MenuItemView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product__ = None;
                let mut variant_id__ = None;
                let mut quantity__ = None;
                let mut optional__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Product => {
                            if product__.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            product__ = map_.next_value()?;
                        }
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Optional => {
                            if optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optional"));
                            }
                            optional__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MenuItemView {
                    product: product__,
                    variant_id: variant_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    optional: optional__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.MenuItemView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MenuSection {
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
        if self.title.is_some() {
            len += 1;
        }
        if !self.product_ids.is_empty() {
            len += 1;
        }
        if self.sort_order != 0 {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.MenuSection", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if !self.product_ids.is_empty() {
            struct_ser.serialize_field("productIds", &self.product_ids)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MenuSection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "title",
            "product_ids",
            "productIds",
            "sort_order",
            "sortOrder",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Title,
            ProductIds,
            SortOrder,
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
                            "id" => Ok(GeneratedField::Id),
                            "title" => Ok(GeneratedField::Title),
                            "productIds" | "product_ids" => Ok(GeneratedField::ProductIds),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
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
            type Value = MenuSection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.MenuSection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MenuSection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut title__ = None;
                let mut product_ids__ = None;
                let mut sort_order__ = None;
                let mut is_active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                        GeneratedField::ProductIds => {
                            if product_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productIds"));
                            }
                            product_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MenuSection {
                    id: id__.unwrap_or_default(),
                    title: title__,
                    product_ids: product_ids__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.MenuSection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MenuView {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.MenuView", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MenuView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "name",
            "description",
            "items",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            Name,
            Description,
            Items,
            IsActive,
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
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "items" => Ok(GeneratedField::Items),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = MenuView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.MenuView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MenuView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut items__ = None;
                let mut is_active__ = None;
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
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
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
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MenuView {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    items: items__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.MenuView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMenuRequest {
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
        if self.menu.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.UpdateMenuRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.menu.as_ref() {
            struct_ser.serialize_field("menu", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMenuRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "menu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Menu,
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
                            "menu" => Ok(GeneratedField::Menu),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.UpdateMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMenuRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut menu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Menu => {
                            if menu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menu"));
                            }
                            menu__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateMenuRequest {
                    id: id__.unwrap_or_default(),
                    menu: menu__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.UpdateMenuRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMenuResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.UpdateMenuResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMenuResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMenuResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.UpdateMenuResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMenuResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateMenuResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.UpdateMenuResponse", FIELDS, GeneratedVisitor)
    }
}
