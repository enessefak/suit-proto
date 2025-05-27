// @generated
impl serde::Serialize for AdjustStockLevelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if !self.product_id.is_empty() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if !self.new_quantity.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.adjusted_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.AdjustStockLevelRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if !self.new_quantity.is_empty() {
            struct_ser.serialize_field("newQuantity", &self.new_quantity)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if let Some(v) = self.adjusted_by_user_id.as_ref() {
            struct_ser.serialize_field("adjustedByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdjustStockLevelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "unit_id",
            "unitId",
            "new_quantity",
            "newQuantity",
            "reason",
            "adjusted_by_user_id",
            "adjustedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            ProductId,
            VariantId,
            UnitId,
            NewQuantity,
            Reason,
            AdjustedByUserId,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "newQuantity" | "new_quantity" => Ok(GeneratedField::NewQuantity),
                            "reason" => Ok(GeneratedField::Reason),
                            "adjustedByUserId" | "adjusted_by_user_id" => Ok(GeneratedField::AdjustedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdjustStockLevelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.AdjustStockLevelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdjustStockLevelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut new_quantity__ = None;
                let mut reason__ = None;
                let mut adjusted_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
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
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewQuantity => {
                            if new_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newQuantity"));
                            }
                            new_quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdjustedByUserId => {
                            if adjusted_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustedByUserId"));
                            }
                            adjusted_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AdjustStockLevelRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    new_quantity: new_quantity__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    adjusted_by_user_id: adjusted_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.AdjustStockLevelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWarehouseRequest {
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
        if !self.branch_type.is_empty() {
            len += 1;
        }
        if self.is_sales_outlet.is_some() {
            len += 1;
        }
        if self.is_pickup_point.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.contact_email.is_some() {
            len += 1;
        }
        if self.contact_phone.is_some() {
            len += 1;
        }
        if self.opening_hours.is_some() {
            len += 1;
        }
        if self.can_be_sales_channel.is_some() {
            len += 1;
        }
        if self.external_sales_channel_id.is_some() {
            len += 1;
        }
        if self.franchisee_id.is_some() {
            len += 1;
        }
        if self.latitude.is_some() {
            len += 1;
        }
        if self.longitude.is_some() {
            len += 1;
        }
        if self.timezone.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.CreateWarehouseRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.branch_type.is_empty() {
            struct_ser.serialize_field("branchType", &self.branch_type)?;
        }
        if let Some(v) = self.is_sales_outlet.as_ref() {
            struct_ser.serialize_field("isSalesOutlet", v)?;
        }
        if let Some(v) = self.is_pickup_point.as_ref() {
            struct_ser.serialize_field("isPickupPoint", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.contact_email.as_ref() {
            struct_ser.serialize_field("contactEmail", v)?;
        }
        if let Some(v) = self.contact_phone.as_ref() {
            struct_ser.serialize_field("contactPhone", v)?;
        }
        if let Some(v) = self.opening_hours.as_ref() {
            struct_ser.serialize_field("openingHours", v)?;
        }
        if let Some(v) = self.can_be_sales_channel.as_ref() {
            struct_ser.serialize_field("canBeSalesChannel", v)?;
        }
        if let Some(v) = self.external_sales_channel_id.as_ref() {
            struct_ser.serialize_field("externalSalesChannelId", v)?;
        }
        if let Some(v) = self.franchisee_id.as_ref() {
            struct_ser.serialize_field("franchiseeId", v)?;
        }
        if let Some(v) = self.latitude.as_ref() {
            struct_ser.serialize_field("latitude", v)?;
        }
        if let Some(v) = self.longitude.as_ref() {
            struct_ser.serialize_field("longitude", v)?;
        }
        if let Some(v) = self.timezone.as_ref() {
            struct_ser.serialize_field("timezone", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWarehouseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "branch_type",
            "branchType",
            "is_sales_outlet",
            "isSalesOutlet",
            "is_pickup_point",
            "isPickupPoint",
            "is_active",
            "isActive",
            "address",
            "contact_email",
            "contactEmail",
            "contact_phone",
            "contactPhone",
            "opening_hours",
            "openingHours",
            "can_be_sales_channel",
            "canBeSalesChannel",
            "external_sales_channel_id",
            "externalSalesChannelId",
            "franchisee_id",
            "franchiseeId",
            "latitude",
            "longitude",
            "timezone",
            "configuration",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            BranchType,
            IsSalesOutlet,
            IsPickupPoint,
            IsActive,
            Address,
            ContactEmail,
            ContactPhone,
            OpeningHours,
            CanBeSalesChannel,
            ExternalSalesChannelId,
            FranchiseeId,
            Latitude,
            Longitude,
            Timezone,
            Configuration,
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
                            "branchType" | "branch_type" => Ok(GeneratedField::BranchType),
                            "isSalesOutlet" | "is_sales_outlet" => Ok(GeneratedField::IsSalesOutlet),
                            "isPickupPoint" | "is_pickup_point" => Ok(GeneratedField::IsPickupPoint),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "address" => Ok(GeneratedField::Address),
                            "contactEmail" | "contact_email" => Ok(GeneratedField::ContactEmail),
                            "contactPhone" | "contact_phone" => Ok(GeneratedField::ContactPhone),
                            "openingHours" | "opening_hours" => Ok(GeneratedField::OpeningHours),
                            "canBeSalesChannel" | "can_be_sales_channel" => Ok(GeneratedField::CanBeSalesChannel),
                            "externalSalesChannelId" | "external_sales_channel_id" => Ok(GeneratedField::ExternalSalesChannelId),
                            "franchiseeId" | "franchisee_id" => Ok(GeneratedField::FranchiseeId),
                            "latitude" => Ok(GeneratedField::Latitude),
                            "longitude" => Ok(GeneratedField::Longitude),
                            "timezone" => Ok(GeneratedField::Timezone),
                            "configuration" => Ok(GeneratedField::Configuration),
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
            type Value = CreateWarehouseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.CreateWarehouseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWarehouseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut branch_type__ = None;
                let mut is_sales_outlet__ = None;
                let mut is_pickup_point__ = None;
                let mut is_active__ = None;
                let mut address__ = None;
                let mut contact_email__ = None;
                let mut contact_phone__ = None;
                let mut opening_hours__ = None;
                let mut can_be_sales_channel__ = None;
                let mut external_sales_channel_id__ = None;
                let mut franchisee_id__ = None;
                let mut latitude__ = None;
                let mut longitude__ = None;
                let mut timezone__ = None;
                let mut configuration__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BranchType => {
                            if branch_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("branchType"));
                            }
                            branch_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSalesOutlet => {
                            if is_sales_outlet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSalesOutlet"));
                            }
                            is_sales_outlet__ = map_.next_value()?;
                        }
                        GeneratedField::IsPickupPoint => {
                            if is_pickup_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPickupPoint"));
                            }
                            is_pickup_point__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::ContactEmail => {
                            if contact_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactEmail"));
                            }
                            contact_email__ = map_.next_value()?;
                        }
                        GeneratedField::ContactPhone => {
                            if contact_phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactPhone"));
                            }
                            contact_phone__ = map_.next_value()?;
                        }
                        GeneratedField::OpeningHours => {
                            if opening_hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openingHours"));
                            }
                            opening_hours__ = map_.next_value()?;
                        }
                        GeneratedField::CanBeSalesChannel => {
                            if can_be_sales_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canBeSalesChannel"));
                            }
                            can_be_sales_channel__ = map_.next_value()?;
                        }
                        GeneratedField::ExternalSalesChannelId => {
                            if external_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSalesChannelId"));
                            }
                            external_sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FranchiseeId => {
                            if franchisee_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("franchiseeId"));
                            }
                            franchisee_id__ = map_.next_value()?;
                        }
                        GeneratedField::Latitude => {
                            if latitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitude"));
                            }
                            latitude__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Longitude => {
                            if longitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitude"));
                            }
                            longitude__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = map_.next_value()?;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateWarehouseRequest {
                    code: code__.unwrap_or_default(),
                    branch_type: branch_type__.unwrap_or_default(),
                    is_sales_outlet: is_sales_outlet__,
                    is_pickup_point: is_pickup_point__,
                    is_active: is_active__,
                    address: address__,
                    contact_email: contact_email__,
                    contact_phone: contact_phone__,
                    opening_hours: opening_hours__,
                    can_be_sales_channel: can_be_sales_channel__,
                    external_sales_channel_id: external_sales_channel_id__,
                    franchisee_id: franchisee_id__,
                    latitude: latitude__,
                    longitude: longitude__,
                    timezone: timezone__,
                    configuration: configuration__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.CreateWarehouseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStockLevelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if !self.product_id.is_empty() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.locale_input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.GetStockLevelRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStockLevelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "unit_id",
            "unitId",
            "locale_input",
            "localeInput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            ProductId,
            VariantId,
            UnitId,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
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
            type Value = GetStockLevelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.GetStockLevelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStockLevelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut locale_input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
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
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetStockLevelRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    locale_input: locale_input__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.GetStockLevelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStockLevelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stock_level.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.GetStockLevelResponse", len)?;
        if let Some(v) = self.stock_level.as_ref() {
            struct_ser.serialize_field("stockLevel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStockLevelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stock_level",
            "stockLevel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StockLevel,
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
                            "stockLevel" | "stock_level" => Ok(GeneratedField::StockLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStockLevelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.GetStockLevelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStockLevelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stock_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StockLevel => {
                            if stock_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stockLevel"));
                            }
                            stock_level__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetStockLevelResponse {
                    stock_level: stock_level__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.GetStockLevelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWarehouseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.warehouse.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.GetWarehouseResponse", len)?;
        if let Some(v) = self.warehouse.as_ref() {
            struct_ser.serialize_field("warehouse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWarehouseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Warehouse,
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
                            "warehouse" => Ok(GeneratedField::Warehouse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWarehouseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.GetWarehouseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWarehouseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Warehouse => {
                            if warehouse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouse"));
                            }
                            warehouse__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWarehouseResponse {
                    warehouse: warehouse__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.GetWarehouseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStockLevelsRequest {
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
        if self.filter_by_warehouse_id.is_some() {
            len += 1;
        }
        if self.filter_by_product_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.ListStockLevelsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_product_id.as_ref() {
            struct_ser.serialize_field("filterByProductId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStockLevelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_warehouse_id",
            "filterByWarehouseId",
            "filter_by_product_id",
            "filterByProductId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
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
                            "filterByWarehouseId" | "filter_by_warehouse_id" => Ok(GeneratedField::FilterByWarehouseId),
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
            type Value = ListStockLevelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.ListStockLevelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStockLevelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_product_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByWarehouseId => {
                            if filter_by_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByWarehouseId"));
                            }
                            filter_by_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByProductId => {
                            if filter_by_product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByProductId"));
                            }
                            filter_by_product_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListStockLevelsRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_product_id: filter_by_product_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.ListStockLevelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStockLevelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stock_levels.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.ListStockLevelsResponse", len)?;
        if !self.stock_levels.is_empty() {
            struct_ser.serialize_field("stockLevels", &self.stock_levels)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStockLevelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stock_levels",
            "stockLevels",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StockLevels,
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
                            "stockLevels" | "stock_levels" => Ok(GeneratedField::StockLevels),
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
            type Value = ListStockLevelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.ListStockLevelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStockLevelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stock_levels__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StockLevels => {
                            if stock_levels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stockLevels"));
                            }
                            stock_levels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListStockLevelsResponse {
                    stock_levels: stock_levels__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.ListStockLevelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStockMovementsRequest {
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
        if self.filter_by_warehouse_id.is_some() {
            len += 1;
        }
        if self.filter_by_product_id.is_some() {
            len += 1;
        }
        if self.filter_by_movement_type.is_some() {
            len += 1;
        }
        if self.filter_by_occurred_date_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.ListStockMovementsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_product_id.as_ref() {
            struct_ser.serialize_field("filterByProductId", v)?;
        }
        if let Some(v) = self.filter_by_movement_type.as_ref() {
            struct_ser.serialize_field("filterByMovementType", v)?;
        }
        if let Some(v) = self.filter_by_occurred_date_range.as_ref() {
            struct_ser.serialize_field("filterByOccurredDateRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStockMovementsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_warehouse_id",
            "filterByWarehouseId",
            "filter_by_product_id",
            "filterByProductId",
            "filter_by_movement_type",
            "filterByMovementType",
            "filter_by_occurred_date_range",
            "filterByOccurredDateRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByProductId,
            FilterByMovementType,
            FilterByOccurredDateRange,
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
                            "filterByWarehouseId" | "filter_by_warehouse_id" => Ok(GeneratedField::FilterByWarehouseId),
                            "filterByProductId" | "filter_by_product_id" => Ok(GeneratedField::FilterByProductId),
                            "filterByMovementType" | "filter_by_movement_type" => Ok(GeneratedField::FilterByMovementType),
                            "filterByOccurredDateRange" | "filter_by_occurred_date_range" => Ok(GeneratedField::FilterByOccurredDateRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListStockMovementsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.ListStockMovementsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStockMovementsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_product_id__ = None;
                let mut filter_by_movement_type__ = None;
                let mut filter_by_occurred_date_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByWarehouseId => {
                            if filter_by_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByWarehouseId"));
                            }
                            filter_by_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByProductId => {
                            if filter_by_product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByProductId"));
                            }
                            filter_by_product_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByMovementType => {
                            if filter_by_movement_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByMovementType"));
                            }
                            filter_by_movement_type__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByOccurredDateRange => {
                            if filter_by_occurred_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByOccurredDateRange"));
                            }
                            filter_by_occurred_date_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListStockMovementsRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_product_id: filter_by_product_id__,
                    filter_by_movement_type: filter_by_movement_type__,
                    filter_by_occurred_date_range: filter_by_occurred_date_range__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.ListStockMovementsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStockMovementsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stock_movements.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.ListStockMovementsResponse", len)?;
        if !self.stock_movements.is_empty() {
            struct_ser.serialize_field("stockMovements", &self.stock_movements)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStockMovementsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stock_movements",
            "stockMovements",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StockMovements,
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
                            "stockMovements" | "stock_movements" => Ok(GeneratedField::StockMovements),
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
            type Value = ListStockMovementsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.ListStockMovementsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStockMovementsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stock_movements__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StockMovements => {
                            if stock_movements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stockMovements"));
                            }
                            stock_movements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListStockMovementsResponse {
                    stock_movements: stock_movements__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.ListStockMovementsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWarehousesRequest {
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
        if self.filter_by_branch_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.ListWarehousesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_branch_type.as_ref() {
            struct_ser.serialize_field("filterByBranchType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWarehousesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_branch_type",
            "filterByBranchType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByBranchType,
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
                            "filterByBranchType" | "filter_by_branch_type" => Ok(GeneratedField::FilterByBranchType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWarehousesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.ListWarehousesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWarehousesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_branch_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByBranchType => {
                            if filter_by_branch_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByBranchType"));
                            }
                            filter_by_branch_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListWarehousesRequest {
                    base_request: base_request__,
                    filter_by_branch_type: filter_by_branch_type__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.ListWarehousesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWarehousesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.warehouses.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.ListWarehousesResponse", len)?;
        if !self.warehouses.is_empty() {
            struct_ser.serialize_field("warehouses", &self.warehouses)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWarehousesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouses",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Warehouses,
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
                            "warehouses" => Ok(GeneratedField::Warehouses),
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
            type Value = ListWarehousesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.ListWarehousesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWarehousesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouses__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Warehouses => {
                            if warehouses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouses"));
                            }
                            warehouses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListWarehousesResponse {
                    warehouses: warehouses__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.ListWarehousesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecordStockMovementRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if !self.product_id.is_empty() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if !self.quantity_changed.is_empty() {
            len += 1;
        }
        if !self.movement_type.is_empty() {
            len += 1;
        }
        if self.reference_document_id.is_some() {
            len += 1;
        }
        if self.reference_document_type.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if self.occurred_at.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.RecordStockMovementRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if !self.quantity_changed.is_empty() {
            struct_ser.serialize_field("quantityChanged", &self.quantity_changed)?;
        }
        if !self.movement_type.is_empty() {
            struct_ser.serialize_field("movementType", &self.movement_type)?;
        }
        if let Some(v) = self.reference_document_id.as_ref() {
            struct_ser.serialize_field("referenceDocumentId", v)?;
        }
        if let Some(v) = self.reference_document_type.as_ref() {
            struct_ser.serialize_field("referenceDocumentType", v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.occurred_at.as_ref() {
            struct_ser.serialize_field("occurredAt", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecordStockMovementRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "unit_id",
            "unitId",
            "quantity_changed",
            "quantityChanged",
            "movement_type",
            "movementType",
            "reference_document_id",
            "referenceDocumentId",
            "reference_document_type",
            "referenceDocumentType",
            "reason",
            "occurred_at",
            "occurredAt",
            "created_by_user_id",
            "createdByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            ProductId,
            VariantId,
            UnitId,
            QuantityChanged,
            MovementType,
            ReferenceDocumentId,
            ReferenceDocumentType,
            Reason,
            OccurredAt,
            CreatedByUserId,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "quantityChanged" | "quantity_changed" => Ok(GeneratedField::QuantityChanged),
                            "movementType" | "movement_type" => Ok(GeneratedField::MovementType),
                            "referenceDocumentId" | "reference_document_id" => Ok(GeneratedField::ReferenceDocumentId),
                            "referenceDocumentType" | "reference_document_type" => Ok(GeneratedField::ReferenceDocumentType),
                            "reason" => Ok(GeneratedField::Reason),
                            "occurredAt" | "occurred_at" => Ok(GeneratedField::OccurredAt),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecordStockMovementRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.RecordStockMovementRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecordStockMovementRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut quantity_changed__ = None;
                let mut movement_type__ = None;
                let mut reference_document_id__ = None;
                let mut reference_document_type__ = None;
                let mut reason__ = None;
                let mut occurred_at__ = None;
                let mut created_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
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
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuantityChanged => {
                            if quantity_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantityChanged"));
                            }
                            quantity_changed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MovementType => {
                            if movement_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("movementType"));
                            }
                            movement_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceDocumentId => {
                            if reference_document_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentId"));
                            }
                            reference_document_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceDocumentType => {
                            if reference_document_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentType"));
                            }
                            reference_document_type__ = map_.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::OccurredAt => {
                            if occurred_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurredAt"));
                            }
                            occurred_at__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RecordStockMovementRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    quantity_changed: quantity_changed__.unwrap_or_default(),
                    movement_type: movement_type__.unwrap_or_default(),
                    reference_document_id: reference_document_id__,
                    reference_document_type: reference_document_type__,
                    reason: reason__,
                    occurred_at: occurred_at__,
                    created_by_user_id: created_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.RecordStockMovementRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StockLevel {
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
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if !self.product_id.is_empty() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.last_counted_at.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        if self.product_summary.is_some() {
            len += 1;
        }
        if self.variant_summary.is_some() {
            len += 1;
        }
        if self.unit_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.StockLevel", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if let Some(v) = self.last_counted_at.as_ref() {
            struct_ser.serialize_field("lastCountedAt", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        if let Some(v) = self.product_summary.as_ref() {
            struct_ser.serialize_field("productSummary", v)?;
        }
        if let Some(v) = self.variant_summary.as_ref() {
            struct_ser.serialize_field("variantSummary", v)?;
        }
        if let Some(v) = self.unit_summary.as_ref() {
            struct_ser.serialize_field("unitSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StockLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "unit_id",
            "unitId",
            "quantity",
            "last_counted_at",
            "lastCountedAt",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "warehouse_summary",
            "warehouseSummary",
            "product_summary",
            "productSummary",
            "variant_summary",
            "variantSummary",
            "unit_summary",
            "unitSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            ProductId,
            VariantId,
            UnitId,
            Quantity,
            LastCountedAt,
            CreatedAt,
            UpdatedAt,
            WarehouseSummary,
            ProductSummary,
            VariantSummary,
            UnitSummary,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "lastCountedAt" | "last_counted_at" => Ok(GeneratedField::LastCountedAt),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            "productSummary" | "product_summary" => Ok(GeneratedField::ProductSummary),
                            "variantSummary" | "variant_summary" => Ok(GeneratedField::VariantSummary),
                            "unitSummary" | "unit_summary" => Ok(GeneratedField::UnitSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StockLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.StockLevel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StockLevel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut quantity__ = None;
                let mut last_counted_at__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut warehouse_summary__ = None;
                let mut product_summary__ = None;
                let mut variant_summary__ = None;
                let mut unit_summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
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
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastCountedAt => {
                            if last_counted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastCountedAt"));
                            }
                            last_counted_at__ = map_.next_value()?;
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
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                        GeneratedField::ProductSummary => {
                            if product_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productSummary"));
                            }
                            product_summary__ = map_.next_value()?;
                        }
                        GeneratedField::VariantSummary => {
                            if variant_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantSummary"));
                            }
                            variant_summary__ = map_.next_value()?;
                        }
                        GeneratedField::UnitSummary => {
                            if unit_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitSummary"));
                            }
                            unit_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StockLevel {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    last_counted_at: last_counted_at__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    warehouse_summary: warehouse_summary__,
                    product_summary: product_summary__,
                    variant_summary: variant_summary__,
                    unit_summary: unit_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.StockLevel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StockMovement {
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
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if !self.product_id.is_empty() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if !self.quantity_changed.is_empty() {
            len += 1;
        }
        if !self.movement_type.is_empty() {
            len += 1;
        }
        if self.reference_document_id.is_some() {
            len += 1;
        }
        if self.reference_document_type.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if self.occurred_at.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        if self.product_summary.is_some() {
            len += 1;
        }
        if self.variant_summary.is_some() {
            len += 1;
        }
        if self.unit_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.StockMovement", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if !self.quantity_changed.is_empty() {
            struct_ser.serialize_field("quantityChanged", &self.quantity_changed)?;
        }
        if !self.movement_type.is_empty() {
            struct_ser.serialize_field("movementType", &self.movement_type)?;
        }
        if let Some(v) = self.reference_document_id.as_ref() {
            struct_ser.serialize_field("referenceDocumentId", v)?;
        }
        if let Some(v) = self.reference_document_type.as_ref() {
            struct_ser.serialize_field("referenceDocumentType", v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.occurred_at.as_ref() {
            struct_ser.serialize_field("occurredAt", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        if let Some(v) = self.product_summary.as_ref() {
            struct_ser.serialize_field("productSummary", v)?;
        }
        if let Some(v) = self.variant_summary.as_ref() {
            struct_ser.serialize_field("variantSummary", v)?;
        }
        if let Some(v) = self.unit_summary.as_ref() {
            struct_ser.serialize_field("unitSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StockMovement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "unit_id",
            "unitId",
            "quantity_changed",
            "quantityChanged",
            "movement_type",
            "movementType",
            "reference_document_id",
            "referenceDocumentId",
            "reference_document_type",
            "referenceDocumentType",
            "reason",
            "occurred_at",
            "occurredAt",
            "created_by_user_id",
            "createdByUserId",
            "created_at",
            "createdAt",
            "warehouse_summary",
            "warehouseSummary",
            "product_summary",
            "productSummary",
            "variant_summary",
            "variantSummary",
            "unit_summary",
            "unitSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            ProductId,
            VariantId,
            UnitId,
            QuantityChanged,
            MovementType,
            ReferenceDocumentId,
            ReferenceDocumentType,
            Reason,
            OccurredAt,
            CreatedByUserId,
            CreatedAt,
            WarehouseSummary,
            ProductSummary,
            VariantSummary,
            UnitSummary,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "quantityChanged" | "quantity_changed" => Ok(GeneratedField::QuantityChanged),
                            "movementType" | "movement_type" => Ok(GeneratedField::MovementType),
                            "referenceDocumentId" | "reference_document_id" => Ok(GeneratedField::ReferenceDocumentId),
                            "referenceDocumentType" | "reference_document_type" => Ok(GeneratedField::ReferenceDocumentType),
                            "reason" => Ok(GeneratedField::Reason),
                            "occurredAt" | "occurred_at" => Ok(GeneratedField::OccurredAt),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            "productSummary" | "product_summary" => Ok(GeneratedField::ProductSummary),
                            "variantSummary" | "variant_summary" => Ok(GeneratedField::VariantSummary),
                            "unitSummary" | "unit_summary" => Ok(GeneratedField::UnitSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StockMovement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.StockMovement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StockMovement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut quantity_changed__ = None;
                let mut movement_type__ = None;
                let mut reference_document_id__ = None;
                let mut reference_document_type__ = None;
                let mut reason__ = None;
                let mut occurred_at__ = None;
                let mut created_by_user_id__ = None;
                let mut created_at__ = None;
                let mut warehouse_summary__ = None;
                let mut product_summary__ = None;
                let mut variant_summary__ = None;
                let mut unit_summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
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
                            variant_id__ = map_.next_value()?;
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuantityChanged => {
                            if quantity_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantityChanged"));
                            }
                            quantity_changed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MovementType => {
                            if movement_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("movementType"));
                            }
                            movement_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceDocumentId => {
                            if reference_document_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentId"));
                            }
                            reference_document_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceDocumentType => {
                            if reference_document_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentType"));
                            }
                            reference_document_type__ = map_.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::OccurredAt => {
                            if occurred_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurredAt"));
                            }
                            occurred_at__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                        GeneratedField::ProductSummary => {
                            if product_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productSummary"));
                            }
                            product_summary__ = map_.next_value()?;
                        }
                        GeneratedField::VariantSummary => {
                            if variant_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantSummary"));
                            }
                            variant_summary__ = map_.next_value()?;
                        }
                        GeneratedField::UnitSummary => {
                            if unit_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitSummary"));
                            }
                            unit_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StockMovement {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    quantity_changed: quantity_changed__.unwrap_or_default(),
                    movement_type: movement_type__.unwrap_or_default(),
                    reference_document_id: reference_document_id__,
                    reference_document_type: reference_document_type__,
                    reason: reason__,
                    occurred_at: occurred_at__,
                    created_by_user_id: created_by_user_id__,
                    created_at: created_at__,
                    warehouse_summary: warehouse_summary__,
                    product_summary: product_summary__,
                    variant_summary: variant_summary__,
                    unit_summary: unit_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.StockMovement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWarehouseRequest {
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
        if self.branch_type.is_some() {
            len += 1;
        }
        if self.is_sales_outlet.is_some() {
            len += 1;
        }
        if self.is_pickup_point.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.contact_email.is_some() {
            len += 1;
        }
        if self.contact_phone.is_some() {
            len += 1;
        }
        if self.opening_hours.is_some() {
            len += 1;
        }
        if self.can_be_sales_channel.is_some() {
            len += 1;
        }
        if self.external_sales_channel_id.is_some() {
            len += 1;
        }
        if self.franchisee_id.is_some() {
            len += 1;
        }
        if self.latitude.is_some() {
            len += 1;
        }
        if self.longitude.is_some() {
            len += 1;
        }
        if self.timezone.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.UpdateWarehouseRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.branch_type.as_ref() {
            struct_ser.serialize_field("branchType", v)?;
        }
        if let Some(v) = self.is_sales_outlet.as_ref() {
            struct_ser.serialize_field("isSalesOutlet", v)?;
        }
        if let Some(v) = self.is_pickup_point.as_ref() {
            struct_ser.serialize_field("isPickupPoint", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.contact_email.as_ref() {
            struct_ser.serialize_field("contactEmail", v)?;
        }
        if let Some(v) = self.contact_phone.as_ref() {
            struct_ser.serialize_field("contactPhone", v)?;
        }
        if let Some(v) = self.opening_hours.as_ref() {
            struct_ser.serialize_field("openingHours", v)?;
        }
        if let Some(v) = self.can_be_sales_channel.as_ref() {
            struct_ser.serialize_field("canBeSalesChannel", v)?;
        }
        if let Some(v) = self.external_sales_channel_id.as_ref() {
            struct_ser.serialize_field("externalSalesChannelId", v)?;
        }
        if let Some(v) = self.franchisee_id.as_ref() {
            struct_ser.serialize_field("franchiseeId", v)?;
        }
        if let Some(v) = self.latitude.as_ref() {
            struct_ser.serialize_field("latitude", v)?;
        }
        if let Some(v) = self.longitude.as_ref() {
            struct_ser.serialize_field("longitude", v)?;
        }
        if let Some(v) = self.timezone.as_ref() {
            struct_ser.serialize_field("timezone", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWarehouseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "branch_type",
            "branchType",
            "is_sales_outlet",
            "isSalesOutlet",
            "is_pickup_point",
            "isPickupPoint",
            "is_active",
            "isActive",
            "address",
            "contact_email",
            "contactEmail",
            "contact_phone",
            "contactPhone",
            "opening_hours",
            "openingHours",
            "can_be_sales_channel",
            "canBeSalesChannel",
            "external_sales_channel_id",
            "externalSalesChannelId",
            "franchisee_id",
            "franchiseeId",
            "latitude",
            "longitude",
            "timezone",
            "configuration",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            BranchType,
            IsSalesOutlet,
            IsPickupPoint,
            IsActive,
            Address,
            ContactEmail,
            ContactPhone,
            OpeningHours,
            CanBeSalesChannel,
            ExternalSalesChannelId,
            FranchiseeId,
            Latitude,
            Longitude,
            Timezone,
            Configuration,
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
                            "branchType" | "branch_type" => Ok(GeneratedField::BranchType),
                            "isSalesOutlet" | "is_sales_outlet" => Ok(GeneratedField::IsSalesOutlet),
                            "isPickupPoint" | "is_pickup_point" => Ok(GeneratedField::IsPickupPoint),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "address" => Ok(GeneratedField::Address),
                            "contactEmail" | "contact_email" => Ok(GeneratedField::ContactEmail),
                            "contactPhone" | "contact_phone" => Ok(GeneratedField::ContactPhone),
                            "openingHours" | "opening_hours" => Ok(GeneratedField::OpeningHours),
                            "canBeSalesChannel" | "can_be_sales_channel" => Ok(GeneratedField::CanBeSalesChannel),
                            "externalSalesChannelId" | "external_sales_channel_id" => Ok(GeneratedField::ExternalSalesChannelId),
                            "franchiseeId" | "franchisee_id" => Ok(GeneratedField::FranchiseeId),
                            "latitude" => Ok(GeneratedField::Latitude),
                            "longitude" => Ok(GeneratedField::Longitude),
                            "timezone" => Ok(GeneratedField::Timezone),
                            "configuration" => Ok(GeneratedField::Configuration),
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
            type Value = UpdateWarehouseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.UpdateWarehouseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWarehouseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut branch_type__ = None;
                let mut is_sales_outlet__ = None;
                let mut is_pickup_point__ = None;
                let mut is_active__ = None;
                let mut address__ = None;
                let mut contact_email__ = None;
                let mut contact_phone__ = None;
                let mut opening_hours__ = None;
                let mut can_be_sales_channel__ = None;
                let mut external_sales_channel_id__ = None;
                let mut franchisee_id__ = None;
                let mut latitude__ = None;
                let mut longitude__ = None;
                let mut timezone__ = None;
                let mut configuration__ = None;
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
                        GeneratedField::BranchType => {
                            if branch_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("branchType"));
                            }
                            branch_type__ = map_.next_value()?;
                        }
                        GeneratedField::IsSalesOutlet => {
                            if is_sales_outlet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSalesOutlet"));
                            }
                            is_sales_outlet__ = map_.next_value()?;
                        }
                        GeneratedField::IsPickupPoint => {
                            if is_pickup_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPickupPoint"));
                            }
                            is_pickup_point__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::ContactEmail => {
                            if contact_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactEmail"));
                            }
                            contact_email__ = map_.next_value()?;
                        }
                        GeneratedField::ContactPhone => {
                            if contact_phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactPhone"));
                            }
                            contact_phone__ = map_.next_value()?;
                        }
                        GeneratedField::OpeningHours => {
                            if opening_hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openingHours"));
                            }
                            opening_hours__ = map_.next_value()?;
                        }
                        GeneratedField::CanBeSalesChannel => {
                            if can_be_sales_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canBeSalesChannel"));
                            }
                            can_be_sales_channel__ = map_.next_value()?;
                        }
                        GeneratedField::ExternalSalesChannelId => {
                            if external_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSalesChannelId"));
                            }
                            external_sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FranchiseeId => {
                            if franchisee_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("franchiseeId"));
                            }
                            franchisee_id__ = map_.next_value()?;
                        }
                        GeneratedField::Latitude => {
                            if latitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitude"));
                            }
                            latitude__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Longitude => {
                            if longitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitude"));
                            }
                            longitude__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = map_.next_value()?;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::TranslationsToUpdate => {
                            if translations_to_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translationsToUpdate"));
                            }
                            translations_to_update__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateWarehouseRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    branch_type: branch_type__,
                    is_sales_outlet: is_sales_outlet__,
                    is_pickup_point: is_pickup_point__,
                    is_active: is_active__,
                    address: address__,
                    contact_email: contact_email__,
                    contact_phone: contact_phone__,
                    opening_hours: opening_hours__,
                    can_be_sales_channel: can_be_sales_channel__,
                    external_sales_channel_id: external_sales_channel_id__,
                    franchisee_id: franchisee_id__,
                    latitude: latitude__,
                    longitude: longitude__,
                    timezone: timezone__,
                    configuration: configuration__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.UpdateWarehouseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Warehouse {
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
        if !self.branch_type.is_empty() {
            len += 1;
        }
        if self.is_sales_outlet {
            len += 1;
        }
        if self.is_pickup_point {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.contact_email.is_some() {
            len += 1;
        }
        if self.contact_phone.is_some() {
            len += 1;
        }
        if self.opening_hours.is_some() {
            len += 1;
        }
        if self.can_be_sales_channel {
            len += 1;
        }
        if self.external_sales_channel_id.is_some() {
            len += 1;
        }
        if self.franchisee_id.is_some() {
            len += 1;
        }
        if self.latitude.is_some() {
            len += 1;
        }
        if self.longitude.is_some() {
            len += 1;
        }
        if self.timezone.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
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
        if !self.public_display_name.is_empty() {
            len += 1;
        }
        if !self.delivery_instructions.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.inventory.v1.Warehouse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.branch_type.is_empty() {
            struct_ser.serialize_field("branchType", &self.branch_type)?;
        }
        if self.is_sales_outlet {
            struct_ser.serialize_field("isSalesOutlet", &self.is_sales_outlet)?;
        }
        if self.is_pickup_point {
            struct_ser.serialize_field("isPickupPoint", &self.is_pickup_point)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.contact_email.as_ref() {
            struct_ser.serialize_field("contactEmail", v)?;
        }
        if let Some(v) = self.contact_phone.as_ref() {
            struct_ser.serialize_field("contactPhone", v)?;
        }
        if let Some(v) = self.opening_hours.as_ref() {
            struct_ser.serialize_field("openingHours", v)?;
        }
        if self.can_be_sales_channel {
            struct_ser.serialize_field("canBeSalesChannel", &self.can_be_sales_channel)?;
        }
        if let Some(v) = self.external_sales_channel_id.as_ref() {
            struct_ser.serialize_field("externalSalesChannelId", v)?;
        }
        if let Some(v) = self.franchisee_id.as_ref() {
            struct_ser.serialize_field("franchiseeId", v)?;
        }
        if let Some(v) = self.latitude.as_ref() {
            struct_ser.serialize_field("latitude", v)?;
        }
        if let Some(v) = self.longitude.as_ref() {
            struct_ser.serialize_field("longitude", v)?;
        }
        if let Some(v) = self.timezone.as_ref() {
            struct_ser.serialize_field("timezone", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
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
        if !self.public_display_name.is_empty() {
            struct_ser.serialize_field("publicDisplayName", &self.public_display_name)?;
        }
        if !self.delivery_instructions.is_empty() {
            struct_ser.serialize_field("deliveryInstructions", &self.delivery_instructions)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Warehouse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "branch_type",
            "branchType",
            "is_sales_outlet",
            "isSalesOutlet",
            "is_pickup_point",
            "isPickupPoint",
            "is_active",
            "isActive",
            "address",
            "contact_email",
            "contactEmail",
            "contact_phone",
            "contactPhone",
            "opening_hours",
            "openingHours",
            "can_be_sales_channel",
            "canBeSalesChannel",
            "external_sales_channel_id",
            "externalSalesChannelId",
            "franchisee_id",
            "franchiseeId",
            "latitude",
            "longitude",
            "timezone",
            "configuration",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "public_display_name",
            "publicDisplayName",
            "delivery_instructions",
            "deliveryInstructions",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            BranchType,
            IsSalesOutlet,
            IsPickupPoint,
            IsActive,
            Address,
            ContactEmail,
            ContactPhone,
            OpeningHours,
            CanBeSalesChannel,
            ExternalSalesChannelId,
            FranchiseeId,
            Latitude,
            Longitude,
            Timezone,
            Configuration,
            CreatedAt,
            UpdatedAt,
            Name,
            PublicDisplayName,
            DeliveryInstructions,
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
                            "branchType" | "branch_type" => Ok(GeneratedField::BranchType),
                            "isSalesOutlet" | "is_sales_outlet" => Ok(GeneratedField::IsSalesOutlet),
                            "isPickupPoint" | "is_pickup_point" => Ok(GeneratedField::IsPickupPoint),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "address" => Ok(GeneratedField::Address),
                            "contactEmail" | "contact_email" => Ok(GeneratedField::ContactEmail),
                            "contactPhone" | "contact_phone" => Ok(GeneratedField::ContactPhone),
                            "openingHours" | "opening_hours" => Ok(GeneratedField::OpeningHours),
                            "canBeSalesChannel" | "can_be_sales_channel" => Ok(GeneratedField::CanBeSalesChannel),
                            "externalSalesChannelId" | "external_sales_channel_id" => Ok(GeneratedField::ExternalSalesChannelId),
                            "franchiseeId" | "franchisee_id" => Ok(GeneratedField::FranchiseeId),
                            "latitude" => Ok(GeneratedField::Latitude),
                            "longitude" => Ok(GeneratedField::Longitude),
                            "timezone" => Ok(GeneratedField::Timezone),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "publicDisplayName" | "public_display_name" => Ok(GeneratedField::PublicDisplayName),
                            "deliveryInstructions" | "delivery_instructions" => Ok(GeneratedField::DeliveryInstructions),
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
            type Value = Warehouse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.inventory.v1.Warehouse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Warehouse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut branch_type__ = None;
                let mut is_sales_outlet__ = None;
                let mut is_pickup_point__ = None;
                let mut is_active__ = None;
                let mut address__ = None;
                let mut contact_email__ = None;
                let mut contact_phone__ = None;
                let mut opening_hours__ = None;
                let mut can_be_sales_channel__ = None;
                let mut external_sales_channel_id__ = None;
                let mut franchisee_id__ = None;
                let mut latitude__ = None;
                let mut longitude__ = None;
                let mut timezone__ = None;
                let mut configuration__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut public_display_name__ = None;
                let mut delivery_instructions__ = None;
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
                        GeneratedField::BranchType => {
                            if branch_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("branchType"));
                            }
                            branch_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSalesOutlet => {
                            if is_sales_outlet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSalesOutlet"));
                            }
                            is_sales_outlet__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPickupPoint => {
                            if is_pickup_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPickupPoint"));
                            }
                            is_pickup_point__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::ContactEmail => {
                            if contact_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactEmail"));
                            }
                            contact_email__ = map_.next_value()?;
                        }
                        GeneratedField::ContactPhone => {
                            if contact_phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactPhone"));
                            }
                            contact_phone__ = map_.next_value()?;
                        }
                        GeneratedField::OpeningHours => {
                            if opening_hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openingHours"));
                            }
                            opening_hours__ = map_.next_value()?;
                        }
                        GeneratedField::CanBeSalesChannel => {
                            if can_be_sales_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canBeSalesChannel"));
                            }
                            can_be_sales_channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExternalSalesChannelId => {
                            if external_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSalesChannelId"));
                            }
                            external_sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FranchiseeId => {
                            if franchisee_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("franchiseeId"));
                            }
                            franchisee_id__ = map_.next_value()?;
                        }
                        GeneratedField::Latitude => {
                            if latitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitude"));
                            }
                            latitude__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Longitude => {
                            if longitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitude"));
                            }
                            longitude__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = map_.next_value()?;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
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
                        GeneratedField::PublicDisplayName => {
                            if public_display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicDisplayName"));
                            }
                            public_display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeliveryInstructions => {
                            if delivery_instructions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliveryInstructions"));
                            }
                            delivery_instructions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Warehouse {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    branch_type: branch_type__.unwrap_or_default(),
                    is_sales_outlet: is_sales_outlet__.unwrap_or_default(),
                    is_pickup_point: is_pickup_point__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    address: address__,
                    contact_email: contact_email__,
                    contact_phone: contact_phone__,
                    opening_hours: opening_hours__,
                    can_be_sales_channel: can_be_sales_channel__.unwrap_or_default(),
                    external_sales_channel_id: external_sales_channel_id__,
                    franchisee_id: franchisee_id__,
                    latitude: latitude__,
                    longitude: longitude__,
                    timezone: timezone__,
                    configuration: configuration__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    public_display_name: public_display_name__.unwrap_or_default(),
                    delivery_instructions: delivery_instructions__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.inventory.v1.Warehouse", FIELDS, GeneratedVisitor)
    }
}
