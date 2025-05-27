// @generated
impl serde::Serialize for AddItemToOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.product_id.is_empty() {
            len += 1;
        }
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.suggested_unit_price.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.AddItemToOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.suggested_unit_price.as_ref() {
            struct_ser.serialize_field("suggestedUnitPrice", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddItemToOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "product_id",
            "productId",
            "variant_id",
            "variantId",
            "quantity",
            "unit_id",
            "unitId",
            "suggested_unit_price",
            "suggestedUnitPrice",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            ProductId,
            VariantId,
            Quantity,
            UnitId,
            SuggestedUnitPrice,
            Notes,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "productId" | "product_id" => Ok(GeneratedField::ProductId),
                            "variantId" | "variant_id" => Ok(GeneratedField::VariantId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "suggestedUnitPrice" | "suggested_unit_price" => Ok(GeneratedField::SuggestedUnitPrice),
                            "notes" => Ok(GeneratedField::Notes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddItemToOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.AddItemToOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddItemToOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut quantity__ = None;
                let mut unit_id__ = None;
                let mut suggested_unit_price__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
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
                        GeneratedField::SuggestedUnitPrice => {
                            if suggested_unit_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suggestedUnitPrice"));
                            }
                            suggested_unit_price__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddItemToOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    quantity: quantity__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    suggested_unit_price: suggested_unit_price__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.AddItemToOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplyCouponToOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.coupon_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.ApplyCouponToOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.coupon_code.is_empty() {
            struct_ser.serialize_field("couponCode", &self.coupon_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplyCouponToOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "coupon_code",
            "couponCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            CouponCode,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "couponCode" | "coupon_code" => Ok(GeneratedField::CouponCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplyCouponToOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.ApplyCouponToOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApplyCouponToOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut coupon_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CouponCode => {
                            if coupon_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("couponCode"));
                            }
                            coupon_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ApplyCouponToOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                    coupon_code: coupon_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.ApplyCouponToOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplyPromotionToOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.promotion_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.ApplyPromotionToOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.promotion_id.is_empty() {
            struct_ser.serialize_field("promotionId", &self.promotion_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplyPromotionToOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "promotion_id",
            "promotionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            PromotionId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "promotionId" | "promotion_id" => Ok(GeneratedField::PromotionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplyPromotionToOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.ApplyPromotionToOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApplyPromotionToOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut promotion_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PromotionId => {
                            if promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionId"));
                            }
                            promotion_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ApplyPromotionToOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                    promotion_id: promotion_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.ApplyPromotionToOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrderItemInput {
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
        if self.variant_id.is_some() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.suggested_unit_price.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.CreateOrderItemInput", len)?;
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.suggested_unit_price.as_ref() {
            struct_ser.serialize_field("suggestedUnitPrice", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrderItemInput {
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
            "unit_id",
            "unitId",
            "suggested_unit_price",
            "suggestedUnitPrice",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProductId,
            VariantId,
            Quantity,
            UnitId,
            SuggestedUnitPrice,
            Notes,
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
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "suggestedUnitPrice" | "suggested_unit_price" => Ok(GeneratedField::SuggestedUnitPrice),
                            "notes" => Ok(GeneratedField::Notes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOrderItemInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.CreateOrderItemInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrderItemInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut quantity__ = None;
                let mut unit_id__ = None;
                let mut suggested_unit_price__ = None;
                let mut notes__ = None;
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
                            variant_id__ = map_.next_value()?;
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
                        GeneratedField::SuggestedUnitPrice => {
                            if suggested_unit_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suggestedUnitPrice"));
                            }
                            suggested_unit_price__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOrderItemInput {
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    quantity: quantity__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    suggested_unit_price: suggested_unit_price__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.CreateOrderItemInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.customer_id.is_some() {
            len += 1;
        }
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.fulfillment_warehouse_id.is_some() {
            len += 1;
        }
        if self.point_of_sale_branch_id.is_some() {
            len += 1;
        }
        if !self.currency_code.is_empty() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.initial_order_status.is_some() {
            len += 1;
        }
        if self.order_session_id.is_some() {
            len += 1;
        }
        if self.reservation_id.is_some() {
            len += 1;
        }
        if self.hotel_service_zone_id.is_some() {
            len += 1;
        }
        if self.delivery_room_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.CreateOrderRequest", len)?;
        if let Some(v) = self.customer_id.as_ref() {
            struct_ser.serialize_field("customerId", v)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if let Some(v) = self.fulfillment_warehouse_id.as_ref() {
            struct_ser.serialize_field("fulfillmentWarehouseId", v)?;
        }
        if let Some(v) = self.point_of_sale_branch_id.as_ref() {
            struct_ser.serialize_field("pointOfSaleBranchId", v)?;
        }
        if !self.currency_code.is_empty() {
            struct_ser.serialize_field("currencyCode", &self.currency_code)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.initial_order_status.as_ref() {
            struct_ser.serialize_field("initialOrderStatus", v)?;
        }
        if let Some(v) = self.order_session_id.as_ref() {
            struct_ser.serialize_field("orderSessionId", v)?;
        }
        if let Some(v) = self.reservation_id.as_ref() {
            struct_ser.serialize_field("reservationId", v)?;
        }
        if let Some(v) = self.hotel_service_zone_id.as_ref() {
            struct_ser.serialize_field("hotelServiceZoneId", v)?;
        }
        if let Some(v) = self.delivery_room_id.as_ref() {
            struct_ser.serialize_field("deliveryRoomId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer_id",
            "customerId",
            "sales_channel_id",
            "salesChannelId",
            "fulfillment_warehouse_id",
            "fulfillmentWarehouseId",
            "point_of_sale_branch_id",
            "pointOfSaleBranchId",
            "currency_code",
            "currencyCode",
            "items",
            "notes",
            "initial_order_status",
            "initialOrderStatus",
            "order_session_id",
            "orderSessionId",
            "reservation_id",
            "reservationId",
            "hotel_service_zone_id",
            "hotelServiceZoneId",
            "delivery_room_id",
            "deliveryRoomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomerId,
            SalesChannelId,
            FulfillmentWarehouseId,
            PointOfSaleBranchId,
            CurrencyCode,
            Items,
            Notes,
            InitialOrderStatus,
            OrderSessionId,
            ReservationId,
            HotelServiceZoneId,
            DeliveryRoomId,
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
                            "customerId" | "customer_id" => Ok(GeneratedField::CustomerId),
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "fulfillmentWarehouseId" | "fulfillment_warehouse_id" => Ok(GeneratedField::FulfillmentWarehouseId),
                            "pointOfSaleBranchId" | "point_of_sale_branch_id" => Ok(GeneratedField::PointOfSaleBranchId),
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            "items" => Ok(GeneratedField::Items),
                            "notes" => Ok(GeneratedField::Notes),
                            "initialOrderStatus" | "initial_order_status" => Ok(GeneratedField::InitialOrderStatus),
                            "orderSessionId" | "order_session_id" => Ok(GeneratedField::OrderSessionId),
                            "reservationId" | "reservation_id" => Ok(GeneratedField::ReservationId),
                            "hotelServiceZoneId" | "hotel_service_zone_id" => Ok(GeneratedField::HotelServiceZoneId),
                            "deliveryRoomId" | "delivery_room_id" => Ok(GeneratedField::DeliveryRoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.CreateOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer_id__ = None;
                let mut sales_channel_id__ = None;
                let mut fulfillment_warehouse_id__ = None;
                let mut point_of_sale_branch_id__ = None;
                let mut currency_code__ = None;
                let mut items__ = None;
                let mut notes__ = None;
                let mut initial_order_status__ = None;
                let mut order_session_id__ = None;
                let mut reservation_id__ = None;
                let mut hotel_service_zone_id__ = None;
                let mut delivery_room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FulfillmentWarehouseId => {
                            if fulfillment_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fulfillmentWarehouseId"));
                            }
                            fulfillment_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::PointOfSaleBranchId => {
                            if point_of_sale_branch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pointOfSaleBranchId"));
                            }
                            point_of_sale_branch_id__ = map_.next_value()?;
                        }
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::InitialOrderStatus => {
                            if initial_order_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialOrderStatus"));
                            }
                            initial_order_status__ = map_.next_value()?;
                        }
                        GeneratedField::OrderSessionId => {
                            if order_session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderSessionId"));
                            }
                            order_session_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReservationId => {
                            if reservation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationId"));
                            }
                            reservation_id__ = map_.next_value()?;
                        }
                        GeneratedField::HotelServiceZoneId => {
                            if hotel_service_zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hotelServiceZoneId"));
                            }
                            hotel_service_zone_id__ = map_.next_value()?;
                        }
                        GeneratedField::DeliveryRoomId => {
                            if delivery_room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliveryRoomId"));
                            }
                            delivery_room_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOrderRequest {
                    customer_id: customer_id__,
                    sales_channel_id: sales_channel_id__,
                    fulfillment_warehouse_id: fulfillment_warehouse_id__,
                    point_of_sale_branch_id: point_of_sale_branch_id__,
                    currency_code: currency_code__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                    notes: notes__,
                    initial_order_status: initial_order_status__,
                    order_session_id: order_session_id__,
                    reservation_id: reservation_id__,
                    hotel_service_zone_id: hotel_service_zone_id__,
                    delivery_room_id: delivery_room_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.CreateOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.GetOrderResponse", len)?;
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Order,
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
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.GetOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOrderResponse {
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.GetOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrdersRequest {
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
        if self.filter_by_customer_id.is_some() {
            len += 1;
        }
        if self.filter_by_order_status.is_some() {
            len += 1;
        }
        if self.filter_by_payment_status.is_some() {
            len += 1;
        }
        if self.filter_by_sales_channel_id.is_some() {
            len += 1;
        }
        if self.filter_by_fulfillment_warehouse_id.is_some() {
            len += 1;
        }
        if self.filter_by_ordered_date_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.ListOrdersRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_customer_id.as_ref() {
            struct_ser.serialize_field("filterByCustomerId", v)?;
        }
        if let Some(v) = self.filter_by_order_status.as_ref() {
            struct_ser.serialize_field("filterByOrderStatus", v)?;
        }
        if let Some(v) = self.filter_by_payment_status.as_ref() {
            struct_ser.serialize_field("filterByPaymentStatus", v)?;
        }
        if let Some(v) = self.filter_by_sales_channel_id.as_ref() {
            struct_ser.serialize_field("filterBySalesChannelId", v)?;
        }
        if let Some(v) = self.filter_by_fulfillment_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByFulfillmentWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_ordered_date_range.as_ref() {
            struct_ser.serialize_field("filterByOrderedDateRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrdersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_customer_id",
            "filterByCustomerId",
            "filter_by_order_status",
            "filterByOrderStatus",
            "filter_by_payment_status",
            "filterByPaymentStatus",
            "filter_by_sales_channel_id",
            "filterBySalesChannelId",
            "filter_by_fulfillment_warehouse_id",
            "filterByFulfillmentWarehouseId",
            "filter_by_ordered_date_range",
            "filterByOrderedDateRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByCustomerId,
            FilterByOrderStatus,
            FilterByPaymentStatus,
            FilterBySalesChannelId,
            FilterByFulfillmentWarehouseId,
            FilterByOrderedDateRange,
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
                            "filterByCustomerId" | "filter_by_customer_id" => Ok(GeneratedField::FilterByCustomerId),
                            "filterByOrderStatus" | "filter_by_order_status" => Ok(GeneratedField::FilterByOrderStatus),
                            "filterByPaymentStatus" | "filter_by_payment_status" => Ok(GeneratedField::FilterByPaymentStatus),
                            "filterBySalesChannelId" | "filter_by_sales_channel_id" => Ok(GeneratedField::FilterBySalesChannelId),
                            "filterByFulfillmentWarehouseId" | "filter_by_fulfillment_warehouse_id" => Ok(GeneratedField::FilterByFulfillmentWarehouseId),
                            "filterByOrderedDateRange" | "filter_by_ordered_date_range" => Ok(GeneratedField::FilterByOrderedDateRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrdersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.ListOrdersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrdersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_customer_id__ = None;
                let mut filter_by_order_status__ = None;
                let mut filter_by_payment_status__ = None;
                let mut filter_by_sales_channel_id__ = None;
                let mut filter_by_fulfillment_warehouse_id__ = None;
                let mut filter_by_ordered_date_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByCustomerId => {
                            if filter_by_customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByCustomerId"));
                            }
                            filter_by_customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByOrderStatus => {
                            if filter_by_order_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByOrderStatus"));
                            }
                            filter_by_order_status__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByPaymentStatus => {
                            if filter_by_payment_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByPaymentStatus"));
                            }
                            filter_by_payment_status__ = map_.next_value()?;
                        }
                        GeneratedField::FilterBySalesChannelId => {
                            if filter_by_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterBySalesChannelId"));
                            }
                            filter_by_sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByFulfillmentWarehouseId => {
                            if filter_by_fulfillment_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByFulfillmentWarehouseId"));
                            }
                            filter_by_fulfillment_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByOrderedDateRange => {
                            if filter_by_ordered_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByOrderedDateRange"));
                            }
                            filter_by_ordered_date_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOrdersRequest {
                    base_request: base_request__,
                    filter_by_customer_id: filter_by_customer_id__,
                    filter_by_order_status: filter_by_order_status__,
                    filter_by_payment_status: filter_by_payment_status__,
                    filter_by_sales_channel_id: filter_by_sales_channel_id__,
                    filter_by_fulfillment_warehouse_id: filter_by_fulfillment_warehouse_id__,
                    filter_by_ordered_date_range: filter_by_ordered_date_range__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.ListOrdersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrdersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orders.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.ListOrdersResponse", len)?;
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrdersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orders",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orders,
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
                            "orders" => Ok(GeneratedField::Orders),
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
            type Value = ListOrdersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.ListOrdersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrdersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orders__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOrdersResponse {
                    orders: orders__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.ListOrdersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Order {
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
        if !self.order_number.is_empty() {
            len += 1;
        }
        if self.customer_id.is_some() {
            len += 1;
        }
        if self.customer_details.is_some() {
            len += 1;
        }
        if self.sales_channel_id.is_some() {
            len += 1;
        }
        if self.fulfillment_warehouse_id.is_some() {
            len += 1;
        }
        if self.fulfillment_warehouse_details.is_some() {
            len += 1;
        }
        if self.point_of_sale_branch_id.is_some() {
            len += 1;
        }
        if self.point_of_sale_branch_details.is_some() {
            len += 1;
        }
        if !self.order_status.is_empty() {
            len += 1;
        }
        if !self.payment_status.is_empty() {
            len += 1;
        }
        if self.shipping_status.is_some() {
            len += 1;
        }
        if self.sub_total_amount.is_some() {
            len += 1;
        }
        if self.discount_total_amount.is_some() {
            len += 1;
        }
        if self.tax_total_amount.is_some() {
            len += 1;
        }
        if self.shipping_fee_amount.is_some() {
            len += 1;
        }
        if self.grand_total_amount.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.ordered_at.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        if !self.applied_promotions.is_empty() {
            len += 1;
        }
        if !self.payments_recorded.is_empty() {
            len += 1;
        }
        if self.order_session_id.is_some() {
            len += 1;
        }
        if self.order_session_details.is_some() {
            len += 1;
        }
        if self.reservation_id.is_some() {
            len += 1;
        }
        if self.reservation_details.is_some() {
            len += 1;
        }
        if self.hotel_service_zone_id.is_some() {
            len += 1;
        }
        if self.hotel_service_zone_details.is_some() {
            len += 1;
        }
        if self.delivery_room_id.is_some() {
            len += 1;
        }
        if self.delivery_room_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.Order", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.order_number.is_empty() {
            struct_ser.serialize_field("orderNumber", &self.order_number)?;
        }
        if let Some(v) = self.customer_id.as_ref() {
            struct_ser.serialize_field("customerId", v)?;
        }
        if let Some(v) = self.customer_details.as_ref() {
            struct_ser.serialize_field("customerDetails", v)?;
        }
        if let Some(v) = self.sales_channel_id.as_ref() {
            struct_ser.serialize_field("salesChannelId", v)?;
        }
        if let Some(v) = self.fulfillment_warehouse_id.as_ref() {
            struct_ser.serialize_field("fulfillmentWarehouseId", v)?;
        }
        if let Some(v) = self.fulfillment_warehouse_details.as_ref() {
            struct_ser.serialize_field("fulfillmentWarehouseDetails", v)?;
        }
        if let Some(v) = self.point_of_sale_branch_id.as_ref() {
            struct_ser.serialize_field("pointOfSaleBranchId", v)?;
        }
        if let Some(v) = self.point_of_sale_branch_details.as_ref() {
            struct_ser.serialize_field("pointOfSaleBranchDetails", v)?;
        }
        if !self.order_status.is_empty() {
            struct_ser.serialize_field("orderStatus", &self.order_status)?;
        }
        if !self.payment_status.is_empty() {
            struct_ser.serialize_field("paymentStatus", &self.payment_status)?;
        }
        if let Some(v) = self.shipping_status.as_ref() {
            struct_ser.serialize_field("shippingStatus", v)?;
        }
        if let Some(v) = self.sub_total_amount.as_ref() {
            struct_ser.serialize_field("subTotalAmount", v)?;
        }
        if let Some(v) = self.discount_total_amount.as_ref() {
            struct_ser.serialize_field("discountTotalAmount", v)?;
        }
        if let Some(v) = self.tax_total_amount.as_ref() {
            struct_ser.serialize_field("taxTotalAmount", v)?;
        }
        if let Some(v) = self.shipping_fee_amount.as_ref() {
            struct_ser.serialize_field("shippingFeeAmount", v)?;
        }
        if let Some(v) = self.grand_total_amount.as_ref() {
            struct_ser.serialize_field("grandTotalAmount", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.ordered_at.as_ref() {
            struct_ser.serialize_field("orderedAt", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if !self.applied_promotions.is_empty() {
            struct_ser.serialize_field("appliedPromotions", &self.applied_promotions)?;
        }
        if !self.payments_recorded.is_empty() {
            struct_ser.serialize_field("paymentsRecorded", &self.payments_recorded)?;
        }
        if let Some(v) = self.order_session_id.as_ref() {
            struct_ser.serialize_field("orderSessionId", v)?;
        }
        if let Some(v) = self.order_session_details.as_ref() {
            struct_ser.serialize_field("orderSessionDetails", v)?;
        }
        if let Some(v) = self.reservation_id.as_ref() {
            struct_ser.serialize_field("reservationId", v)?;
        }
        if let Some(v) = self.reservation_details.as_ref() {
            struct_ser.serialize_field("reservationDetails", v)?;
        }
        if let Some(v) = self.hotel_service_zone_id.as_ref() {
            struct_ser.serialize_field("hotelServiceZoneId", v)?;
        }
        if let Some(v) = self.hotel_service_zone_details.as_ref() {
            struct_ser.serialize_field("hotelServiceZoneDetails", v)?;
        }
        if let Some(v) = self.delivery_room_id.as_ref() {
            struct_ser.serialize_field("deliveryRoomId", v)?;
        }
        if let Some(v) = self.delivery_room_details.as_ref() {
            struct_ser.serialize_field("deliveryRoomDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "order_number",
            "orderNumber",
            "customer_id",
            "customerId",
            "customer_details",
            "customerDetails",
            "sales_channel_id",
            "salesChannelId",
            "fulfillment_warehouse_id",
            "fulfillmentWarehouseId",
            "fulfillment_warehouse_details",
            "fulfillmentWarehouseDetails",
            "point_of_sale_branch_id",
            "pointOfSaleBranchId",
            "point_of_sale_branch_details",
            "pointOfSaleBranchDetails",
            "order_status",
            "orderStatus",
            "payment_status",
            "paymentStatus",
            "shipping_status",
            "shippingStatus",
            "sub_total_amount",
            "subTotalAmount",
            "discount_total_amount",
            "discountTotalAmount",
            "tax_total_amount",
            "taxTotalAmount",
            "shipping_fee_amount",
            "shippingFeeAmount",
            "grand_total_amount",
            "grandTotalAmount",
            "notes",
            "ordered_at",
            "orderedAt",
            "created_by_user_id",
            "createdByUserId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "items",
            "applied_promotions",
            "appliedPromotions",
            "payments_recorded",
            "paymentsRecorded",
            "order_session_id",
            "orderSessionId",
            "order_session_details",
            "orderSessionDetails",
            "reservation_id",
            "reservationId",
            "reservation_details",
            "reservationDetails",
            "hotel_service_zone_id",
            "hotelServiceZoneId",
            "hotel_service_zone_details",
            "hotelServiceZoneDetails",
            "delivery_room_id",
            "deliveryRoomId",
            "delivery_room_details",
            "deliveryRoomDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            OrderNumber,
            CustomerId,
            CustomerDetails,
            SalesChannelId,
            FulfillmentWarehouseId,
            FulfillmentWarehouseDetails,
            PointOfSaleBranchId,
            PointOfSaleBranchDetails,
            OrderStatus,
            PaymentStatus,
            ShippingStatus,
            SubTotalAmount,
            DiscountTotalAmount,
            TaxTotalAmount,
            ShippingFeeAmount,
            GrandTotalAmount,
            Notes,
            OrderedAt,
            CreatedByUserId,
            CreatedAt,
            UpdatedAt,
            Items,
            AppliedPromotions,
            PaymentsRecorded,
            OrderSessionId,
            OrderSessionDetails,
            ReservationId,
            ReservationDetails,
            HotelServiceZoneId,
            HotelServiceZoneDetails,
            DeliveryRoomId,
            DeliveryRoomDetails,
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
                            "orderNumber" | "order_number" => Ok(GeneratedField::OrderNumber),
                            "customerId" | "customer_id" => Ok(GeneratedField::CustomerId),
                            "customerDetails" | "customer_details" => Ok(GeneratedField::CustomerDetails),
                            "salesChannelId" | "sales_channel_id" => Ok(GeneratedField::SalesChannelId),
                            "fulfillmentWarehouseId" | "fulfillment_warehouse_id" => Ok(GeneratedField::FulfillmentWarehouseId),
                            "fulfillmentWarehouseDetails" | "fulfillment_warehouse_details" => Ok(GeneratedField::FulfillmentWarehouseDetails),
                            "pointOfSaleBranchId" | "point_of_sale_branch_id" => Ok(GeneratedField::PointOfSaleBranchId),
                            "pointOfSaleBranchDetails" | "point_of_sale_branch_details" => Ok(GeneratedField::PointOfSaleBranchDetails),
                            "orderStatus" | "order_status" => Ok(GeneratedField::OrderStatus),
                            "paymentStatus" | "payment_status" => Ok(GeneratedField::PaymentStatus),
                            "shippingStatus" | "shipping_status" => Ok(GeneratedField::ShippingStatus),
                            "subTotalAmount" | "sub_total_amount" => Ok(GeneratedField::SubTotalAmount),
                            "discountTotalAmount" | "discount_total_amount" => Ok(GeneratedField::DiscountTotalAmount),
                            "taxTotalAmount" | "tax_total_amount" => Ok(GeneratedField::TaxTotalAmount),
                            "shippingFeeAmount" | "shipping_fee_amount" => Ok(GeneratedField::ShippingFeeAmount),
                            "grandTotalAmount" | "grand_total_amount" => Ok(GeneratedField::GrandTotalAmount),
                            "notes" => Ok(GeneratedField::Notes),
                            "orderedAt" | "ordered_at" => Ok(GeneratedField::OrderedAt),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "items" => Ok(GeneratedField::Items),
                            "appliedPromotions" | "applied_promotions" => Ok(GeneratedField::AppliedPromotions),
                            "paymentsRecorded" | "payments_recorded" => Ok(GeneratedField::PaymentsRecorded),
                            "orderSessionId" | "order_session_id" => Ok(GeneratedField::OrderSessionId),
                            "orderSessionDetails" | "order_session_details" => Ok(GeneratedField::OrderSessionDetails),
                            "reservationId" | "reservation_id" => Ok(GeneratedField::ReservationId),
                            "reservationDetails" | "reservation_details" => Ok(GeneratedField::ReservationDetails),
                            "hotelServiceZoneId" | "hotel_service_zone_id" => Ok(GeneratedField::HotelServiceZoneId),
                            "hotelServiceZoneDetails" | "hotel_service_zone_details" => Ok(GeneratedField::HotelServiceZoneDetails),
                            "deliveryRoomId" | "delivery_room_id" => Ok(GeneratedField::DeliveryRoomId),
                            "deliveryRoomDetails" | "delivery_room_details" => Ok(GeneratedField::DeliveryRoomDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Order;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.Order")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Order, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut order_number__ = None;
                let mut customer_id__ = None;
                let mut customer_details__ = None;
                let mut sales_channel_id__ = None;
                let mut fulfillment_warehouse_id__ = None;
                let mut fulfillment_warehouse_details__ = None;
                let mut point_of_sale_branch_id__ = None;
                let mut point_of_sale_branch_details__ = None;
                let mut order_status__ = None;
                let mut payment_status__ = None;
                let mut shipping_status__ = None;
                let mut sub_total_amount__ = None;
                let mut discount_total_amount__ = None;
                let mut tax_total_amount__ = None;
                let mut shipping_fee_amount__ = None;
                let mut grand_total_amount__ = None;
                let mut notes__ = None;
                let mut ordered_at__ = None;
                let mut created_by_user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut items__ = None;
                let mut applied_promotions__ = None;
                let mut payments_recorded__ = None;
                let mut order_session_id__ = None;
                let mut order_session_details__ = None;
                let mut reservation_id__ = None;
                let mut reservation_details__ = None;
                let mut hotel_service_zone_id__ = None;
                let mut hotel_service_zone_details__ = None;
                let mut delivery_room_id__ = None;
                let mut delivery_room_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderNumber => {
                            if order_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderNumber"));
                            }
                            order_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::CustomerDetails => {
                            if customer_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerDetails"));
                            }
                            customer_details__ = map_.next_value()?;
                        }
                        GeneratedField::SalesChannelId => {
                            if sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salesChannelId"));
                            }
                            sales_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::FulfillmentWarehouseId => {
                            if fulfillment_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fulfillmentWarehouseId"));
                            }
                            fulfillment_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::FulfillmentWarehouseDetails => {
                            if fulfillment_warehouse_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fulfillmentWarehouseDetails"));
                            }
                            fulfillment_warehouse_details__ = map_.next_value()?;
                        }
                        GeneratedField::PointOfSaleBranchId => {
                            if point_of_sale_branch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pointOfSaleBranchId"));
                            }
                            point_of_sale_branch_id__ = map_.next_value()?;
                        }
                        GeneratedField::PointOfSaleBranchDetails => {
                            if point_of_sale_branch_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pointOfSaleBranchDetails"));
                            }
                            point_of_sale_branch_details__ = map_.next_value()?;
                        }
                        GeneratedField::OrderStatus => {
                            if order_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderStatus"));
                            }
                            order_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentStatus => {
                            if payment_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentStatus"));
                            }
                            payment_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShippingStatus => {
                            if shipping_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shippingStatus"));
                            }
                            shipping_status__ = map_.next_value()?;
                        }
                        GeneratedField::SubTotalAmount => {
                            if sub_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subTotalAmount"));
                            }
                            sub_total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::DiscountTotalAmount => {
                            if discount_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discountTotalAmount"));
                            }
                            discount_total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::TaxTotalAmount => {
                            if tax_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxTotalAmount"));
                            }
                            tax_total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::ShippingFeeAmount => {
                            if shipping_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shippingFeeAmount"));
                            }
                            shipping_fee_amount__ = map_.next_value()?;
                        }
                        GeneratedField::GrandTotalAmount => {
                            if grand_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grandTotalAmount"));
                            }
                            grand_total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::OrderedAt => {
                            if ordered_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderedAt"));
                            }
                            ordered_at__ = map_.next_value()?;
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
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppliedPromotions => {
                            if applied_promotions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appliedPromotions"));
                            }
                            applied_promotions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentsRecorded => {
                            if payments_recorded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentsRecorded"));
                            }
                            payments_recorded__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderSessionId => {
                            if order_session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderSessionId"));
                            }
                            order_session_id__ = map_.next_value()?;
                        }
                        GeneratedField::OrderSessionDetails => {
                            if order_session_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderSessionDetails"));
                            }
                            order_session_details__ = map_.next_value()?;
                        }
                        GeneratedField::ReservationId => {
                            if reservation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationId"));
                            }
                            reservation_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReservationDetails => {
                            if reservation_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationDetails"));
                            }
                            reservation_details__ = map_.next_value()?;
                        }
                        GeneratedField::HotelServiceZoneId => {
                            if hotel_service_zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hotelServiceZoneId"));
                            }
                            hotel_service_zone_id__ = map_.next_value()?;
                        }
                        GeneratedField::HotelServiceZoneDetails => {
                            if hotel_service_zone_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hotelServiceZoneDetails"));
                            }
                            hotel_service_zone_details__ = map_.next_value()?;
                        }
                        GeneratedField::DeliveryRoomId => {
                            if delivery_room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliveryRoomId"));
                            }
                            delivery_room_id__ = map_.next_value()?;
                        }
                        GeneratedField::DeliveryRoomDetails => {
                            if delivery_room_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliveryRoomDetails"));
                            }
                            delivery_room_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Order {
                    id: id__.unwrap_or_default(),
                    order_number: order_number__.unwrap_or_default(),
                    customer_id: customer_id__,
                    customer_details: customer_details__,
                    sales_channel_id: sales_channel_id__,
                    fulfillment_warehouse_id: fulfillment_warehouse_id__,
                    fulfillment_warehouse_details: fulfillment_warehouse_details__,
                    point_of_sale_branch_id: point_of_sale_branch_id__,
                    point_of_sale_branch_details: point_of_sale_branch_details__,
                    order_status: order_status__.unwrap_or_default(),
                    payment_status: payment_status__.unwrap_or_default(),
                    shipping_status: shipping_status__,
                    sub_total_amount: sub_total_amount__,
                    discount_total_amount: discount_total_amount__,
                    tax_total_amount: tax_total_amount__,
                    shipping_fee_amount: shipping_fee_amount__,
                    grand_total_amount: grand_total_amount__,
                    notes: notes__,
                    ordered_at: ordered_at__,
                    created_by_user_id: created_by_user_id__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    items: items__.unwrap_or_default(),
                    applied_promotions: applied_promotions__.unwrap_or_default(),
                    payments_recorded: payments_recorded__.unwrap_or_default(),
                    order_session_id: order_session_id__,
                    order_session_details: order_session_details__,
                    reservation_id: reservation_id__,
                    reservation_details: reservation_details__,
                    hotel_service_zone_id: hotel_service_zone_id__,
                    hotel_service_zone_details: hotel_service_zone_details__,
                    delivery_room_id: delivery_room_id__,
                    delivery_room_details: delivery_room_details__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.Order", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderAppliedPromotion {
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
        if !self.promotion_id.is_empty() {
            len += 1;
        }
        if self.coupon_code_used.is_some() {
            len += 1;
        }
        if self.discount_amount_applied.is_some() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.promotion_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.OrderAppliedPromotion", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.promotion_id.is_empty() {
            struct_ser.serialize_field("promotionId", &self.promotion_id)?;
        }
        if let Some(v) = self.coupon_code_used.as_ref() {
            struct_ser.serialize_field("couponCodeUsed", v)?;
        }
        if let Some(v) = self.discount_amount_applied.as_ref() {
            struct_ser.serialize_field("discountAmountApplied", v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.promotion_details.as_ref() {
            struct_ser.serialize_field("promotionDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderAppliedPromotion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "promotion_id",
            "promotionId",
            "coupon_code_used",
            "couponCodeUsed",
            "discount_amount_applied",
            "discountAmountApplied",
            "description",
            "promotion_details",
            "promotionDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            PromotionId,
            CouponCodeUsed,
            DiscountAmountApplied,
            Description,
            PromotionDetails,
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
                            "promotionId" | "promotion_id" => Ok(GeneratedField::PromotionId),
                            "couponCodeUsed" | "coupon_code_used" => Ok(GeneratedField::CouponCodeUsed),
                            "discountAmountApplied" | "discount_amount_applied" => Ok(GeneratedField::DiscountAmountApplied),
                            "description" => Ok(GeneratedField::Description),
                            "promotionDetails" | "promotion_details" => Ok(GeneratedField::PromotionDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderAppliedPromotion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.OrderAppliedPromotion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderAppliedPromotion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut promotion_id__ = None;
                let mut coupon_code_used__ = None;
                let mut discount_amount_applied__ = None;
                let mut description__ = None;
                let mut promotion_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PromotionId => {
                            if promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionId"));
                            }
                            promotion_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CouponCodeUsed => {
                            if coupon_code_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("couponCodeUsed"));
                            }
                            coupon_code_used__ = map_.next_value()?;
                        }
                        GeneratedField::DiscountAmountApplied => {
                            if discount_amount_applied__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discountAmountApplied"));
                            }
                            discount_amount_applied__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PromotionDetails => {
                            if promotion_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionDetails"));
                            }
                            promotion_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrderAppliedPromotion {
                    id: id__.unwrap_or_default(),
                    promotion_id: promotion_id__.unwrap_or_default(),
                    coupon_code_used: coupon_code_used__,
                    discount_amount_applied: discount_amount_applied__,
                    description: description__.unwrap_or_default(),
                    promotion_details: promotion_details__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.OrderAppliedPromotion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderItem {
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
        if self.product_snapshot.is_some() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.unit_price.is_some() {
            len += 1;
        }
        if self.line_sub_total_amount.is_some() {
            len += 1;
        }
        if self.line_discount_amount.is_some() {
            len += 1;
        }
        if self.line_tax_amount.is_some() {
            len += 1;
        }
        if self.line_total_amount.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
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
        if !self.applied_promotions_to_item.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.OrderItem", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.product_id.is_empty() {
            struct_ser.serialize_field("productId", &self.product_id)?;
        }
        if let Some(v) = self.variant_id.as_ref() {
            struct_ser.serialize_field("variantId", v)?;
        }
        if let Some(v) = self.product_snapshot.as_ref() {
            struct_ser.serialize_field("productSnapshot", v)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.unit_price.as_ref() {
            struct_ser.serialize_field("unitPrice", v)?;
        }
        if let Some(v) = self.line_sub_total_amount.as_ref() {
            struct_ser.serialize_field("lineSubTotalAmount", v)?;
        }
        if let Some(v) = self.line_discount_amount.as_ref() {
            struct_ser.serialize_field("lineDiscountAmount", v)?;
        }
        if let Some(v) = self.line_tax_amount.as_ref() {
            struct_ser.serialize_field("lineTaxAmount", v)?;
        }
        if let Some(v) = self.line_total_amount.as_ref() {
            struct_ser.serialize_field("lineTotalAmount", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
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
        if !self.applied_promotions_to_item.is_empty() {
            struct_ser.serialize_field("appliedPromotionsToItem", &self.applied_promotions_to_item)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderItem {
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
            "product_snapshot",
            "productSnapshot",
            "quantity",
            "unit_id",
            "unitId",
            "unit_price",
            "unitPrice",
            "line_sub_total_amount",
            "lineSubTotalAmount",
            "line_discount_amount",
            "lineDiscountAmount",
            "line_tax_amount",
            "lineTaxAmount",
            "line_total_amount",
            "lineTotalAmount",
            "notes",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "product_summary",
            "productSummary",
            "variant_summary",
            "variantSummary",
            "unit_summary",
            "unitSummary",
            "applied_promotions_to_item",
            "appliedPromotionsToItem",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProductId,
            VariantId,
            ProductSnapshot,
            Quantity,
            UnitId,
            UnitPrice,
            LineSubTotalAmount,
            LineDiscountAmount,
            LineTaxAmount,
            LineTotalAmount,
            Notes,
            CreatedAt,
            UpdatedAt,
            ProductSummary,
            VariantSummary,
            UnitSummary,
            AppliedPromotionsToItem,
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
                            "productSnapshot" | "product_snapshot" => Ok(GeneratedField::ProductSnapshot),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "unitPrice" | "unit_price" => Ok(GeneratedField::UnitPrice),
                            "lineSubTotalAmount" | "line_sub_total_amount" => Ok(GeneratedField::LineSubTotalAmount),
                            "lineDiscountAmount" | "line_discount_amount" => Ok(GeneratedField::LineDiscountAmount),
                            "lineTaxAmount" | "line_tax_amount" => Ok(GeneratedField::LineTaxAmount),
                            "lineTotalAmount" | "line_total_amount" => Ok(GeneratedField::LineTotalAmount),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "productSummary" | "product_summary" => Ok(GeneratedField::ProductSummary),
                            "variantSummary" | "variant_summary" => Ok(GeneratedField::VariantSummary),
                            "unitSummary" | "unit_summary" => Ok(GeneratedField::UnitSummary),
                            "appliedPromotionsToItem" | "applied_promotions_to_item" => Ok(GeneratedField::AppliedPromotionsToItem),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.OrderItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_id__ = None;
                let mut variant_id__ = None;
                let mut product_snapshot__ = None;
                let mut quantity__ = None;
                let mut unit_id__ = None;
                let mut unit_price__ = None;
                let mut line_sub_total_amount__ = None;
                let mut line_discount_amount__ = None;
                let mut line_tax_amount__ = None;
                let mut line_total_amount__ = None;
                let mut notes__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut product_summary__ = None;
                let mut variant_summary__ = None;
                let mut unit_summary__ = None;
                let mut applied_promotions_to_item__ = None;
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
                        GeneratedField::ProductSnapshot => {
                            if product_snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productSnapshot"));
                            }
                            product_snapshot__ = map_.next_value()?;
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
                        GeneratedField::UnitPrice => {
                            if unit_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            unit_price__ = map_.next_value()?;
                        }
                        GeneratedField::LineSubTotalAmount => {
                            if line_sub_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineSubTotalAmount"));
                            }
                            line_sub_total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LineDiscountAmount => {
                            if line_discount_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineDiscountAmount"));
                            }
                            line_discount_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LineTaxAmount => {
                            if line_tax_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineTaxAmount"));
                            }
                            line_tax_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LineTotalAmount => {
                            if line_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineTotalAmount"));
                            }
                            line_total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
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
                        GeneratedField::AppliedPromotionsToItem => {
                            if applied_promotions_to_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appliedPromotionsToItem"));
                            }
                            applied_promotions_to_item__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrderItem {
                    id: id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    variant_id: variant_id__,
                    product_snapshot: product_snapshot__,
                    quantity: quantity__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    unit_price: unit_price__,
                    line_sub_total_amount: line_sub_total_amount__,
                    line_discount_amount: line_discount_amount__,
                    line_tax_amount: line_tax_amount__,
                    line_total_amount: line_total_amount__,
                    notes: notes__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    product_summary: product_summary__,
                    variant_summary: variant_summary__,
                    unit_summary: unit_summary__,
                    applied_promotions_to_item: applied_promotions_to_item__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.OrderItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderItemAppliedPromotion {
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
        if !self.promotion_id.is_empty() {
            len += 1;
        }
        if self.discount_amount_applied.is_some() {
            len += 1;
        }
        if self.promotion_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.OrderItemAppliedPromotion", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.promotion_id.is_empty() {
            struct_ser.serialize_field("promotionId", &self.promotion_id)?;
        }
        if let Some(v) = self.discount_amount_applied.as_ref() {
            struct_ser.serialize_field("discountAmountApplied", v)?;
        }
        if let Some(v) = self.promotion_details.as_ref() {
            struct_ser.serialize_field("promotionDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderItemAppliedPromotion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "promotion_id",
            "promotionId",
            "discount_amount_applied",
            "discountAmountApplied",
            "promotion_details",
            "promotionDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            PromotionId,
            DiscountAmountApplied,
            PromotionDetails,
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
                            "promotionId" | "promotion_id" => Ok(GeneratedField::PromotionId),
                            "discountAmountApplied" | "discount_amount_applied" => Ok(GeneratedField::DiscountAmountApplied),
                            "promotionDetails" | "promotion_details" => Ok(GeneratedField::PromotionDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderItemAppliedPromotion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.OrderItemAppliedPromotion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderItemAppliedPromotion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut promotion_id__ = None;
                let mut discount_amount_applied__ = None;
                let mut promotion_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PromotionId => {
                            if promotion_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionId"));
                            }
                            promotion_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DiscountAmountApplied => {
                            if discount_amount_applied__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discountAmountApplied"));
                            }
                            discount_amount_applied__ = map_.next_value()?;
                        }
                        GeneratedField::PromotionDetails => {
                            if promotion_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotionDetails"));
                            }
                            promotion_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrderItemAppliedPromotion {
                    id: id__.unwrap_or_default(),
                    promotion_id: promotion_id__.unwrap_or_default(),
                    discount_amount_applied: discount_amount_applied__,
                    promotion_details: promotion_details__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.OrderItemAppliedPromotion", FIELDS, GeneratedVisitor)
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
        if !self.payment_method_id.is_empty() {
            len += 1;
        }
        if self.payment_method_details.is_some() {
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
        if self.paid_at.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.Payment", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.payment_method_id.is_empty() {
            struct_ser.serialize_field("paymentMethodId", &self.payment_method_id)?;
        }
        if let Some(v) = self.payment_method_details.as_ref() {
            struct_ser.serialize_field("paymentMethodDetails", v)?;
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
        if let Some(v) = self.paid_at.as_ref() {
            struct_ser.serialize_field("paidAt", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
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
            "payment_method_id",
            "paymentMethodId",
            "payment_method_details",
            "paymentMethodDetails",
            "amount",
            "status",
            "transaction_reference",
            "transactionReference",
            "paid_at",
            "paidAt",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            PaymentMethodId,
            PaymentMethodDetails,
            Amount,
            Status,
            TransactionReference,
            PaidAt,
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
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "paymentMethodDetails" | "payment_method_details" => Ok(GeneratedField::PaymentMethodDetails),
                            "amount" => Ok(GeneratedField::Amount),
                            "status" => Ok(GeneratedField::Status),
                            "transactionReference" | "transaction_reference" => Ok(GeneratedField::TransactionReference),
                            "paidAt" | "paid_at" => Ok(GeneratedField::PaidAt),
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
            type Value = Payment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.Payment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut payment_method_id__ = None;
                let mut payment_method_details__ = None;
                let mut amount__ = None;
                let mut status__ = None;
                let mut transaction_reference__ = None;
                let mut paid_at__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentMethodId => {
                            if payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodId"));
                            }
                            payment_method_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentMethodDetails => {
                            if payment_method_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodDetails"));
                            }
                            payment_method_details__ = map_.next_value()?;
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
                        GeneratedField::PaidAt => {
                            if paid_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paidAt"));
                            }
                            paid_at__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Payment {
                    id: id__.unwrap_or_default(),
                    payment_method_id: payment_method_id__.unwrap_or_default(),
                    payment_method_details: payment_method_details__,
                    amount: amount__,
                    status: status__.unwrap_or_default(),
                    transaction_reference: transaction_reference__,
                    paid_at: paid_at__,
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.Payment", FIELDS, GeneratedVisitor)
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.PaymentMethod", len)?;
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
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
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
            "name",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            Type,
            IsActive,
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
                formatter.write_str("struct projectsuit.order.v1.PaymentMethod")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PaymentMethod, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut r#type__ = None;
                let mut is_active__ = None;
                let mut name__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
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
                    name: name__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.PaymentMethod", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecordPaymentForOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.payment_method_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.transaction_reference.is_some() {
            len += 1;
        }
        if self.paid_at.is_some() {
            len += 1;
        }
        if self.status_override.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.RecordPaymentForOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.payment_method_id.is_empty() {
            struct_ser.serialize_field("paymentMethodId", &self.payment_method_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.transaction_reference.as_ref() {
            struct_ser.serialize_field("transactionReference", v)?;
        }
        if let Some(v) = self.paid_at.as_ref() {
            struct_ser.serialize_field("paidAt", v)?;
        }
        if let Some(v) = self.status_override.as_ref() {
            struct_ser.serialize_field("statusOverride", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecordPaymentForOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "payment_method_id",
            "paymentMethodId",
            "amount",
            "transaction_reference",
            "transactionReference",
            "paid_at",
            "paidAt",
            "status_override",
            "statusOverride",
            "created_by_user_id",
            "createdByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            PaymentMethodId,
            Amount,
            TransactionReference,
            PaidAt,
            StatusOverride,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "amount" => Ok(GeneratedField::Amount),
                            "transactionReference" | "transaction_reference" => Ok(GeneratedField::TransactionReference),
                            "paidAt" | "paid_at" => Ok(GeneratedField::PaidAt),
                            "statusOverride" | "status_override" => Ok(GeneratedField::StatusOverride),
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
            type Value = RecordPaymentForOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.RecordPaymentForOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecordPaymentForOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut payment_method_id__ = None;
                let mut amount__ = None;
                let mut transaction_reference__ = None;
                let mut paid_at__ = None;
                let mut status_override__ = None;
                let mut created_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
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
                        GeneratedField::TransactionReference => {
                            if transaction_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionReference"));
                            }
                            transaction_reference__ = map_.next_value()?;
                        }
                        GeneratedField::PaidAt => {
                            if paid_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paidAt"));
                            }
                            paid_at__ = map_.next_value()?;
                        }
                        GeneratedField::StatusOverride => {
                            if status_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusOverride"));
                            }
                            status_override__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RecordPaymentForOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                    payment_method_id: payment_method_id__.unwrap_or_default(),
                    amount: amount__,
                    transaction_reference: transaction_reference__,
                    paid_at: paid_at__,
                    status_override: status_override__,
                    created_by_user_id: created_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.RecordPaymentForOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOrderItemRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.order_item_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.RemoveOrderItemRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.order_item_id.is_empty() {
            struct_ser.serialize_field("orderItemId", &self.order_item_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOrderItemRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "order_item_id",
            "orderItemId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OrderItemId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "orderItemId" | "order_item_id" => Ok(GeneratedField::OrderItemId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveOrderItemRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.RemoveOrderItemRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOrderItemRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut order_item_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderItemId => {
                            if order_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderItemId"));
                            }
                            order_item_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveOrderItemRequest {
                    order_id: order_id__.unwrap_or_default(),
                    order_item_id: order_item_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.RemoveOrderItemRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrderItemRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_item_id.is_empty() {
            len += 1;
        }
        if self.quantity.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.UpdateOrderItemRequest", len)?;
        if !self.order_item_id.is_empty() {
            struct_ser.serialize_field("orderItemId", &self.order_item_id)?;
        }
        if let Some(v) = self.quantity.as_ref() {
            struct_ser.serialize_field("quantity", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrderItemRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_item_id",
            "orderItemId",
            "quantity",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderItemId,
            Quantity,
            Notes,
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
                            "orderItemId" | "order_item_id" => Ok(GeneratedField::OrderItemId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "notes" => Ok(GeneratedField::Notes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOrderItemRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.UpdateOrderItemRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrderItemRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_item_id__ = None;
                let mut quantity__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderItemId => {
                            if order_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderItemId"));
                            }
                            order_item_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrderItemRequest {
                    order_item_id: order_item_id__.unwrap_or_default(),
                    quantity: quantity__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.UpdateOrderItemRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrderStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.new_status.is_empty() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.changed_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.order.v1.UpdateOrderStatusRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.new_status.is_empty() {
            struct_ser.serialize_field("newStatus", &self.new_status)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.changed_by_user_id.as_ref() {
            struct_ser.serialize_field("changedByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrderStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "new_status",
            "newStatus",
            "notes",
            "changed_by_user_id",
            "changedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            NewStatus,
            Notes,
            ChangedByUserId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "newStatus" | "new_status" => Ok(GeneratedField::NewStatus),
                            "notes" => Ok(GeneratedField::Notes),
                            "changedByUserId" | "changed_by_user_id" => Ok(GeneratedField::ChangedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOrderStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.order.v1.UpdateOrderStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrderStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut new_status__ = None;
                let mut notes__ = None;
                let mut changed_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewStatus => {
                            if new_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newStatus"));
                            }
                            new_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::ChangedByUserId => {
                            if changed_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changedByUserId"));
                            }
                            changed_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrderStatusRequest {
                    order_id: order_id__.unwrap_or_default(),
                    new_status: new_status__.unwrap_or_default(),
                    notes: notes__,
                    changed_by_user_id: changed_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.order.v1.UpdateOrderStatusRequest", FIELDS, GeneratedVisitor)
    }
}
