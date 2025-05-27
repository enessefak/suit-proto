// @generated
impl serde::Serialize for AddItemToMenuRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.menu_id.is_empty() {
            len += 1;
        }
        if self.product_id.is_some() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.optional_item.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.AddItemToMenuRequest", len)?;
        if !self.menu_id.is_empty() {
            struct_ser.serialize_field("menuId", &self.menu_id)?;
        }
        if let Some(v) = self.product_id.as_ref() {
            struct_ser.serialize_field("productId", v)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if let Some(v) = self.optional_item.as_ref() {
            struct_ser.serialize_field("optionalItem", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddItemToMenuRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "menu_id",
            "menuId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "quantity",
            "optional_item",
            "optionalItem",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MenuId,
            ProductId,
            VariantId,
            Quantity,
            OptionalItem,
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
                            "menuId" | "menu_id" => Ok(GeneratedField::MenuId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "optionalItem" | "optional_item" => Ok(GeneratedField::OptionalItem),
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
            type Value = AddItemToMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.AddItemToMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddItemToMenuRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut menu_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut quantity__ = None;
                let mut optional_item__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MenuId => {
                            if menu_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menuId"));
                            }
                            menu_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = map_.next_value()?;
                        }
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionalItem => {
                            if optional_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalItem"));
                            }
                            optional_item__ = map_.next_value()?;
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
                Ok(AddItemToMenuRequest {
                    menu_id: menu_id__.unwrap_or_default(),
                    product_id: product_id__,
                    variant_id: variant_id__,
                    quantity: quantity__.unwrap_or_default(),
                    optional_item: optional_item__,
                    sort_order: sort_order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.AddItemToMenuRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateMenuRequest {
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
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.CreateMenuRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
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
impl<'de> serde::Deserialize<'de> for CreateMenuRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
            type Value = CreateMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.CreateMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMenuRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
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
                Ok(CreateMenuRequest {
                    code: code__.unwrap_or_default(),
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.CreateMenuRequest", FIELDS, GeneratedVisitor)
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
        if self.locale_input.is_some() {
            len += 1;
        }
        if self.include_items {
            len += 1;
        }
        if self.include_item_details {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.GetMenuRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        if self.include_items {
            struct_ser.serialize_field("includeItems", &self.include_items)?;
        }
        if self.include_item_details {
            struct_ser.serialize_field("includeItemDetails", &self.include_item_details)?;
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
            "locale_input",
            "localeInput",
            "include_items",
            "includeItems",
            "include_item_details",
            "includeItemDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            LocaleInput,
            IncludeItems,
            IncludeItemDetails,
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
                            "includeItems" | "include_items" => Ok(GeneratedField::IncludeItems),
                            "includeItemDetails" | "include_item_details" => Ok(GeneratedField::IncludeItemDetails),
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
                let mut locale_input__ = None;
                let mut include_items__ = None;
                let mut include_item_details__ = None;
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
                        GeneratedField::IncludeItems => {
                            if include_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeItems"));
                            }
                            include_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeItemDetails => {
                            if include_item_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeItemDetails"));
                            }
                            include_item_details__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetMenuRequest {
                    id: id__.unwrap_or_default(),
                    locale_input: locale_input__,
                    include_items: include_items__.unwrap_or_default(),
                    include_item_details: include_item_details__.unwrap_or_default(),
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
        if self.base_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.ListMenusRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
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
            type Value = ListMenusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.ListMenusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMenusRequest, V::Error>
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
                Ok(ListMenusRequest {
                    base_request: base_request__,
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
        if !self.menus.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.ListMenusResponse", len)?;
        if !self.menus.is_empty() {
            struct_ser.serialize_field("menus", &self.menus)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
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
            "menus",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Menus,
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
                            "menus" => Ok(GeneratedField::Menus),
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
            type Value = ListMenusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.ListMenusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMenusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut menus__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Menus => {
                            if menus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menus"));
                            }
                            menus__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMenusResponse {
                    menus: menus__.unwrap_or_default(),
                    pagination: pagination__,
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
        if !self.code.is_empty() {
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
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.Menu", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
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
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
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
            "code",
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
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            AllTranslations,
            Items,
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
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "items" => Ok(GeneratedField::Items),
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
                let mut code__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut all_translations__ = None;
                let mut items__ = None;
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
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Menu {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.Menu", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MenuItem {
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
        if self.product_id.is_some() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.optional_item {
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
        if self.product_details.is_some() {
            len += 1;
        }
        if self.variant_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.MenuItem", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.product_id.as_ref() {
            struct_ser.serialize_field("productId", v)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if self.optional_item {
            struct_ser.serialize_field("optionalItem", &self.optional_item)?;
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
        if let Some(v) = self.product_details.as_ref() {
            struct_ser.serialize_field("productDetails", v)?;
        }
        if let Some(v) = self.variant_details.as_ref() {
            struct_ser.serialize_field("variantDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MenuItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "quantity",
            "optional_item",
            "optionalItem",
            "sort_order",
            "sortOrder",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "product_details",
            "productDetails",
            "variant_details",
            "variantDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProductId,
            VariantId,
            Quantity,
            OptionalItem,
            SortOrder,
            CreatedAt,
            UpdatedAt,
            ProductDetails,
            VariantDetails,
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
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "optionalItem" | "optional_item" => Ok(GeneratedField::OptionalItem),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "productDetails" | "product_details" => Ok(GeneratedField::ProductDetails),
                            "variantDetails" | "variant_details" => Ok(GeneratedField::VariantDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MenuItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.MenuItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MenuItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut quantity__ = None;
                let mut optional_item__ = None;
                let mut sort_order__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut product_details__ = None;
                let mut variant_details__ = None;
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
                            product_id__ = map_.next_value()?;
                        }
                        GeneratedField::VariantId => {
                            if variant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantId"));
                            }
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionalItem => {
                            if optional_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalItem"));
                            }
                            optional_item__ = Some(map_.next_value()?);
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
                        GeneratedField::ProductDetails => {
                            if product_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productDetails"));
                            }
                            product_details__ = map_.next_value()?;
                        }
                        GeneratedField::VariantDetails => {
                            if variant_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantDetails"));
                            }
                            variant_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MenuItem {
                    id: id__.unwrap_or_default(),
                    product_id: product_id__,
                    variant_id: variant_id__,
                    quantity: quantity__.unwrap_or_default(),
                    optional_item: optional_item__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    product_details: product_details__,
                    variant_details: variant_details__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.MenuItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMenuItemRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.menu_item_id.is_empty() {
            len += 1;
        }
        if self.quantity.is_some() {
            len += 1;
        }
        if self.optional_item.is_some() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.UpdateMenuItemRequest", len)?;
        if !self.menu_item_id.is_empty() {
            struct_ser.serialize_field("menuItemId", &self.menu_item_id)?;
        }
        if let Some(v) = self.quantity.as_ref() {
            struct_ser.serialize_field("quantity", v)?;
        }
        if let Some(v) = self.optional_item.as_ref() {
            struct_ser.serialize_field("optionalItem", v)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMenuItemRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "menu_item_id",
            "menuItemId",
            "quantity",
            "optional_item",
            "optionalItem",
            "sort_order",
            "sortOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MenuItemId,
            Quantity,
            OptionalItem,
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
                            "menuItemId" | "menu_item_id" => Ok(GeneratedField::MenuItemId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "optionalItem" | "optional_item" => Ok(GeneratedField::OptionalItem),
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
            type Value = UpdateMenuItemRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.UpdateMenuItemRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMenuItemRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut menu_item_id__ = None;
                let mut quantity__ = None;
                let mut optional_item__ = None;
                let mut sort_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MenuItemId => {
                            if menu_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menuItemId"));
                            }
                            menu_item_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = map_.next_value()?;
                        }
                        GeneratedField::OptionalItem => {
                            if optional_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalItem"));
                            }
                            optional_item__ = map_.next_value()?;
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
                Ok(UpdateMenuItemRequest {
                    menu_item_id: menu_item_id__.unwrap_or_default(),
                    quantity: quantity__,
                    optional_item: optional_item__,
                    sort_order: sort_order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.UpdateMenuItemRequest", FIELDS, GeneratedVisitor)
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
        if self.code.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.menu.v1.UpdateMenuRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateMenuRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
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
            type Value = UpdateMenuRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.menu.v1.UpdateMenuRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMenuRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
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
                Ok(UpdateMenuRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.menu.v1.UpdateMenuRequest", FIELDS, GeneratedVisitor)
    }
}
