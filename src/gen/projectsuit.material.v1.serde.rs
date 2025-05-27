// @generated
impl serde::Serialize for CreateMaterialRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.material.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.CreateMaterialRequest", len)?;
        if let Some(v) = self.material.as_ref() {
            struct_ser.serialize_field("material", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateMaterialRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "material",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Material,
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
                            "material" => Ok(GeneratedField::Material),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateMaterialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.CreateMaterialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMaterialRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut material__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Material => {
                            if material__.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            material__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateMaterialRequest {
                    material: material__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.CreateMaterialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateMaterialResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.CreateMaterialResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateMaterialResponse {
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
            type Value = CreateMaterialResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.CreateMaterialResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMaterialResponse, V::Error>
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
                Ok(CreateMaterialResponse {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.CreateMaterialResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMaterialRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.DeleteMaterialRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMaterialRequest {
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
            type Value = DeleteMaterialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.DeleteMaterialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMaterialRequest, V::Error>
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
                Ok(DeleteMaterialRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.DeleteMaterialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMaterialResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.DeleteMaterialResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMaterialResponse {
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
            type Value = DeleteMaterialResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.DeleteMaterialResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMaterialResponse, V::Error>
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
                Ok(DeleteMaterialResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.DeleteMaterialResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMaterialRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.GetMaterialRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMaterialRequest {
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
            type Value = GetMaterialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.GetMaterialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMaterialRequest, V::Error>
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
                Ok(GetMaterialRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.GetMaterialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMaterialResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.material.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.GetMaterialResponse", len)?;
        if let Some(v) = self.material.as_ref() {
            struct_ser.serialize_field("material", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMaterialResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "material",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Material,
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
                            "material" => Ok(GeneratedField::Material),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMaterialResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.GetMaterialResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMaterialResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut material__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Material => {
                            if material__.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            material__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMaterialResponse {
                    material: material__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.GetMaterialResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMaterialsRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.ListMaterialsRequest", len)?;
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMaterialsRequest {
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
            type Value = ListMaterialsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.ListMaterialsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMaterialsRequest, V::Error>
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
                Ok(ListMaterialsRequest {
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.ListMaterialsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMaterialsResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.ListMaterialsResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMaterialsResponse {
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
            type Value = ListMaterialsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.ListMaterialsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMaterialsResponse, V::Error>
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
                Ok(ListMaterialsResponse {
                    items: items__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.ListMaterialsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Material {
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
        if self.description.is_some() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if self.cost.is_some() {
            len += 1;
        }
        if self.is_purchasable {
            len += 1;
        }
        if self.is_producible {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.Material", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if let Some(v) = self.cost.as_ref() {
            struct_ser.serialize_field("cost", v)?;
        }
        if self.is_purchasable {
            struct_ser.serialize_field("isPurchasable", &self.is_purchasable)?;
        }
        if self.is_producible {
            struct_ser.serialize_field("isProducible", &self.is_producible)?;
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
impl<'de> serde::Deserialize<'de> for Material {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "unit",
            "cost",
            "is_purchasable",
            "isPurchasable",
            "is_producible",
            "isProducible",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            Unit,
            Cost,
            IsPurchasable,
            IsProducible,
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
                            "description" => Ok(GeneratedField::Description),
                            "unit" => Ok(GeneratedField::Unit),
                            "cost" => Ok(GeneratedField::Cost),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "isProducible" | "is_producible" => Ok(GeneratedField::IsProducible),
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
            type Value = Material;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.Material")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Material, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut unit__ = None;
                let mut cost__ = None;
                let mut is_purchasable__ = None;
                let mut is_producible__ = None;
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
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cost => {
                            if cost__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            cost__ = map_.next_value()?;
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsProducible => {
                            if is_producible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isProducible"));
                            }
                            is_producible__ = Some(map_.next_value()?);
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
                Ok(Material {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    unit: unit__.unwrap_or_default(),
                    cost: cost__,
                    is_purchasable: is_purchasable__.unwrap_or_default(),
                    is_producible: is_producible__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.Material", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MaterialInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if self.cost.is_some() {
            len += 1;
        }
        if self.is_purchasable {
            len += 1;
        }
        if self.is_producible {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.MaterialInput", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if let Some(v) = self.cost.as_ref() {
            struct_ser.serialize_field("cost", v)?;
        }
        if self.is_purchasable {
            struct_ser.serialize_field("isPurchasable", &self.is_purchasable)?;
        }
        if self.is_producible {
            struct_ser.serialize_field("isProducible", &self.is_producible)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MaterialInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "unit",
            "cost",
            "is_purchasable",
            "isPurchasable",
            "is_producible",
            "isProducible",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Unit,
            Cost,
            IsPurchasable,
            IsProducible,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "unit" => Ok(GeneratedField::Unit),
                            "cost" => Ok(GeneratedField::Cost),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "isProducible" | "is_producible" => Ok(GeneratedField::IsProducible),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MaterialInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.MaterialInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MaterialInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut unit__ = None;
                let mut cost__ = None;
                let mut is_purchasable__ = None;
                let mut is_producible__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cost => {
                            if cost__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            cost__ = map_.next_value()?;
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsProducible => {
                            if is_producible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isProducible"));
                            }
                            is_producible__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MaterialInput {
                    name: name__,
                    description: description__,
                    unit: unit__.unwrap_or_default(),
                    cost: cost__,
                    is_purchasable: is_purchasable__.unwrap_or_default(),
                    is_producible: is_producible__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.MaterialInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MaterialView {
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
        if self.description.is_some() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if self.cost.is_some() {
            len += 1;
        }
        if self.is_purchasable {
            len += 1;
        }
        if self.is_producible {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.MaterialView", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if let Some(v) = self.cost.as_ref() {
            struct_ser.serialize_field("cost", v)?;
        }
        if self.is_purchasable {
            struct_ser.serialize_field("isPurchasable", &self.is_purchasable)?;
        }
        if self.is_producible {
            struct_ser.serialize_field("isProducible", &self.is_producible)?;
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
impl<'de> serde::Deserialize<'de> for MaterialView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "unit",
            "cost",
            "is_purchasable",
            "isPurchasable",
            "is_producible",
            "isProducible",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            Unit,
            Cost,
            IsPurchasable,
            IsProducible,
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
                            "description" => Ok(GeneratedField::Description),
                            "unit" => Ok(GeneratedField::Unit),
                            "cost" => Ok(GeneratedField::Cost),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "isProducible" | "is_producible" => Ok(GeneratedField::IsProducible),
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
            type Value = MaterialView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.MaterialView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MaterialView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut unit__ = None;
                let mut cost__ = None;
                let mut is_purchasable__ = None;
                let mut is_producible__ = None;
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
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cost => {
                            if cost__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            cost__ = map_.next_value()?;
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsProducible => {
                            if is_producible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isProducible"));
                            }
                            is_producible__ = Some(map_.next_value()?);
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
                Ok(MaterialView {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    unit: unit__.unwrap_or_default(),
                    cost: cost__,
                    is_purchasable: is_purchasable__.unwrap_or_default(),
                    is_producible: is_producible__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.MaterialView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMaterialRequest {
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
        if self.material.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.UpdateMaterialRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.material.as_ref() {
            struct_ser.serialize_field("material", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMaterialRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "material",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Material,
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
                            "material" => Ok(GeneratedField::Material),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMaterialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.UpdateMaterialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMaterialRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut material__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Material => {
                            if material__.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            material__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateMaterialRequest {
                    id: id__.unwrap_or_default(),
                    material: material__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.UpdateMaterialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMaterialResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.UpdateMaterialResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMaterialResponse {
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
            type Value = UpdateMaterialResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.UpdateMaterialResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMaterialResponse, V::Error>
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
                Ok(UpdateMaterialResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.UpdateMaterialResponse", FIELDS, GeneratedVisitor)
    }
}
