// @generated
impl serde::Serialize for AddMaterialToProductRequest {
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
        if !self.material_id.is_empty() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.wastage_percentage.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.AddMaterialToProductRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if !self.material_id.is_empty() {
            struct_ser.serialize_field("materialId", &self.material_id)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.wastage_percentage.as_ref() {
            struct_ser.serialize_field("wastagePercentage", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddMaterialToProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "material_id",
            "materialId",
            "quantity",
            "unit_id",
            "unitId",
            "wastage_percentage",
            "wastagePercentage",
            "notes",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            MaterialId,
            Quantity,
            UnitId,
            WastagePercentage,
            Notes,
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
                            "materialId" | "material_id" => Ok(GeneratedField::MaterialId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "wastagePercentage" | "wastage_percentage" => Ok(GeneratedField::WastagePercentage),
                            "notes" => Ok(GeneratedField::Notes),
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
            type Value = AddMaterialToProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.AddMaterialToProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddMaterialToProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut material_id__ = None;
                let mut quantity__ = None;
                let mut unit_id__ = None;
                let mut wastage_percentage__ = None;
                let mut notes__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
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
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WastagePercentage => {
                            if wastage_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wastagePercentage"));
                            }
                            wastage_percentage__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
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
                Ok(AddMaterialToProductRequest {
                    product_id: product_id__.unwrap_or_default(),
                    material_id: material_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    wastage_percentage: wastage_percentage__,
                    notes: notes__,
                    sort_order: sort_order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.AddMaterialToProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateMaterialRequest {
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
        if !self.default_unit_id.is_empty() {
            len += 1;
        }
        if self.is_purchasable.is_some() {
            len += 1;
        }
        if self.is_producible.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.CreateMaterialRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.default_unit_id.is_empty() {
            struct_ser.serialize_field("defaultUnitId", &self.default_unit_id)?;
        }
        if let Some(v) = self.is_purchasable.as_ref() {
            struct_ser.serialize_field("isPurchasable", v)?;
        }
        if let Some(v) = self.is_producible.as_ref() {
            struct_ser.serialize_field("isProducible", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
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
            "code",
            "default_unit_id",
            "defaultUnitId",
            "is_purchasable",
            "isPurchasable",
            "is_producible",
            "isProducible",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            DefaultUnitId,
            IsPurchasable,
            IsProducible,
            IsActive,
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
                            "code" => Ok(GeneratedField::Code),
                            "defaultUnitId" | "default_unit_id" => Ok(GeneratedField::DefaultUnitId),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "isProducible" | "is_producible" => Ok(GeneratedField::IsProducible),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = CreateMaterialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.CreateMaterialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMaterialRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut default_unit_id__ = None;
                let mut is_purchasable__ = None;
                let mut is_producible__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultUnitId => {
                            if default_unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultUnitId"));
                            }
                            default_unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = map_.next_value()?;
                        }
                        GeneratedField::IsProducible => {
                            if is_producible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isProducible"));
                            }
                            is_producible__ = map_.next_value()?;
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
                    }
                }
                Ok(CreateMaterialRequest {
                    code: code__.unwrap_or_default(),
                    default_unit_id: default_unit_id__.unwrap_or_default(),
                    is_purchasable: is_purchasable__,
                    is_producible: is_producible__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.CreateMaterialRequest", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for GetProductMaterialsRequest {
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
        if self.locale_input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.GetProductMaterialsRequest", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProductMaterialsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_id",
            "productId",
            "locale_input",
            "localeInput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            LocaleInput,
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
                            "localeInput" | "locale_input" => Ok(GeneratedField::LocaleInput),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProductMaterialsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.GetProductMaterialsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProductMaterialsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut locale_input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetProductMaterialsRequest {
                    product_id: product_id__.unwrap_or_default(),
                    locale_input: locale_input__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.GetProductMaterialsRequest", FIELDS, GeneratedVisitor)
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
        if self.base_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.ListMaterialsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
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
            "base_request",
            "baseRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
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
                let mut base_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMaterialsRequest {
                    base_request: base_request__,
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
        if !self.materials.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.ListMaterialsResponse", len)?;
        if !self.materials.is_empty() {
            struct_ser.serialize_field("materials", &self.materials)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
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
            "materials",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Materials,
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
                            "materials" => Ok(GeneratedField::Materials),
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
            type Value = ListMaterialsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.ListMaterialsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMaterialsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut materials__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Materials => {
                            if materials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("materials"));
                            }
                            materials__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMaterialsResponse {
                    materials: materials__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.ListMaterialsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProductMaterialsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bom_items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.ListProductMaterialsResponse", len)?;
        if !self.bom_items.is_empty() {
            struct_ser.serialize_field("bomItems", &self.bom_items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProductMaterialsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bom_items",
            "bomItems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BomItems,
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
                            "bomItems" | "bom_items" => Ok(GeneratedField::BomItems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProductMaterialsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.ListProductMaterialsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProductMaterialsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bom_items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BomItems => {
                            if bom_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bomItems"));
                            }
                            bom_items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListProductMaterialsResponse {
                    bom_items: bom_items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.ListProductMaterialsResponse", FIELDS, GeneratedVisitor)
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
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.default_unit_id.is_empty() {
            len += 1;
        }
        if self.default_unit_details.is_some() {
            len += 1;
        }
        if self.is_purchasable {
            len += 1;
        }
        if self.is_producible {
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
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.Material", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.default_unit_id.is_empty() {
            struct_ser.serialize_field("defaultUnitId", &self.default_unit_id)?;
        }
        if let Some(v) = self.default_unit_details.as_ref() {
            struct_ser.serialize_field("defaultUnitDetails", v)?;
        }
        if self.is_purchasable {
            struct_ser.serialize_field("isPurchasable", &self.is_purchasable)?;
        }
        if self.is_producible {
            struct_ser.serialize_field("isProducible", &self.is_producible)?;
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
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
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
            "code",
            "default_unit_id",
            "defaultUnitId",
            "default_unit_details",
            "defaultUnitDetails",
            "is_purchasable",
            "isPurchasable",
            "is_producible",
            "isProducible",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "description",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            DefaultUnitId,
            DefaultUnitDetails,
            IsPurchasable,
            IsProducible,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
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
                            "code" => Ok(GeneratedField::Code),
                            "defaultUnitId" | "default_unit_id" => Ok(GeneratedField::DefaultUnitId),
                            "defaultUnitDetails" | "default_unit_details" => Ok(GeneratedField::DefaultUnitDetails),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "isProducible" | "is_producible" => Ok(GeneratedField::IsProducible),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
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
            type Value = Material;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.Material")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Material, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut default_unit_id__ = None;
                let mut default_unit_details__ = None;
                let mut is_purchasable__ = None;
                let mut is_producible__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut all_translations__ = None;
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
                        GeneratedField::DefaultUnitId => {
                            if default_unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultUnitId"));
                            }
                            default_unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultUnitDetails => {
                            if default_unit_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultUnitDetails"));
                            }
                            default_unit_details__ = map_.next_value()?;
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
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Material {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    default_unit_id: default_unit_id__.unwrap_or_default(),
                    default_unit_details: default_unit_details__,
                    is_purchasable: is_purchasable__.unwrap_or_default(),
                    is_producible: is_producible__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.Material", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProductMaterial {
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
        if self.material_details.is_some() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.unit_details.is_some() {
            len += 1;
        }
        if !self.wastage_percentage.is_empty() {
            len += 1;
        }
        if self.notes.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.ProductMaterial", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.material_details.as_ref() {
            struct_ser.serialize_field("materialDetails", v)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if let Some(v) = self.unit_details.as_ref() {
            struct_ser.serialize_field("unitDetails", v)?;
        }
        if !self.wastage_percentage.is_empty() {
            struct_ser.serialize_field("wastagePercentage", &self.wastage_percentage)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
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
impl<'de> serde::Deserialize<'de> for ProductMaterial {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "material_details",
            "materialDetails",
            "quantity",
            "unit_details",
            "unitDetails",
            "wastage_percentage",
            "wastagePercentage",
            "notes",
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
            MaterialDetails,
            Quantity,
            UnitDetails,
            WastagePercentage,
            Notes,
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
                            "materialDetails" | "material_details" => Ok(GeneratedField::MaterialDetails),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "unitDetails" | "unit_details" => Ok(GeneratedField::UnitDetails),
                            "wastagePercentage" | "wastage_percentage" => Ok(GeneratedField::WastagePercentage),
                            "notes" => Ok(GeneratedField::Notes),
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
            type Value = ProductMaterial;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.ProductMaterial")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductMaterial, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut material_details__ = None;
                let mut quantity__ = None;
                let mut unit_details__ = None;
                let mut wastage_percentage__ = None;
                let mut notes__ = None;
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
                        GeneratedField::MaterialDetails => {
                            if material_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("materialDetails"));
                            }
                            material_details__ = map_.next_value()?;
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnitDetails => {
                            if unit_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitDetails"));
                            }
                            unit_details__ = map_.next_value()?;
                        }
                        GeneratedField::WastagePercentage => {
                            if wastage_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wastagePercentage"));
                            }
                            wastage_percentage__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
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
                Ok(ProductMaterial {
                    id: id__.unwrap_or_default(),
                    material_details: material_details__,
                    quantity: quantity__.unwrap_or_default(),
                    unit_details: unit_details__,
                    wastage_percentage: wastage_percentage__.unwrap_or_default(),
                    notes: notes__,
                    sort_order: sort_order__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.ProductMaterial", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveMaterialFromProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_material_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.RemoveMaterialFromProductRequest", len)?;
        if !self.product_material_id.is_empty() {
            struct_ser.serialize_field("productMaterialId", &self.product_material_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveMaterialFromProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_material_id",
            "productMaterialId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductMaterialId,
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
                            "productMaterialId" | "product_material_id" => Ok(GeneratedField::ProductMaterialId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveMaterialFromProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.RemoveMaterialFromProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveMaterialFromProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_material_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductMaterialId => {
                            if product_material_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productMaterialId"));
                            }
                            product_material_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveMaterialFromProductRequest {
                    product_material_id: product_material_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.RemoveMaterialFromProductRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMaterialInProductRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.product_material_id.is_empty() {
            len += 1;
        }
        if self.quantity.is_some() {
            len += 1;
        }
        if self.unit_id.is_some() {
            len += 1;
        }
        if self.wastage_percentage.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.UpdateMaterialInProductRequest", len)?;
        if !self.product_material_id.is_empty() {
            struct_ser.serialize_field("productMaterialId", &self.product_material_id)?;
        }
        if let Some(v) = self.quantity.as_ref() {
            struct_ser.serialize_field("quantity", v)?;
        }
        if let Some(v) = self.unit_id.as_ref() {
            struct_ser.serialize_field("unitId", v)?;
        }
        if let Some(v) = self.wastage_percentage.as_ref() {
            struct_ser.serialize_field("wastagePercentage", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMaterialInProductRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "product_material_id",
            "productMaterialId",
            "quantity",
            "unit_id",
            "unitId",
            "wastage_percentage",
            "wastagePercentage",
            "notes",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductMaterialId,
            Quantity,
            UnitId,
            WastagePercentage,
            Notes,
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
                            "productMaterialId" | "product_material_id" => Ok(GeneratedField::ProductMaterialId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "wastagePercentage" | "wastage_percentage" => Ok(GeneratedField::WastagePercentage),
                            "notes" => Ok(GeneratedField::Notes),
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
            type Value = UpdateMaterialInProductRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.UpdateMaterialInProductRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMaterialInProductRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_material_id__ = None;
                let mut quantity__ = None;
                let mut unit_id__ = None;
                let mut wastage_percentage__ = None;
                let mut notes__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProductMaterialId => {
                            if product_material_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productMaterialId"));
                            }
                            product_material_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = map_.next_value()?;
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = map_.next_value()?;
                        }
                        GeneratedField::WastagePercentage => {
                            if wastage_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wastagePercentage"));
                            }
                            wastage_percentage__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
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
                Ok(UpdateMaterialInProductRequest {
                    product_material_id: product_material_id__.unwrap_or_default(),
                    quantity: quantity__,
                    unit_id: unit_id__,
                    wastage_percentage: wastage_percentage__,
                    notes: notes__,
                    sort_order: sort_order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.UpdateMaterialInProductRequest", FIELDS, GeneratedVisitor)
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
        if self.code.is_some() {
            len += 1;
        }
        if self.default_unit_id.is_some() {
            len += 1;
        }
        if self.is_purchasable.is_some() {
            len += 1;
        }
        if self.is_producible.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.material.v1.UpdateMaterialRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.default_unit_id.as_ref() {
            struct_ser.serialize_field("defaultUnitId", v)?;
        }
        if let Some(v) = self.is_purchasable.as_ref() {
            struct_ser.serialize_field("isPurchasable", v)?;
        }
        if let Some(v) = self.is_producible.as_ref() {
            struct_ser.serialize_field("isProducible", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateMaterialRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "default_unit_id",
            "defaultUnitId",
            "is_purchasable",
            "isPurchasable",
            "is_producible",
            "isProducible",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            DefaultUnitId,
            IsPurchasable,
            IsProducible,
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
                            "defaultUnitId" | "default_unit_id" => Ok(GeneratedField::DefaultUnitId),
                            "isPurchasable" | "is_purchasable" => Ok(GeneratedField::IsPurchasable),
                            "isProducible" | "is_producible" => Ok(GeneratedField::IsProducible),
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
            type Value = UpdateMaterialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.material.v1.UpdateMaterialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMaterialRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut default_unit_id__ = None;
                let mut is_purchasable__ = None;
                let mut is_producible__ = None;
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
                        GeneratedField::DefaultUnitId => {
                            if default_unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultUnitId"));
                            }
                            default_unit_id__ = map_.next_value()?;
                        }
                        GeneratedField::IsPurchasable => {
                            if is_purchasable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPurchasable"));
                            }
                            is_purchasable__ = map_.next_value()?;
                        }
                        GeneratedField::IsProducible => {
                            if is_producible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isProducible"));
                            }
                            is_producible__ = map_.next_value()?;
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
                Ok(UpdateMaterialRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    default_unit_id: default_unit_id__,
                    is_purchasable: is_purchasable__,
                    is_producible: is_producible__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.material.v1.UpdateMaterialRequest", FIELDS, GeneratedVisitor)
    }
}
