// @generated
impl serde::Serialize for CreateRecipeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.recipe.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.CreateRecipeRequest", len)?;
        if let Some(v) = self.recipe.as_ref() {
            struct_ser.serialize_field("recipe", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRecipeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recipe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipe,
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
                            "recipe" => Ok(GeneratedField::Recipe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRecipeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.CreateRecipeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRecipeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipe__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Recipe => {
                            if recipe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipe"));
                            }
                            recipe__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateRecipeRequest {
                    recipe: recipe__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.CreateRecipeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRecipeResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.CreateRecipeResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRecipeResponse {
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
            type Value = CreateRecipeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.CreateRecipeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRecipeResponse, V::Error>
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
                Ok(CreateRecipeResponse {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.CreateRecipeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRecipeRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.DeleteRecipeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRecipeRequest {
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
            type Value = DeleteRecipeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.DeleteRecipeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRecipeRequest, V::Error>
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
                Ok(DeleteRecipeRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.DeleteRecipeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRecipeResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.DeleteRecipeResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRecipeResponse {
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
            type Value = DeleteRecipeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.DeleteRecipeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRecipeResponse, V::Error>
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
                Ok(DeleteRecipeResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.DeleteRecipeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRecipeRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.GetRecipeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRecipeRequest {
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
            type Value = GetRecipeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.GetRecipeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRecipeRequest, V::Error>
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
                Ok(GetRecipeRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.GetRecipeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRecipeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.recipe.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.GetRecipeResponse", len)?;
        if let Some(v) = self.recipe.as_ref() {
            struct_ser.serialize_field("recipe", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRecipeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recipe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipe,
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
                            "recipe" => Ok(GeneratedField::Recipe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRecipeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.GetRecipeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRecipeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipe__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Recipe => {
                            if recipe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipe"));
                            }
                            recipe__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRecipeResponse {
                    recipe: recipe__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.GetRecipeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRecipesRequest {
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
        if !self.product_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.ListRecipesRequest", len)?;
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRecipesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
            "page_size",
            "pageSize",
            "product_id",
            "productId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
            PageSize,
            ProductId,
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
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRecipesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.ListRecipesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRecipesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                let mut page_size__ = None;
                let mut product_id__ = None;
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
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListRecipesRequest {
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.ListRecipesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRecipesResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.ListRecipesResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRecipesResponse {
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
            type Value = ListRecipesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.ListRecipesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRecipesResponse, V::Error>
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
                Ok(ListRecipesResponse {
                    items: items__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.ListRecipesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Recipe {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.Recipe", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
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
impl<'de> serde::Deserialize<'de> for Recipe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "product_id",
            "productId",
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
            ProductId,
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
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
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
            type Value = Recipe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.Recipe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Recipe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_id__ = None;
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
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
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
                Ok(Recipe {
                    id: id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.Recipe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecipeInput {
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
        if !self.items.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.RecipeInput", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
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
impl<'de> serde::Deserialize<'de> for RecipeInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "items",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
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
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
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
            type Value = RecipeInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.RecipeInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecipeInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut items__ = None;
                let mut is_active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
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
                Ok(RecipeInput {
                    product_id: product_id__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.RecipeInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecipeItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.material_id.is_empty() {
            len += 1;
        }
        if self.quantity != 0. {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.RecipeItem", len)?;
        if !self.material_id.is_empty() {
            struct_ser.serialize_field("materialId", &self.material_id)?;
        }
        if self.quantity != 0. {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecipeItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "material_id",
            "materialId",
            "quantity",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaterialId,
            Quantity,
            Unit,
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
                            "materialId" | "material_id" => Ok(GeneratedField::MaterialId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecipeItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.RecipeItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecipeItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut material_id__ = None;
                let mut quantity__ = None;
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaterialId => {
                            if material_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("materialId"));
                            }
                            material_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RecipeItem {
                    material_id: material_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.RecipeItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecipeView {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.RecipeView", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
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
impl<'de> serde::Deserialize<'de> for RecipeView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "product_id",
            "productId",
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
            ProductId,
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
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
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
            type Value = RecipeView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.RecipeView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecipeView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_id__ = None;
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
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
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
                Ok(RecipeView {
                    id: id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.RecipeView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRecipeRequest {
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
        if self.recipe.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.UpdateRecipeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.recipe.as_ref() {
            struct_ser.serialize_field("recipe", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRecipeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "recipe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Recipe,
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
                            "recipe" => Ok(GeneratedField::Recipe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRecipeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.UpdateRecipeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRecipeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut recipe__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipe => {
                            if recipe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipe"));
                            }
                            recipe__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRecipeRequest {
                    id: id__.unwrap_or_default(),
                    recipe: recipe__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.UpdateRecipeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRecipeResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.recipe.v1.UpdateRecipeResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRecipeResponse {
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
            type Value = UpdateRecipeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.recipe.v1.UpdateRecipeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRecipeResponse, V::Error>
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
                Ok(UpdateRecipeResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.recipe.v1.UpdateRecipeResponse", FIELDS, GeneratedVisitor)
    }
}
