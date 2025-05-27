// @generated
impl serde::Serialize for CreatePaymentMethodRequest {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.CreatePaymentMethodRequest", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
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
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "type",
            "is_active",
            "isActive",
            "configuration",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Type,
            IsActive,
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
                            "type" => Ok(GeneratedField::Type),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = CreatePaymentMethodRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.CreatePaymentMethodRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePaymentMethodRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut r#type__ = None;
                let mut is_active__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
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
                Ok(CreatePaymentMethodRequest {
                    code: code__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    is_active: is_active__,
                    configuration: configuration__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.CreatePaymentMethodRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPaymentMethodResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payment_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.GetPaymentMethodResponse", len)?;
        if let Some(v) = self.payment_method.as_ref() {
            struct_ser.serialize_field("paymentMethod", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPaymentMethodResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payment_method",
            "paymentMethod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PaymentMethod,
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
                            "paymentMethod" | "payment_method" => Ok(GeneratedField::PaymentMethod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPaymentMethodResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.GetPaymentMethodResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPaymentMethodResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payment_method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PaymentMethod => {
                            if payment_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethod"));
                            }
                            payment_method__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPaymentMethodResponse {
                    payment_method: payment_method__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.GetPaymentMethodResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPaymentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.GetPaymentResponse", len)?;
        if let Some(v) = self.payment.as_ref() {
            struct_ser.serialize_field("payment", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPaymentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Payment,
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
                            "payment" => Ok(GeneratedField::Payment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPaymentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.GetPaymentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPaymentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Payment => {
                            if payment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            payment__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPaymentResponse {
                    payment: payment__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.GetPaymentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPaymentMethodsRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.ListPaymentMethodsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPaymentMethodsRequest {
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
            type Value = ListPaymentMethodsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.ListPaymentMethodsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPaymentMethodsRequest, V::Error>
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
                Ok(ListPaymentMethodsRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.ListPaymentMethodsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPaymentMethodsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payment_methods.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.ListPaymentMethodsResponse", len)?;
        if !self.payment_methods.is_empty() {
            struct_ser.serialize_field("paymentMethods", &self.payment_methods)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPaymentMethodsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payment_methods",
            "paymentMethods",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PaymentMethods,
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
                            "paymentMethods" | "payment_methods" => Ok(GeneratedField::PaymentMethods),
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
            type Value = ListPaymentMethodsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.ListPaymentMethodsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPaymentMethodsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payment_methods__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PaymentMethods => {
                            if payment_methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethods"));
                            }
                            payment_methods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPaymentMethodsResponse {
                    payment_methods: payment_methods__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.ListPaymentMethodsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPaymentsRequest {
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
        if self.filter_by_order_id.is_some() {
            len += 1;
        }
        if self.filter_by_customer_id.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        if self.filter_by_payment_method_id.is_some() {
            len += 1;
        }
        if self.filter_by_paid_date_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.ListPaymentsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_order_id.as_ref() {
            struct_ser.serialize_field("filterByOrderId", v)?;
        }
        if let Some(v) = self.filter_by_customer_id.as_ref() {
            struct_ser.serialize_field("filterByCustomerId", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        if let Some(v) = self.filter_by_payment_method_id.as_ref() {
            struct_ser.serialize_field("filterByPaymentMethodId", v)?;
        }
        if let Some(v) = self.filter_by_paid_date_range.as_ref() {
            struct_ser.serialize_field("filterByPaidDateRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPaymentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_order_id",
            "filterByOrderId",
            "filter_by_customer_id",
            "filterByCustomerId",
            "filter_by_status",
            "filterByStatus",
            "filter_by_payment_method_id",
            "filterByPaymentMethodId",
            "filter_by_paid_date_range",
            "filterByPaidDateRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByOrderId,
            FilterByCustomerId,
            FilterByStatus,
            FilterByPaymentMethodId,
            FilterByPaidDateRange,
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
                            "filterByOrderId" | "filter_by_order_id" => Ok(GeneratedField::FilterByOrderId),
                            "filterByCustomerId" | "filter_by_customer_id" => Ok(GeneratedField::FilterByCustomerId),
                            "filterByStatus" | "filter_by_status" => Ok(GeneratedField::FilterByStatus),
                            "filterByPaymentMethodId" | "filter_by_payment_method_id" => Ok(GeneratedField::FilterByPaymentMethodId),
                            "filterByPaidDateRange" | "filter_by_paid_date_range" => Ok(GeneratedField::FilterByPaidDateRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPaymentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.ListPaymentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPaymentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_order_id__ = None;
                let mut filter_by_customer_id__ = None;
                let mut filter_by_status__ = None;
                let mut filter_by_payment_method_id__ = None;
                let mut filter_by_paid_date_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByOrderId => {
                            if filter_by_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByOrderId"));
                            }
                            filter_by_order_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByCustomerId => {
                            if filter_by_customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByCustomerId"));
                            }
                            filter_by_customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByPaymentMethodId => {
                            if filter_by_payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByPaymentMethodId"));
                            }
                            filter_by_payment_method_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByPaidDateRange => {
                            if filter_by_paid_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByPaidDateRange"));
                            }
                            filter_by_paid_date_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPaymentsRequest {
                    base_request: base_request__,
                    filter_by_order_id: filter_by_order_id__,
                    filter_by_customer_id: filter_by_customer_id__,
                    filter_by_status: filter_by_status__,
                    filter_by_payment_method_id: filter_by_payment_method_id__,
                    filter_by_paid_date_range: filter_by_paid_date_range__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.ListPaymentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPaymentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payments.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.ListPaymentsResponse", len)?;
        if !self.payments.is_empty() {
            struct_ser.serialize_field("payments", &self.payments)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPaymentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payments",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Payments,
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
                            "payments" => Ok(GeneratedField::Payments),
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
            type Value = ListPaymentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.ListPaymentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPaymentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payments__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Payments => {
                            if payments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payments"));
                            }
                            payments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPaymentsResponse {
                    payments: payments__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.ListPaymentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Payment {
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
        if self.order_id.is_some() {
            len += 1;
        }
        if self.customer_id.is_some() {
            len += 1;
        }
        if !self.payment_method_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.transaction_reference.is_some() {
            len += 1;
        }
        if self.gateway_response.is_some() {
            len += 1;
        }
        if self.paid_at.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.payment_method_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.Payment", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.order_id.as_ref() {
            struct_ser.serialize_field("orderId", v)?;
        }
        if let Some(v) = self.customer_id.as_ref() {
            struct_ser.serialize_field("customerId", v)?;
        }
        if !self.payment_method_id.is_empty() {
            struct_ser.serialize_field("paymentMethodId", &self.payment_method_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.transaction_reference.as_ref() {
            struct_ser.serialize_field("transactionReference", v)?;
        }
        if let Some(v) = self.gateway_response.as_ref() {
            struct_ser.serialize_field("gatewayResponse", v)?;
        }
        if let Some(v) = self.paid_at.as_ref() {
            struct_ser.serialize_field("paidAt", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.payment_method_details.as_ref() {
            struct_ser.serialize_field("paymentMethodDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Payment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "order_id",
            "orderId",
            "customer_id",
            "customerId",
            "payment_method_id",
            "paymentMethodId",
            "amount",
            "status",
            "transaction_reference",
            "transactionReference",
            "gateway_response",
            "gatewayResponse",
            "paid_at",
            "paidAt",
            "created_by_user_id",
            "createdByUserId",
            "created_at",
            "createdAt",
            "payment_method_details",
            "paymentMethodDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            OrderId,
            CustomerId,
            PaymentMethodId,
            Amount,
            Status,
            TransactionReference,
            GatewayResponse,
            PaidAt,
            CreatedByUserId,
            CreatedAt,
            PaymentMethodDetails,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "customerId" | "customer_id" => Ok(GeneratedField::CustomerId),
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "amount" => Ok(GeneratedField::Amount),
                            "status" => Ok(GeneratedField::Status),
                            "transactionReference" | "transaction_reference" => Ok(GeneratedField::TransactionReference),
                            "gatewayResponse" | "gateway_response" => Ok(GeneratedField::GatewayResponse),
                            "paidAt" | "paid_at" => Ok(GeneratedField::PaidAt),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "paymentMethodDetails" | "payment_method_details" => Ok(GeneratedField::PaymentMethodDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Payment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.Payment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut order_id__ = None;
                let mut customer_id__ = None;
                let mut payment_method_id__ = None;
                let mut amount__ = None;
                let mut status__ = None;
                let mut transaction_reference__ = None;
                let mut gateway_response__ = None;
                let mut paid_at__ = None;
                let mut created_by_user_id__ = None;
                let mut created_at__ = None;
                let mut payment_method_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = map_.next_value()?;
                        }
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethodId => {
                            if payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodId"));
                            }
                            payment_method_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TransactionReference => {
                            if transaction_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionReference"));
                            }
                            transaction_reference__ = map_.next_value()?;
                        }
                        GeneratedField::GatewayResponse => {
                            if gateway_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gatewayResponse"));
                            }
                            gateway_response__ = map_.next_value()?;
                        }
                        GeneratedField::PaidAt => {
                            if paid_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paidAt"));
                            }
                            paid_at__ = map_.next_value()?;
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
                        GeneratedField::PaymentMethodDetails => {
                            if payment_method_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodDetails"));
                            }
                            payment_method_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Payment {
                    id: id__.unwrap_or_default(),
                    order_id: order_id__,
                    customer_id: customer_id__,
                    payment_method_id: payment_method_id__.unwrap_or_default(),
                    amount: amount__,
                    status: status__.unwrap_or_default(),
                    transaction_reference: transaction_reference__,
                    gateway_response: gateway_response__,
                    paid_at: paid_at__,
                    created_by_user_id: created_by_user_id__,
                    created_at: created_at__,
                    payment_method_details: payment_method_details__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.Payment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaymentMethod {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.is_active {
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
        if !self.user_guide.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.PaymentMethod", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
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
        if !self.user_guide.is_empty() {
            struct_ser.serialize_field("userGuide", &self.user_guide)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "type",
            "is_active",
            "isActive",
            "configuration",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "user_guide",
            "userGuide",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            Type,
            IsActive,
            Configuration,
            CreatedAt,
            UpdatedAt,
            Name,
            UserGuide,
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
                            "type" => Ok(GeneratedField::Type),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "userGuide" | "user_guide" => Ok(GeneratedField::UserGuide),
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
            type Value = PaymentMethod;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.PaymentMethod")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PaymentMethod, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut r#type__ = None;
                let mut is_active__ = None;
                let mut configuration__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut user_guide__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
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
                        GeneratedField::UserGuide => {
                            if user_guide__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGuide"));
                            }
                            user_guide__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PaymentMethod {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    configuration: configuration__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    user_guide: user_guide__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.PaymentMethod", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePaymentMethodRequest {
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
        if self.is_active.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.payment.v1.UpdatePaymentMethodRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdatePaymentMethodRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "type",
            "is_active",
            "isActive",
            "configuration",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            Type,
            IsActive,
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
                            "type" => Ok(GeneratedField::Type),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = UpdatePaymentMethodRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.payment.v1.UpdatePaymentMethodRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePaymentMethodRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut r#type__ = None;
                let mut is_active__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
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
                Ok(UpdatePaymentMethodRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    r#type: r#type__,
                    is_active: is_active__,
                    configuration: configuration__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.payment.v1.UpdatePaymentMethodRequest", FIELDS, GeneratedVisitor)
    }
}
