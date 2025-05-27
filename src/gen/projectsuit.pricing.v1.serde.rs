// @generated
impl serde::Serialize for AddItemToPriceListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_list_id.is_empty() {
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
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.AddItemToPriceListRequest", len)?;
        if !self.price_list_id.is_empty() {
            struct_ser.serialize_field("priceListId", &self.price_list_id)?;
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
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddItemToPriceListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_list_id",
            "priceListId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "unit_id",
            "unitId",
            "price",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceListId,
            ProductId,
            VariantId,
            UnitId,
            Price,
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
                            "priceListId" | "price_list_id" => Ok(GeneratedField::PriceListId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddItemToPriceListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.AddItemToPriceListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddItemToPriceListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_list_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceListId => {
                            if price_list_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceListId"));
                            }
                            price_list_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddItemToPriceListRequest {
                    price_list_id: price_list_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.AddItemToPriceListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPriceListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_list_id.is_empty() {
            len += 1;
        }
        if self.warehouse_id.is_some() {
            len += 1;
        }
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.AssignPriceListRequest", len)?;
        if !self.price_list_id.is_empty() {
            struct_ser.serialize_field("priceListId", &self.price_list_id)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPriceListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_list_id",
            "priceListId",
            "warehouse_id",
            "warehouseId",
            "sales_channel_id",
            "salesChannelId",
            "priority",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceListId,
            WarehouseId,
            SalesChannelId,
            Priority,
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
                            "priceListId" | "price_list_id" => Ok(GeneratedField::PriceListId),
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "priority" => Ok(GeneratedField::Priority),
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
            type Value = AssignPriceListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.AssignPriceListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPriceListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_list_id__ = None;
                let mut warehouse_id__ = None;
                let mut sales_channel_id__ = None;
                let mut priority__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceListId => {
                            if price_list_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceListId"));
                            }
                            price_list_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
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
                Ok(AssignPriceListRequest {
                    price_list_id: price_list_id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__,
                    sales_channel_id: sales_channel_id__,
                    priority: priority__,
                    start_date: start_date__,
                    end_date: end_date__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.AssignPriceListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Coupon {
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
        if self.promotion_id.is_some() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if self.max_usage_total.is_some() {
            len += 1;
        }
        if self.max_usage_per_customer.is_some() {
            len += 1;
        }
        if self.current_usage_total != 0 {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.Coupon", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.promotion_id.as_ref() {
            struct_ser.serialize_field("promotionId", v)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if let Some(v) = self.max_usage_total.as_ref() {
            struct_ser.serialize_field("maxUsageTotal", v)?;
        }
        if let Some(v) = self.max_usage_per_customer.as_ref() {
            struct_ser.serialize_field("maxUsagePerCustomer", v)?;
        }
        if self.current_usage_total != 0 {
            struct_ser.serialize_field("currentUsageTotal", &self.current_usage_total)?;
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
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Coupon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "promotion_id",
            "promotionId",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "max_usage_total",
            "maxUsageTotal",
            "max_usage_per_customer",
            "maxUsagePerCustomer",
            "current_usage_total",
            "currentUsageTotal",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "description",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            PromotionId,
            ValidFrom,
            ValidTo,
            MaxUsageTotal,
            MaxUsagePerCustomer,
            CurrentUsageTotal,
            IsActive,
            CreatedAt,
            UpdatedAt,
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
                            "promotionId" | "promotion_id" => Ok(GeneratedField::PromotionId),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "maxUsageTotal" | "max_usage_total" => Ok(GeneratedField::MaxUsageTotal),
                            "maxUsagePerCustomer" | "max_usage_per_customer" => Ok(GeneratedField::MaxUsagePerCustomer),
                            "currentUsageTotal" | "current_usage_total" => Ok(GeneratedField::CurrentUsageTotal),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
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
            type Value = Coupon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.Coupon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Coupon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut promotion_id__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut max_usage_total__ = None;
                let mut max_usage_per_customer__ = None;
                let mut current_usage_total__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
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
                        GeneratedField::PromotionId => {
                            if promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionId"));
                            }
                            promotion_id__ = map_.next_value()?;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::MaxUsageTotal => {
                            if max_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsageTotal"));
                            }
                            max_usage_total__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxUsagePerCustomer => {
                            if max_usage_per_customer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsagePerCustomer"));
                            }
                            max_usage_per_customer__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CurrentUsageTotal => {
                            if current_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentUsageTotal"));
                            }
                            current_usage_total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
                Ok(Coupon {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    promotion_id: promotion_id__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    max_usage_total: max_usage_total__,
                    max_usage_per_customer: max_usage_per_customer__,
                    current_usage_total: current_usage_total__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.Coupon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCouponRequest {
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
        if self.promotion_id.is_some() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if self.max_usage_total.is_some() {
            len += 1;
        }
        if self.max_usage_per_customer.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreateCouponRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.promotion_id.as_ref() {
            struct_ser.serialize_field("promotionId", v)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if let Some(v) = self.max_usage_total.as_ref() {
            struct_ser.serialize_field("maxUsageTotal", v)?;
        }
        if let Some(v) = self.max_usage_per_customer.as_ref() {
            struct_ser.serialize_field("maxUsagePerCustomer", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateCouponRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "promotion_id",
            "promotionId",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "max_usage_total",
            "maxUsageTotal",
            "max_usage_per_customer",
            "maxUsagePerCustomer",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            PromotionId,
            ValidFrom,
            ValidTo,
            MaxUsageTotal,
            MaxUsagePerCustomer,
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
                            "promotionId" | "promotion_id" => Ok(GeneratedField::PromotionId),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "maxUsageTotal" | "max_usage_total" => Ok(GeneratedField::MaxUsageTotal),
                            "maxUsagePerCustomer" | "max_usage_per_customer" => Ok(GeneratedField::MaxUsagePerCustomer),
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
            type Value = CreateCouponRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreateCouponRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCouponRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut promotion_id__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut max_usage_total__ = None;
                let mut max_usage_per_customer__ = None;
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
                        GeneratedField::PromotionId => {
                            if promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionId"));
                            }
                            promotion_id__ = map_.next_value()?;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::MaxUsageTotal => {
                            if max_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsageTotal"));
                            }
                            max_usage_total__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxUsagePerCustomer => {
                            if max_usage_per_customer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsagePerCustomer"));
                            }
                            max_usage_per_customer__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
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
                Ok(CreateCouponRequest {
                    code: code__.unwrap_or_default(),
                    promotion_id: promotion_id__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    max_usage_total: max_usage_total__,
                    max_usage_per_customer: max_usage_per_customer__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreateCouponRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePriceListRequest {
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
        if !self.currency_code.is_empty() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreatePriceListRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.currency_code.is_empty() {
            struct_ser.serialize_field("currencyCode", &self.currency_code)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePriceListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "currency_code",
            "currencyCode",
            "is_active",
            "isActive",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            CurrencyCode,
            IsActive,
            ValidFrom,
            ValidTo,
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
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
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
            type Value = CreatePriceListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreatePriceListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePriceListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut currency_code__ = None;
                let mut is_active__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreatePriceListRequest {
                    code: code__.unwrap_or_default(),
                    currency_code: currency_code__.unwrap_or_default(),
                    is_active: is_active__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreatePriceListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePromotionActionInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.action_type.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreatePromotionActionInput", len)?;
        if !self.action_type.is_empty() {
            struct_ser.serialize_field("actionType", &self.action_type)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePromotionActionInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action_type",
            "actionType",
            "configuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionType,
            Configuration,
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
                            "actionType" | "action_type" => Ok(GeneratedField::ActionType),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePromotionActionInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreatePromotionActionInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePromotionActionInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_type__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionType => {
                            if action_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionType"));
                            }
                            action_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePromotionActionInput {
                    action_type: action_type__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreatePromotionActionInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePromotionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code.is_some() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        if self.max_usage_total.is_some() {
            len += 1;
        }
        if self.max_usage_per_customer.is_some() {
            len += 1;
        }
        if self.is_exclusive.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreatePromotionRequest", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        if let Some(v) = self.max_usage_total.as_ref() {
            struct_ser.serialize_field("maxUsageTotal", v)?;
        }
        if let Some(v) = self.max_usage_per_customer.as_ref() {
            struct_ser.serialize_field("maxUsagePerCustomer", v)?;
        }
        if let Some(v) = self.is_exclusive.as_ref() {
            struct_ser.serialize_field("isExclusive", v)?;
        }
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePromotionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "type",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "is_active",
            "isActive",
            "priority",
            "max_usage_total",
            "maxUsageTotal",
            "max_usage_per_customer",
            "maxUsagePerCustomer",
            "is_exclusive",
            "isExclusive",
            "translations",
            "rules",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Type,
            ValidFrom,
            ValidTo,
            IsActive,
            Priority,
            MaxUsageTotal,
            MaxUsagePerCustomer,
            IsExclusive,
            Translations,
            Rules,
            Actions,
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
                            "type" => Ok(GeneratedField::Type),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "priority" => Ok(GeneratedField::Priority),
                            "maxUsageTotal" | "max_usage_total" => Ok(GeneratedField::MaxUsageTotal),
                            "maxUsagePerCustomer" | "max_usage_per_customer" => Ok(GeneratedField::MaxUsagePerCustomer),
                            "isExclusive" | "is_exclusive" => Ok(GeneratedField::IsExclusive),
                            "translations" => Ok(GeneratedField::Translations),
                            "rules" => Ok(GeneratedField::Rules),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePromotionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreatePromotionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePromotionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut r#type__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut is_active__ = None;
                let mut priority__ = None;
                let mut max_usage_total__ = None;
                let mut max_usage_per_customer__ = None;
                let mut is_exclusive__ = None;
                let mut translations__ = None;
                let mut rules__ = None;
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxUsageTotal => {
                            if max_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsageTotal"));
                            }
                            max_usage_total__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxUsagePerCustomer => {
                            if max_usage_per_customer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsagePerCustomer"));
                            }
                            max_usage_per_customer__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsExclusive => {
                            if is_exclusive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExclusive"));
                            }
                            is_exclusive__ = map_.next_value()?;
                        }
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreatePromotionRequest {
                    code: code__,
                    r#type: r#type__.unwrap_or_default(),
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    is_active: is_active__,
                    priority: priority__,
                    max_usage_total: max_usage_total__,
                    max_usage_per_customer: max_usage_per_customer__,
                    is_exclusive: is_exclusive__,
                    translations: translations__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreatePromotionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePromotionRuleInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule_group_id != 0 {
            len += 1;
        }
        if !self.rule_type.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.CreatePromotionRuleInput", len)?;
        if self.rule_group_id != 0 {
            struct_ser.serialize_field("ruleGroupId", &self.rule_group_id)?;
        }
        if !self.rule_type.is_empty() {
            struct_ser.serialize_field("ruleType", &self.rule_type)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePromotionRuleInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_group_id",
            "ruleGroupId",
            "rule_type",
            "ruleType",
            "configuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleGroupId,
            RuleType,
            Configuration,
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
                            "ruleGroupId" | "rule_group_id" => Ok(GeneratedField::RuleGroupId),
                            "ruleType" | "rule_type" => Ok(GeneratedField::RuleType),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePromotionRuleInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.CreatePromotionRuleInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePromotionRuleInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_group_id__ = None;
                let mut rule_type__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleGroupId => {
                            if rule_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleGroupId"));
                            }
                            rule_group_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RuleType => {
                            if rule_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleType"));
                            }
                            rule_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePromotionRuleInput {
                    rule_group_id: rule_group_id__.unwrap_or_default(),
                    rule_type: rule_type__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.CreatePromotionRuleInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCouponByCodeRequest {
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
        if self.locale_input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.GetCouponByCodeRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCouponByCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "locale_input",
            "localeInput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
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
            type Value = GetCouponByCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.GetCouponByCodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCouponByCodeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut locale_input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCouponByCodeRequest {
                    code: code__.unwrap_or_default(),
                    locale_input: locale_input__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.GetCouponByCodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCouponResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.coupon.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.GetCouponResponse", len)?;
        if let Some(v) = self.coupon.as_ref() {
            struct_ser.serialize_field("coupon", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCouponResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "coupon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Coupon,
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
                            "coupon" => Ok(GeneratedField::Coupon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCouponResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.GetCouponResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCouponResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut coupon__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Coupon => {
                            if coupon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coupon"));
                            }
                            coupon__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCouponResponse {
                    coupon: coupon__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.GetCouponResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPriceListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_list.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.GetPriceListResponse", len)?;
        if let Some(v) = self.price_list.as_ref() {
            struct_ser.serialize_field("priceList", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPriceListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_list",
            "priceList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceList,
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
                            "priceList" | "price_list" => Ok(GeneratedField::PriceList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPriceListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.GetPriceListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPriceListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_list__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceList => {
                            if price_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceList"));
                            }
                            price_list__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPriceListResponse {
                    price_list: price_list__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.GetPriceListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPromotionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.promotion.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.GetPromotionResponse", len)?;
        if let Some(v) = self.promotion.as_ref() {
            struct_ser.serialize_field("promotion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPromotionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "promotion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Promotion,
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
                            "promotion" => Ok(GeneratedField::Promotion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPromotionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.GetPromotionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPromotionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut promotion__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Promotion => {
                            if promotion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotion"));
                            }
                            promotion__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPromotionResponse {
                    promotion: promotion__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.GetPromotionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCouponsRequest {
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
        if self.filter_by_promotion_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.ListCouponsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_promotion_id.as_ref() {
            struct_ser.serialize_field("filterByPromotionId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCouponsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_promotion_id",
            "filterByPromotionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByPromotionId,
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
                            "filterByPromotionId" | "filter_by_promotion_id" => Ok(GeneratedField::FilterByPromotionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListCouponsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.ListCouponsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCouponsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_promotion_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByPromotionId => {
                            if filter_by_promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByPromotionId"));
                            }
                            filter_by_promotion_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListCouponsRequest {
                    base_request: base_request__,
                    filter_by_promotion_id: filter_by_promotion_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.ListCouponsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCouponsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.coupons.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.ListCouponsResponse", len)?;
        if !self.coupons.is_empty() {
            struct_ser.serialize_field("coupons", &self.coupons)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCouponsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "coupons",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Coupons,
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
                            "coupons" => Ok(GeneratedField::Coupons),
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
            type Value = ListCouponsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.ListCouponsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCouponsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut coupons__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Coupons => {
                            if coupons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coupons"));
                            }
                            coupons__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListCouponsResponse {
                    coupons: coupons__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.ListCouponsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPriceListsRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.ListPriceListsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPriceListsRequest {
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
            type Value = ListPriceListsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.ListPriceListsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPriceListsRequest, V::Error>
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
                Ok(ListPriceListsRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.ListPriceListsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPriceListsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_lists.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.ListPriceListsResponse", len)?;
        if !self.price_lists.is_empty() {
            struct_ser.serialize_field("priceLists", &self.price_lists)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPriceListsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_lists",
            "priceLists",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceLists,
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
                            "priceLists" | "price_lists" => Ok(GeneratedField::PriceLists),
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
            type Value = ListPriceListsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.ListPriceListsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPriceListsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_lists__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceLists => {
                            if price_lists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceLists"));
                            }
                            price_lists__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPriceListsResponse {
                    price_lists: price_lists__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.ListPriceListsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPromotionsRequest {
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
        if self.filter_by_is_currently_valid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.ListPromotionsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_is_currently_valid.as_ref() {
            struct_ser.serialize_field("filterByIsCurrentlyValid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPromotionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_is_currently_valid",
            "filterByIsCurrentlyValid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByIsCurrentlyValid,
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
                            "filterByIsCurrentlyValid" | "filter_by_is_currently_valid" => Ok(GeneratedField::FilterByIsCurrentlyValid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPromotionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.ListPromotionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPromotionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_is_currently_valid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByIsCurrentlyValid => {
                            if filter_by_is_currently_valid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByIsCurrentlyValid"));
                            }
                            filter_by_is_currently_valid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPromotionsRequest {
                    base_request: base_request__,
                    filter_by_is_currently_valid: filter_by_is_currently_valid__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.ListPromotionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPromotionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.promotions.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.ListPromotionsResponse", len)?;
        if !self.promotions.is_empty() {
            struct_ser.serialize_field("promotions", &self.promotions)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPromotionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "promotions",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Promotions,
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
                            "promotions" => Ok(GeneratedField::Promotions),
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
            type Value = ListPromotionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.ListPromotionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPromotionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut promotions__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Promotions => {
                            if promotions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotions"));
                            }
                            promotions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPromotionsResponse {
                    promotions: promotions__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.ListPromotionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PriceList {
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
        if !self.currency_code.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
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
        if !self.assignments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.PriceList", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.currency_code.is_empty() {
            struct_ser.serialize_field("currencyCode", &self.currency_code)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
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
        if !self.assignments.is_empty() {
            struct_ser.serialize_field("assignments", &self.assignments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PriceList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "currency_code",
            "currencyCode",
            "is_active",
            "isActive",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "description",
            "all_translations",
            "allTranslations",
            "items",
            "assignments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            CurrencyCode,
            IsActive,
            ValidFrom,
            ValidTo,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            AllTranslations,
            Items,
            Assignments,
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
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "items" => Ok(GeneratedField::Items),
                            "assignments" => Ok(GeneratedField::Assignments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.PriceList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut currency_code__ = None;
                let mut is_active__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut all_translations__ = None;
                let mut items__ = None;
                let mut assignments__ = None;
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
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
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
                        GeneratedField::Assignments => {
                            if assignments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignments"));
                            }
                            assignments__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PriceList {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    currency_code: currency_code__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                    assignments: assignments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.PriceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PriceListAssignment {
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
        if self.warehouse_id.is_some() {
            len += 1;
        }
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.PriceListAssignment", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PriceListAssignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "sales_channel_id",
            "salesChannelId",
            "priority",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            SalesChannelId,
            Priority,
            StartDate,
            EndDate,
            CreatedAt,
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
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "priority" => Ok(GeneratedField::Priority),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceListAssignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.PriceListAssignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceListAssignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut sales_channel_id__ = None;
                let mut priority__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut created_at__ = None;
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
                            warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
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
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PriceListAssignment {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__,
                    sales_channel_id: sales_channel_id__,
                    priority: priority__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.PriceListAssignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PriceListItem {
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
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
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
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.PriceListItem", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
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
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
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
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PriceListItem {
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
            "unit_id",
            "unitId",
            "price",
            "product_summary",
            "productSummary",
            "variant_summary",
            "variantSummary",
            "unit_summary",
            "unitSummary",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProductId,
            VariantId,
            UnitId,
            Price,
            ProductSummary,
            VariantSummary,
            UnitSummary,
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
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "price" => Ok(GeneratedField::Price),
                            "productSummary" | "product_summary" => Ok(GeneratedField::ProductSummary),
                            "variantSummary" | "variant_summary" => Ok(GeneratedField::VariantSummary),
                            "unitSummary" | "unit_summary" => Ok(GeneratedField::UnitSummary),
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
            type Value = PriceListItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.PriceListItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceListItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut unit_id__ = None;
                let mut price__ = None;
                let mut product_summary__ = None;
                let mut variant_summary__ = None;
                let mut unit_summary__ = None;
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
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
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
                Ok(PriceListItem {
                    id: id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    unit_id: unit_id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    product_summary: product_summary__,
                    variant_summary: variant_summary__,
                    unit_summary: unit_summary__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.PriceListItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Promotion {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.max_usage_total.is_some() {
            len += 1;
        }
        if self.max_usage_per_customer.is_some() {
            len += 1;
        }
        if self.is_exclusive {
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
        if !self.public_display_text.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.Promotion", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if let Some(v) = self.max_usage_total.as_ref() {
            struct_ser.serialize_field("maxUsageTotal", v)?;
        }
        if let Some(v) = self.max_usage_per_customer.as_ref() {
            struct_ser.serialize_field("maxUsagePerCustomer", v)?;
        }
        if self.is_exclusive {
            struct_ser.serialize_field("isExclusive", &self.is_exclusive)?;
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
        if !self.public_display_text.is_empty() {
            struct_ser.serialize_field("publicDisplayText", &self.public_display_text)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Promotion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "type",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "is_active",
            "isActive",
            "priority",
            "max_usage_total",
            "maxUsageTotal",
            "max_usage_per_customer",
            "maxUsagePerCustomer",
            "is_exclusive",
            "isExclusive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "description",
            "public_display_text",
            "publicDisplayText",
            "all_translations",
            "allTranslations",
            "rules",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            Type,
            ValidFrom,
            ValidTo,
            IsActive,
            Priority,
            MaxUsageTotal,
            MaxUsagePerCustomer,
            IsExclusive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            PublicDisplayText,
            AllTranslations,
            Rules,
            Actions,
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
                            "type" => Ok(GeneratedField::Type),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "priority" => Ok(GeneratedField::Priority),
                            "maxUsageTotal" | "max_usage_total" => Ok(GeneratedField::MaxUsageTotal),
                            "maxUsagePerCustomer" | "max_usage_per_customer" => Ok(GeneratedField::MaxUsagePerCustomer),
                            "isExclusive" | "is_exclusive" => Ok(GeneratedField::IsExclusive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "publicDisplayText" | "public_display_text" => Ok(GeneratedField::PublicDisplayText),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "rules" => Ok(GeneratedField::Rules),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Promotion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.Promotion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Promotion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut r#type__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut is_active__ = None;
                let mut priority__ = None;
                let mut max_usage_total__ = None;
                let mut max_usage_per_customer__ = None;
                let mut is_exclusive__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut public_display_text__ = None;
                let mut all_translations__ = None;
                let mut rules__ = None;
                let mut actions__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxUsageTotal => {
                            if max_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsageTotal"));
                            }
                            max_usage_total__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxUsagePerCustomer => {
                            if max_usage_per_customer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsagePerCustomer"));
                            }
                            max_usage_per_customer__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsExclusive => {
                            if is_exclusive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExclusive"));
                            }
                            is_exclusive__ = Some(map_.next_value()?);
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
                        GeneratedField::PublicDisplayText => {
                            if public_display_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicDisplayText"));
                            }
                            public_display_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Promotion {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    r#type: r#type__.unwrap_or_default(),
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    is_active: is_active__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    max_usage_total: max_usage_total__,
                    max_usage_per_customer: max_usage_per_customer__,
                    is_exclusive: is_exclusive__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    public_display_text: public_display_text__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.Promotion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PromotionAction {
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
        if !self.action_type.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.PromotionAction", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.action_type.is_empty() {
            struct_ser.serialize_field("actionType", &self.action_type)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PromotionAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "action_type",
            "actionType",
            "configuration",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ActionType,
            Configuration,
            CreatedAt,
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
                            "actionType" | "action_type" => Ok(GeneratedField::ActionType),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PromotionAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.PromotionAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PromotionAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut action_type__ = None;
                let mut configuration__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionType => {
                            if action_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionType"));
                            }
                            action_type__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(PromotionAction {
                    id: id__.unwrap_or_default(),
                    action_type: action_type__.unwrap_or_default(),
                    configuration: configuration__,
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.PromotionAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PromotionRule {
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
        if self.rule_group_id != 0 {
            len += 1;
        }
        if !self.rule_type.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.PromotionRule", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.rule_group_id != 0 {
            struct_ser.serialize_field("ruleGroupId", &self.rule_group_id)?;
        }
        if !self.rule_type.is_empty() {
            struct_ser.serialize_field("ruleType", &self.rule_type)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PromotionRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "rule_group_id",
            "ruleGroupId",
            "rule_type",
            "ruleType",
            "configuration",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            RuleGroupId,
            RuleType,
            Configuration,
            CreatedAt,
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
                            "ruleGroupId" | "rule_group_id" => Ok(GeneratedField::RuleGroupId),
                            "ruleType" | "rule_type" => Ok(GeneratedField::RuleType),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PromotionRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.PromotionRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PromotionRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut rule_group_id__ = None;
                let mut rule_type__ = None;
                let mut configuration__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleGroupId => {
                            if rule_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleGroupId"));
                            }
                            rule_group_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RuleType => {
                            if rule_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleType"));
                            }
                            rule_type__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(PromotionRule {
                    id: id__.unwrap_or_default(),
                    rule_group_id: rule_group_id__.unwrap_or_default(),
                    rule_type: rule_type__.unwrap_or_default(),
                    configuration: configuration__,
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.PromotionRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCouponRequest {
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
        if self.promotion_id.is_some() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if self.max_usage_total.is_some() {
            len += 1;
        }
        if self.max_usage_per_customer.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.UpdateCouponRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.promotion_id.as_ref() {
            struct_ser.serialize_field("promotionId", v)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if let Some(v) = self.max_usage_total.as_ref() {
            struct_ser.serialize_field("maxUsageTotal", v)?;
        }
        if let Some(v) = self.max_usage_per_customer.as_ref() {
            struct_ser.serialize_field("maxUsagePerCustomer", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateCouponRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "promotion_id",
            "promotionId",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "max_usage_total",
            "maxUsageTotal",
            "max_usage_per_customer",
            "maxUsagePerCustomer",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            PromotionId,
            ValidFrom,
            ValidTo,
            MaxUsageTotal,
            MaxUsagePerCustomer,
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
                            "promotionId" | "promotion_id" => Ok(GeneratedField::PromotionId),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "maxUsageTotal" | "max_usage_total" => Ok(GeneratedField::MaxUsageTotal),
                            "maxUsagePerCustomer" | "max_usage_per_customer" => Ok(GeneratedField::MaxUsagePerCustomer),
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
            type Value = UpdateCouponRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.UpdateCouponRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCouponRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut promotion_id__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut max_usage_total__ = None;
                let mut max_usage_per_customer__ = None;
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
                        GeneratedField::PromotionId => {
                            if promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionId"));
                            }
                            promotion_id__ = map_.next_value()?;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::MaxUsageTotal => {
                            if max_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsageTotal"));
                            }
                            max_usage_total__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxUsagePerCustomer => {
                            if max_usage_per_customer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUsagePerCustomer"));
                            }
                            max_usage_per_customer__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
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
                Ok(UpdateCouponRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    promotion_id: promotion_id__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    max_usage_total: max_usage_total__,
                    max_usage_per_customer: max_usage_per_customer__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.UpdateCouponRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePriceListAssignmentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assignment_id.is_empty() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.UpdatePriceListAssignmentRequest", len)?;
        if !self.assignment_id.is_empty() {
            struct_ser.serialize_field("assignmentId", &self.assignment_id)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePriceListAssignmentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assignment_id",
            "assignmentId",
            "priority",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssignmentId,
            Priority,
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
                            "assignmentId" | "assignment_id" => Ok(GeneratedField::AssignmentId),
                            "priority" => Ok(GeneratedField::Priority),
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
            type Value = UpdatePriceListAssignmentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.UpdatePriceListAssignmentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePriceListAssignmentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assignment_id__ = None;
                let mut priority__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssignmentId => {
                            if assignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignmentId"));
                            }
                            assignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
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
                Ok(UpdatePriceListAssignmentRequest {
                    assignment_id: assignment_id__.unwrap_or_default(),
                    priority: priority__,
                    start_date: start_date__,
                    end_date: end_date__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.UpdatePriceListAssignmentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePriceListItemRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_list_item_id.is_empty() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.UpdatePriceListItemRequest", len)?;
        if !self.price_list_item_id.is_empty() {
            struct_ser.serialize_field("priceListItemId", &self.price_list_item_id)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePriceListItemRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_list_item_id",
            "priceListItemId",
            "price",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceListItemId,
            Price,
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
                            "priceListItemId" | "price_list_item_id" => Ok(GeneratedField::PriceListItemId),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePriceListItemRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.UpdatePriceListItemRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePriceListItemRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_list_item_id__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceListItemId => {
                            if price_list_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceListItemId"));
                            }
                            price_list_item_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePriceListItemRequest {
                    price_list_item_id: price_list_item_id__.unwrap_or_default(),
                    price: price__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.UpdatePriceListItemRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePriceListRequest {
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
        if self.currency_code.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.UpdatePriceListRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.currency_code.as_ref() {
            struct_ser.serialize_field("currencyCode", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePriceListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "currency_code",
            "currencyCode",
            "is_active",
            "isActive",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            CurrencyCode,
            IsActive,
            ValidFrom,
            ValidTo,
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
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
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
            type Value = UpdatePriceListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.UpdatePriceListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePriceListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut currency_code__ = None;
                let mut is_active__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
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
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::TranslationsToUpdate => {
                            if translations_to_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translationsToUpdate"));
                            }
                            translations_to_update__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdatePriceListRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    currency_code: currency_code__,
                    is_active: is_active__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.UpdatePriceListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePromotionRequest {
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
        if self.r#type.is_some() {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.valid_to.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.pricing.v1.UpdatePromotionRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.valid_to.as_ref() {
            struct_ser.serialize_field("validTo", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePromotionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "type",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "is_active",
            "isActive",
            "priority",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            Type,
            ValidFrom,
            ValidTo,
            IsActive,
            Priority,
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
                            "type" => Ok(GeneratedField::Type),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "priority" => Ok(GeneratedField::Priority),
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
            type Value = UpdatePromotionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.pricing.v1.UpdatePromotionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePromotionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut r#type__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut is_active__ = None;
                let mut priority__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map_.next_value()?;
                        }
                        GeneratedField::ValidTo => {
                            if valid_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            valid_to__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
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
                Ok(UpdatePromotionRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    r#type: r#type__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    is_active: is_active__,
                    priority: priority__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.pricing.v1.UpdatePromotionRequest", FIELDS, GeneratedVisitor)
    }
}
