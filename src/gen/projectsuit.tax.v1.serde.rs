// @generated
impl serde::Serialize for AssignTaxRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tax_rate_id.is_empty() {
            len += 1;
        }
        if !self.tax_category_id.is_empty() {
            len += 1;
        }
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.warehouse_id.is_some() {
            len += 1;
        }
        if self.country_code.is_some() {
            len += 1;
        }
        if self.region_code.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.AssignTaxRateRequest", len)?;
        if !self.tax_rate_id.is_empty() {
            struct_ser.serialize_field("taxRateId", &self.tax_rate_id)?;
        }
        if !self.tax_category_id.is_empty() {
            struct_ser.serialize_field("taxCategoryId", &self.tax_category_id)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if let Some(v) = self.country_code.as_ref() {
            struct_ser.serialize_field("countryCode", v)?;
        }
        if let Some(v) = self.region_code.as_ref() {
            struct_ser.serialize_field("regionCode", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignTaxRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tax_rate_id",
            "taxRateId",
            "tax_category_id",
            "taxCategoryId",
            "sales_channel_id",
            "salesChannelId",
            "warehouse_id",
            "warehouseId",
            "country_code",
            "countryCode",
            "region_code",
            "regionCode",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaxRateId,
            TaxCategoryId,
            SalesChannelId,
            WarehouseId,
            CountryCode,
            RegionCode,
            Priority,
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
                            "taxRateId" | "tax_rate_id" => Ok(GeneratedField::TaxRateId),
                            "taxCategoryId" | "tax_category_id" => Ok(GeneratedField::TaxCategoryId),
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "countryCode" | "country_code" => Ok(GeneratedField::CountryCode),
                            "regionCode" | "region_code" => Ok(GeneratedField::RegionCode),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignTaxRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.AssignTaxRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignTaxRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tax_rate_id__ = None;
                let mut tax_category_id__ = None;
                let mut sales_channel_id__ = None;
                let mut warehouse_id__ = None;
                let mut country_code__ = None;
                let mut region_code__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaxRateId => {
                            if tax_rate_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxRateId"));
                            }
                            tax_rate_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TaxCategoryId => {
                            if tax_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxCategoryId"));
                            }
                            tax_category_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::CountryCode => {
                            if country_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryCode"));
                            }
                            country_code__ = map_.next_value()?;
                        }
                        GeneratedField::RegionCode => {
                            if region_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regionCode"));
                            }
                            region_code__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(AssignTaxRateRequest {
                    tax_rate_id: tax_rate_id__.unwrap_or_default(),
                    tax_category_id: tax_category_id__.unwrap_or_default(),
                    sales_channel_id: sales_channel_id__,
                    warehouse_id: warehouse_id__,
                    country_code: country_code__,
                    region_code: region_code__,
                    priority: priority__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.AssignTaxRateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTaxCategoryRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.CreateTaxCategoryRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for CreateTaxCategoryRequest {
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
            type Value = CreateTaxCategoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.CreateTaxCategoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTaxCategoryRequest, V::Error>
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
                Ok(CreateTaxCategoryRequest {
                    code: code__.unwrap_or_default(),
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.CreateTaxCategoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTaxRateRequest {
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
        if !self.rate_percentage.is_empty() {
            len += 1;
        }
        if !self.calculation_type.is_empty() {
            len += 1;
        }
        if self.fixed_amount.is_some() {
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
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.CreateTaxRateRequest", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if !self.rate_percentage.is_empty() {
            struct_ser.serialize_field("ratePercentage", &self.rate_percentage)?;
        }
        if !self.calculation_type.is_empty() {
            struct_ser.serialize_field("calculationType", &self.calculation_type)?;
        }
        if let Some(v) = self.fixed_amount.as_ref() {
            struct_ser.serialize_field("fixedAmount", v)?;
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
        if !self.translations.is_empty() {
            struct_ser.serialize_field("translations", &self.translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTaxRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "rate_percentage",
            "ratePercentage",
            "calculation_type",
            "calculationType",
            "fixed_amount",
            "fixedAmount",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            RatePercentage,
            CalculationType,
            FixedAmount,
            ValidFrom,
            ValidTo,
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
                            "ratePercentage" | "rate_percentage" => Ok(GeneratedField::RatePercentage),
                            "calculationType" | "calculation_type" => Ok(GeneratedField::CalculationType),
                            "fixedAmount" | "fixed_amount" => Ok(GeneratedField::FixedAmount),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
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
            type Value = CreateTaxRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.CreateTaxRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTaxRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut rate_percentage__ = None;
                let mut calculation_type__ = None;
                let mut fixed_amount__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::RatePercentage => {
                            if rate_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ratePercentage"));
                            }
                            rate_percentage__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CalculationType => {
                            if calculation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculationType"));
                            }
                            calculation_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FixedAmount => {
                            if fixed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAmount"));
                            }
                            fixed_amount__ = map_.next_value()?;
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
                        GeneratedField::Translations => {
                            if translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translations"));
                            }
                            translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTaxRateRequest {
                    code: code__,
                    rate_percentage: rate_percentage__.unwrap_or_default(),
                    calculation_type: calculation_type__.unwrap_or_default(),
                    fixed_amount: fixed_amount__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.CreateTaxRateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTaxCategoryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tax_category.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.GetTaxCategoryResponse", len)?;
        if let Some(v) = self.tax_category.as_ref() {
            struct_ser.serialize_field("taxCategory", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTaxCategoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tax_category",
            "taxCategory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaxCategory,
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
                            "taxCategory" | "tax_category" => Ok(GeneratedField::TaxCategory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTaxCategoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.GetTaxCategoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTaxCategoryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tax_category__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaxCategory => {
                            if tax_category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxCategory"));
                            }
                            tax_category__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTaxCategoryResponse {
                    tax_category: tax_category__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.GetTaxCategoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTaxRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tax_rate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.GetTaxRateResponse", len)?;
        if let Some(v) = self.tax_rate.as_ref() {
            struct_ser.serialize_field("taxRate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTaxRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tax_rate",
            "taxRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaxRate,
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
                            "taxRate" | "tax_rate" => Ok(GeneratedField::TaxRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTaxRateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.GetTaxRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTaxRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tax_rate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaxRate => {
                            if tax_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxRate"));
                            }
                            tax_rate__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTaxRateResponse {
                    tax_rate: tax_rate__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.GetTaxRateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaxAssignmentsRequest {
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
        if self.filter_by_tax_rate_id.is_some() {
            len += 1;
        }
        if self.filter_by_tax_category_id.is_some() {
            len += 1;
        }
        if self.filter_by_sales_channel_id.is_some() {
            len += 1;
        }
        if self.filter_by_warehouse_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.ListTaxAssignmentsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_tax_rate_id.as_ref() {
            struct_ser.serialize_field("filterByTaxRateId", v)?;
        }
        if let Some(v) = self.filter_by_tax_category_id.as_ref() {
            struct_ser.serialize_field("filterByTaxCategoryId", v)?;
        }
        if let Some(v) = self.filter_by_sales_channel_id.as_ref() {
            struct_ser.serialize_field("filterBySalesChannelId", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaxAssignmentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_tax_rate_id",
            "filterByTaxRateId",
            "filter_by_tax_category_id",
            "filterByTaxCategoryId",
            "filter_by_sales_channel_id",
            "filterBySalesChannelId",
            "filter_by_warehouse_id",
            "filterByWarehouseId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByTaxRateId,
            FilterByTaxCategoryId,
            FilterBySalesChannelId,
            FilterByWarehouseId,
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
                            "filterByTaxRateId" | "filter_by_tax_rate_id" => Ok(GeneratedField::FilterByTaxRateId),
                            "filterByTaxCategoryId" | "filter_by_tax_category_id" => Ok(GeneratedField::FilterByTaxCategoryId),
                            "filterBySalesChannelId" | "filter_by_sales_channel_id" => Ok(GeneratedField::FilterBySalesChannelId),
                            "filterByWarehouseId" | "filter_by_warehouse_id" => Ok(GeneratedField::FilterByWarehouseId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTaxAssignmentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.ListTaxAssignmentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaxAssignmentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_tax_rate_id__ = None;
                let mut filter_by_tax_category_id__ = None;
                let mut filter_by_sales_channel_id__ = None;
                let mut filter_by_warehouse_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByTaxRateId => {
                            if filter_by_tax_rate_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByTaxRateId"));
                            }
                            filter_by_tax_rate_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByTaxCategoryId => {
                            if filter_by_tax_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByTaxCategoryId"));
                            }
                            filter_by_tax_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterBySalesChannelId => {
                            if filter_by_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterBySalesChannelId"));
                            }
                            filter_by_sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByWarehouseId => {
                            if filter_by_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByWarehouseId"));
                            }
                            filter_by_warehouse_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListTaxAssignmentsRequest {
                    base_request: base_request__,
                    filter_by_tax_rate_id: filter_by_tax_rate_id__,
                    filter_by_tax_category_id: filter_by_tax_category_id__,
                    filter_by_sales_channel_id: filter_by_sales_channel_id__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.ListTaxAssignmentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaxAssignmentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assignments.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.ListTaxAssignmentsResponse", len)?;
        if !self.assignments.is_empty() {
            struct_ser.serialize_field("assignments", &self.assignments)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaxAssignmentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assignments",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assignments,
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
                            "assignments" => Ok(GeneratedField::Assignments),
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
            type Value = ListTaxAssignmentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.ListTaxAssignmentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaxAssignmentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assignments__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assignments => {
                            if assignments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignments"));
                            }
                            assignments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListTaxAssignmentsResponse {
                    assignments: assignments__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.ListTaxAssignmentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaxCategoriesRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.ListTaxCategoriesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaxCategoriesRequest {
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
            type Value = ListTaxCategoriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.ListTaxCategoriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaxCategoriesRequest, V::Error>
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
                Ok(ListTaxCategoriesRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.ListTaxCategoriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaxCategoriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tax_categories.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.ListTaxCategoriesResponse", len)?;
        if !self.tax_categories.is_empty() {
            struct_ser.serialize_field("taxCategories", &self.tax_categories)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaxCategoriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tax_categories",
            "taxCategories",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaxCategories,
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
                            "taxCategories" | "tax_categories" => Ok(GeneratedField::TaxCategories),
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
            type Value = ListTaxCategoriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.ListTaxCategoriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaxCategoriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tax_categories__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaxCategories => {
                            if tax_categories__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxCategories"));
                            }
                            tax_categories__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListTaxCategoriesResponse {
                    tax_categories: tax_categories__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.ListTaxCategoriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaxRatesRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.ListTaxRatesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_is_currently_valid.as_ref() {
            struct_ser.serialize_field("filterByIsCurrentlyValid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaxRatesRequest {
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
            type Value = ListTaxRatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.ListTaxRatesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaxRatesRequest, V::Error>
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
                Ok(ListTaxRatesRequest {
                    base_request: base_request__,
                    filter_by_is_currently_valid: filter_by_is_currently_valid__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.ListTaxRatesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaxRatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tax_rates.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.ListTaxRatesResponse", len)?;
        if !self.tax_rates.is_empty() {
            struct_ser.serialize_field("taxRates", &self.tax_rates)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaxRatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tax_rates",
            "taxRates",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaxRates,
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
                            "taxRates" | "tax_rates" => Ok(GeneratedField::TaxRates),
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
            type Value = ListTaxRatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.ListTaxRatesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaxRatesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tax_rates__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaxRates => {
                            if tax_rates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxRates"));
                            }
                            tax_rates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListTaxRatesResponse {
                    tax_rates: tax_rates__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.ListTaxRatesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaxAssignment {
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
        if !self.tax_rate_id.is_empty() {
            len += 1;
        }
        if self.tax_rate_details.is_some() {
            len += 1;
        }
        if !self.tax_category_id.is_empty() {
            len += 1;
        }
        if self.tax_category_details.is_some() {
            len += 1;
        }
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.warehouse_id.is_some() {
            len += 1;
        }
        if self.country_code.is_some() {
            len += 1;
        }
        if self.region_code.is_some() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.TaxAssignment", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.tax_rate_id.is_empty() {
            struct_ser.serialize_field("taxRateId", &self.tax_rate_id)?;
        }
        if let Some(v) = self.tax_rate_details.as_ref() {
            struct_ser.serialize_field("taxRateDetails", v)?;
        }
        if !self.tax_category_id.is_empty() {
            struct_ser.serialize_field("taxCategoryId", &self.tax_category_id)?;
        }
        if let Some(v) = self.tax_category_details.as_ref() {
            struct_ser.serialize_field("taxCategoryDetails", v)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if let Some(v) = self.country_code.as_ref() {
            struct_ser.serialize_field("countryCode", v)?;
        }
        if let Some(v) = self.region_code.as_ref() {
            struct_ser.serialize_field("regionCode", v)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
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
impl<'de> serde::Deserialize<'de> for TaxAssignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "tax_rate_id",
            "taxRateId",
            "tax_rate_details",
            "taxRateDetails",
            "tax_category_id",
            "taxCategoryId",
            "tax_category_details",
            "taxCategoryDetails",
            "sales_channel_id",
            "salesChannelId",
            "warehouse_id",
            "warehouseId",
            "country_code",
            "countryCode",
            "region_code",
            "regionCode",
            "priority",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TaxRateId,
            TaxRateDetails,
            TaxCategoryId,
            TaxCategoryDetails,
            SalesChannelId,
            WarehouseId,
            CountryCode,
            RegionCode,
            Priority,
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
                            "taxRateId" | "tax_rate_id" => Ok(GeneratedField::TaxRateId),
                            "taxRateDetails" | "tax_rate_details" => Ok(GeneratedField::TaxRateDetails),
                            "taxCategoryId" | "tax_category_id" => Ok(GeneratedField::TaxCategoryId),
                            "taxCategoryDetails" | "tax_category_details" => Ok(GeneratedField::TaxCategoryDetails),
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "countryCode" | "country_code" => Ok(GeneratedField::CountryCode),
                            "regionCode" | "region_code" => Ok(GeneratedField::RegionCode),
                            "priority" => Ok(GeneratedField::Priority),
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
            type Value = TaxAssignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.TaxAssignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaxAssignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut tax_rate_id__ = None;
                let mut tax_rate_details__ = None;
                let mut tax_category_id__ = None;
                let mut tax_category_details__ = None;
                let mut sales_channel_id__ = None;
                let mut warehouse_id__ = None;
                let mut country_code__ = None;
                let mut region_code__ = None;
                let mut priority__ = None;
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
                        GeneratedField::TaxRateId => {
                            if tax_rate_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxRateId"));
                            }
                            tax_rate_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TaxRateDetails => {
                            if tax_rate_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxRateDetails"));
                            }
                            tax_rate_details__ = map_.next_value()?;
                        }
                        GeneratedField::TaxCategoryId => {
                            if tax_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxCategoryId"));
                            }
                            tax_category_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TaxCategoryDetails => {
                            if tax_category_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxCategoryDetails"));
                            }
                            tax_category_details__ = map_.next_value()?;
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::CountryCode => {
                            if country_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryCode"));
                            }
                            country_code__ = map_.next_value()?;
                        }
                        GeneratedField::RegionCode => {
                            if region_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regionCode"));
                            }
                            region_code__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
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
                Ok(TaxAssignment {
                    id: id__.unwrap_or_default(),
                    tax_rate_id: tax_rate_id__.unwrap_or_default(),
                    tax_rate_details: tax_rate_details__,
                    tax_category_id: tax_category_id__.unwrap_or_default(),
                    tax_category_details: tax_category_details__,
                    sales_channel_id: sales_channel_id__,
                    warehouse_id: warehouse_id__,
                    country_code: country_code__,
                    region_code: region_code__,
                    priority: priority__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.TaxAssignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaxCategory {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.TaxCategory", len)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaxCategory {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaxCategory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.TaxCategory")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaxCategory, V::Error>
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
                    }
                }
                Ok(TaxCategory {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.TaxCategory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaxRate {
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
        if !self.rate_percentage.is_empty() {
            len += 1;
        }
        if !self.calculation_type.is_empty() {
            len += 1;
        }
        if self.fixed_amount.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.TaxRate", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if !self.rate_percentage.is_empty() {
            struct_ser.serialize_field("ratePercentage", &self.rate_percentage)?;
        }
        if !self.calculation_type.is_empty() {
            struct_ser.serialize_field("calculationType", &self.calculation_type)?;
        }
        if let Some(v) = self.fixed_amount.as_ref() {
            struct_ser.serialize_field("fixedAmount", v)?;
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
impl<'de> serde::Deserialize<'de> for TaxRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "rate_percentage",
            "ratePercentage",
            "calculation_type",
            "calculationType",
            "fixed_amount",
            "fixedAmount",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
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
            RatePercentage,
            CalculationType,
            FixedAmount,
            ValidFrom,
            ValidTo,
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
                            "ratePercentage" | "rate_percentage" => Ok(GeneratedField::RatePercentage),
                            "calculationType" | "calculation_type" => Ok(GeneratedField::CalculationType),
                            "fixedAmount" | "fixed_amount" => Ok(GeneratedField::FixedAmount),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
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
            type Value = TaxRate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.TaxRate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaxRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut rate_percentage__ = None;
                let mut calculation_type__ = None;
                let mut fixed_amount__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
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
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::RatePercentage => {
                            if rate_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ratePercentage"));
                            }
                            rate_percentage__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CalculationType => {
                            if calculation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculationType"));
                            }
                            calculation_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FixedAmount => {
                            if fixed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAmount"));
                            }
                            fixed_amount__ = map_.next_value()?;
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
                Ok(TaxRate {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    rate_percentage: rate_percentage__.unwrap_or_default(),
                    calculation_type: calculation_type__.unwrap_or_default(),
                    fixed_amount: fixed_amount__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.TaxRate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTaxAssignmentRequest {
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
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.warehouse_id.is_some() {
            len += 1;
        }
        if self.country_code.is_some() {
            len += 1;
        }
        if self.region_code.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.UpdateTaxAssignmentRequest", len)?;
        if !self.assignment_id.is_empty() {
            struct_ser.serialize_field("assignmentId", &self.assignment_id)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if let Some(v) = self.country_code.as_ref() {
            struct_ser.serialize_field("countryCode", v)?;
        }
        if let Some(v) = self.region_code.as_ref() {
            struct_ser.serialize_field("regionCode", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTaxAssignmentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assignment_id",
            "assignmentId",
            "sales_channel_id",
            "salesChannelId",
            "warehouse_id",
            "warehouseId",
            "country_code",
            "countryCode",
            "region_code",
            "regionCode",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssignmentId,
            SalesChannelId,
            WarehouseId,
            CountryCode,
            RegionCode,
            Priority,
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
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "countryCode" | "country_code" => Ok(GeneratedField::CountryCode),
                            "regionCode" | "region_code" => Ok(GeneratedField::RegionCode),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTaxAssignmentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.UpdateTaxAssignmentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTaxAssignmentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assignment_id__ = None;
                let mut sales_channel_id__ = None;
                let mut warehouse_id__ = None;
                let mut country_code__ = None;
                let mut region_code__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssignmentId => {
                            if assignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignmentId"));
                            }
                            assignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::CountryCode => {
                            if country_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryCode"));
                            }
                            country_code__ = map_.next_value()?;
                        }
                        GeneratedField::RegionCode => {
                            if region_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regionCode"));
                            }
                            region_code__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(UpdateTaxAssignmentRequest {
                    assignment_id: assignment_id__.unwrap_or_default(),
                    sales_channel_id: sales_channel_id__,
                    warehouse_id: warehouse_id__,
                    country_code: country_code__,
                    region_code: region_code__,
                    priority: priority__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.UpdateTaxAssignmentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTaxCategoryRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.UpdateTaxCategoryRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for UpdateTaxCategoryRequest {
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
            type Value = UpdateTaxCategoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.UpdateTaxCategoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTaxCategoryRequest, V::Error>
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
                Ok(UpdateTaxCategoryRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.UpdateTaxCategoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTaxRateRequest {
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
        if self.rate_percentage.is_some() {
            len += 1;
        }
        if self.calculation_type.is_some() {
            len += 1;
        }
        if self.fixed_amount.is_some() {
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
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.tax.v1.UpdateTaxRateRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.rate_percentage.as_ref() {
            struct_ser.serialize_field("ratePercentage", v)?;
        }
        if let Some(v) = self.calculation_type.as_ref() {
            struct_ser.serialize_field("calculationType", v)?;
        }
        if let Some(v) = self.fixed_amount.as_ref() {
            struct_ser.serialize_field("fixedAmount", v)?;
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
        if !self.translations_to_update.is_empty() {
            struct_ser.serialize_field("translationsToUpdate", &self.translations_to_update)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTaxRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "rate_percentage",
            "ratePercentage",
            "calculation_type",
            "calculationType",
            "fixed_amount",
            "fixedAmount",
            "valid_from",
            "validFrom",
            "valid_to",
            "validTo",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            RatePercentage,
            CalculationType,
            FixedAmount,
            ValidFrom,
            ValidTo,
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
                            "ratePercentage" | "rate_percentage" => Ok(GeneratedField::RatePercentage),
                            "calculationType" | "calculation_type" => Ok(GeneratedField::CalculationType),
                            "fixedAmount" | "fixed_amount" => Ok(GeneratedField::FixedAmount),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "validTo" | "valid_to" => Ok(GeneratedField::ValidTo),
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
            type Value = UpdateTaxRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.tax.v1.UpdateTaxRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTaxRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut rate_percentage__ = None;
                let mut calculation_type__ = None;
                let mut fixed_amount__ = None;
                let mut valid_from__ = None;
                let mut valid_to__ = None;
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
                        GeneratedField::RatePercentage => {
                            if rate_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ratePercentage"));
                            }
                            rate_percentage__ = map_.next_value()?;
                        }
                        GeneratedField::CalculationType => {
                            if calculation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculationType"));
                            }
                            calculation_type__ = map_.next_value()?;
                        }
                        GeneratedField::FixedAmount => {
                            if fixed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAmount"));
                            }
                            fixed_amount__ = map_.next_value()?;
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
                        GeneratedField::TranslationsToUpdate => {
                            if translations_to_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translationsToUpdate"));
                            }
                            translations_to_update__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateTaxRateRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    rate_percentage: rate_percentage__,
                    calculation_type: calculation_type__,
                    fixed_amount: fixed_amount__,
                    valid_from: valid_from__,
                    valid_to: valid_to__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.tax.v1.UpdateTaxRateRequest", FIELDS, GeneratedVisitor)
    }
}
