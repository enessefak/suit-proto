// @generated
impl serde::Serialize for CancelReservationRequest {
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
        if self.reason.is_some() {
            len += 1;
        }
        if self.cancelled_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CancelReservationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.cancelled_by_user_id.as_ref() {
            struct_ser.serialize_field("cancelledByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelReservationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "reason",
            "cancelled_by_user_id",
            "cancelledByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Reason,
            CancelledByUserId,
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
                            "reason" => Ok(GeneratedField::Reason),
                            "cancelledByUserId" | "cancelled_by_user_id" => Ok(GeneratedField::CancelledByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelReservationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CancelReservationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelReservationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut reason__ = None;
                let mut cancelled_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::CancelledByUserId => {
                            if cancelled_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelledByUserId"));
                            }
                            cancelled_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CancelReservationRequest {
                    id: id__.unwrap_or_default(),
                    reason: reason__,
                    cancelled_by_user_id: cancelled_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CancelReservationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckInGuestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reservation_id.is_empty() {
            len += 1;
        }
        if !self.room_id.is_empty() {
            len += 1;
        }
        if self.checked_in_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CheckInGuestRequest", len)?;
        if !self.reservation_id.is_empty() {
            struct_ser.serialize_field("reservationId", &self.reservation_id)?;
        }
        if !self.room_id.is_empty() {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        if let Some(v) = self.checked_in_by_user_id.as_ref() {
            struct_ser.serialize_field("checkedInByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckInGuestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservation_id",
            "reservationId",
            "room_id",
            "roomId",
            "checked_in_by_user_id",
            "checkedInByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReservationId,
            RoomId,
            CheckedInByUserId,
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
                            "reservationId" | "reservation_id" => Ok(GeneratedField::ReservationId),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            "checkedInByUserId" | "checked_in_by_user_id" => Ok(GeneratedField::CheckedInByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckInGuestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CheckInGuestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckInGuestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservation_id__ = None;
                let mut room_id__ = None;
                let mut checked_in_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReservationId => {
                            if reservation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationId"));
                            }
                            reservation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckedInByUserId => {
                            if checked_in_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedInByUserId"));
                            }
                            checked_in_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckInGuestRequest {
                    reservation_id: reservation_id__.unwrap_or_default(),
                    room_id: room_id__.unwrap_or_default(),
                    checked_in_by_user_id: checked_in_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CheckInGuestRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckOutGuestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reservation_id.is_empty() {
            len += 1;
        }
        if self.checked_out_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CheckOutGuestRequest", len)?;
        if !self.reservation_id.is_empty() {
            struct_ser.serialize_field("reservationId", &self.reservation_id)?;
        }
        if let Some(v) = self.checked_out_by_user_id.as_ref() {
            struct_ser.serialize_field("checkedOutByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckOutGuestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservation_id",
            "reservationId",
            "checked_out_by_user_id",
            "checkedOutByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReservationId,
            CheckedOutByUserId,
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
                            "reservationId" | "reservation_id" => Ok(GeneratedField::ReservationId),
                            "checkedOutByUserId" | "checked_out_by_user_id" => Ok(GeneratedField::CheckedOutByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckOutGuestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CheckOutGuestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckOutGuestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservation_id__ = None;
                let mut checked_out_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReservationId => {
                            if reservation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationId"));
                            }
                            reservation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckedOutByUserId => {
                            if checked_out_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedOutByUserId"));
                            }
                            checked_out_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckOutGuestRequest {
                    reservation_id: reservation_id__.unwrap_or_default(),
                    checked_out_by_user_id: checked_out_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CheckOutGuestRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateHotelServiceZoneRequest {
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
        if !self.code.is_empty() {
            len += 1;
        }
        if self.is_order_point.is_some() {
            len += 1;
        }
        if self.default_kitchen_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CreateHotelServiceZoneRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.is_order_point.as_ref() {
            struct_ser.serialize_field("isOrderPoint", v)?;
        }
        if let Some(v) = self.default_kitchen_id.as_ref() {
            struct_ser.serialize_field("defaultKitchenId", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateHotelServiceZoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "code",
            "is_order_point",
            "isOrderPoint",
            "default_kitchen_id",
            "defaultKitchenId",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            Code,
            IsOrderPoint,
            DefaultKitchenId,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "code" => Ok(GeneratedField::Code),
                            "isOrderPoint" | "is_order_point" => Ok(GeneratedField::IsOrderPoint),
                            "defaultKitchenId" | "default_kitchen_id" => Ok(GeneratedField::DefaultKitchenId),
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
            type Value = CreateHotelServiceZoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CreateHotelServiceZoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateHotelServiceZoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut code__ = None;
                let mut is_order_point__ = None;
                let mut default_kitchen_id__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsOrderPoint => {
                            if is_order_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOrderPoint"));
                            }
                            is_order_point__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultKitchenId => {
                            if default_kitchen_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultKitchenId"));
                            }
                            default_kitchen_id__ = map_.next_value()?;
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
                Ok(CreateHotelServiceZoneRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    is_order_point: is_order_point__,
                    default_kitchen_id: default_kitchen_id__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CreateHotelServiceZoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReservationRequest {
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
        if self.customer_id.is_some() {
            len += 1;
        }
        if !self.room_type_id.is_empty() {
            len += 1;
        }
        if !self.check_in_date.is_empty() {
            len += 1;
        }
        if !self.check_out_date.is_empty() {
            len += 1;
        }
        if self.num_adults != 0 {
            len += 1;
        }
        if self.num_children.is_some() {
            len += 1;
        }
        if !self.reservation_type.is_empty() {
            len += 1;
        }
        if self.reservable_item_id.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.initial_status.is_some() {
            len += 1;
        }
        if self.total_amount.is_some() {
            len += 1;
        }
        if self.currency_code.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.booked_by_user_id.is_some() {
            len += 1;
        }
        if self.booked_via_sales_channel_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CreateReservationRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.customer_id.as_ref() {
            struct_ser.serialize_field("customerId", v)?;
        }
        if !self.room_type_id.is_empty() {
            struct_ser.serialize_field("roomTypeId", &self.room_type_id)?;
        }
        if !self.check_in_date.is_empty() {
            struct_ser.serialize_field("checkInDate", &self.check_in_date)?;
        }
        if !self.check_out_date.is_empty() {
            struct_ser.serialize_field("checkOutDate", &self.check_out_date)?;
        }
        if self.num_adults != 0 {
            struct_ser.serialize_field("numAdults", &self.num_adults)?;
        }
        if let Some(v) = self.num_children.as_ref() {
            struct_ser.serialize_field("numChildren", v)?;
        }
        if !self.reservation_type.is_empty() {
            struct_ser.serialize_field("reservationType", &self.reservation_type)?;
        }
        if let Some(v) = self.reservable_item_id.as_ref() {
            struct_ser.serialize_field("reservableItemId", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if let Some(v) = self.initial_status.as_ref() {
            struct_ser.serialize_field("initialStatus", v)?;
        }
        if let Some(v) = self.total_amount.as_ref() {
            struct_ser.serialize_field("totalAmount", v)?;
        }
        if let Some(v) = self.currency_code.as_ref() {
            struct_ser.serialize_field("currencyCode", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.booked_by_user_id.as_ref() {
            struct_ser.serialize_field("bookedByUserId", v)?;
        }
        if let Some(v) = self.booked_via_sales_channel_id.as_ref() {
            struct_ser.serialize_field("bookedViaSalesChannelId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReservationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "customer_id",
            "customerId",
            "room_type_id",
            "roomTypeId",
            "check_in_date",
            "checkInDate",
            "check_out_date",
            "checkOutDate",
            "num_adults",
            "numAdults",
            "num_children",
            "numChildren",
            "reservation_type",
            "reservationType",
            "reservable_item_id",
            "reservableItemId",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "initial_status",
            "initialStatus",
            "total_amount",
            "totalAmount",
            "currency_code",
            "currencyCode",
            "notes",
            "booked_by_user_id",
            "bookedByUserId",
            "booked_via_sales_channel_id",
            "bookedViaSalesChannelId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            CustomerId,
            RoomTypeId,
            CheckInDate,
            CheckOutDate,
            NumAdults,
            NumChildren,
            ReservationType,
            ReservableItemId,
            StartTime,
            EndTime,
            InitialStatus,
            TotalAmount,
            CurrencyCode,
            Notes,
            BookedByUserId,
            BookedViaSalesChannelId,
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
                            "customerId" | "customer_id" => Ok(GeneratedField::CustomerId),
                            "roomTypeId" | "room_type_id" => Ok(GeneratedField::RoomTypeId),
                            "checkInDate" | "check_in_date" => Ok(GeneratedField::CheckInDate),
                            "checkOutDate" | "check_out_date" => Ok(GeneratedField::CheckOutDate),
                            "numAdults" | "num_adults" => Ok(GeneratedField::NumAdults),
                            "numChildren" | "num_children" => Ok(GeneratedField::NumChildren),
                            "reservationType" | "reservation_type" => Ok(GeneratedField::ReservationType),
                            "reservableItemId" | "reservable_item_id" => Ok(GeneratedField::ReservableItemId),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "initialStatus" | "initial_status" => Ok(GeneratedField::InitialStatus),
                            "totalAmount" | "total_amount" => Ok(GeneratedField::TotalAmount),
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            "notes" => Ok(GeneratedField::Notes),
                            "bookedByUserId" | "booked_by_user_id" => Ok(GeneratedField::BookedByUserId),
                            "bookedViaSalesChannelId" | "booked_via_sales_channel_id" => Ok(GeneratedField::BookedViaSalesChannelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReservationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CreateReservationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReservationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut customer_id__ = None;
                let mut room_type_id__ = None;
                let mut check_in_date__ = None;
                let mut check_out_date__ = None;
                let mut num_adults__ = None;
                let mut num_children__ = None;
                let mut reservation_type__ = None;
                let mut reservable_item_id__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut initial_status__ = None;
                let mut total_amount__ = None;
                let mut currency_code__ = None;
                let mut notes__ = None;
                let mut booked_by_user_id__ = None;
                let mut booked_via_sales_channel_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::RoomTypeId => {
                            if room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeId"));
                            }
                            room_type_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckInDate => {
                            if check_in_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkInDate"));
                            }
                            check_in_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckOutDate => {
                            if check_out_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkOutDate"));
                            }
                            check_out_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumAdults => {
                            if num_adults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numAdults"));
                            }
                            num_adults__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumChildren => {
                            if num_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numChildren"));
                            }
                            num_children__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ReservationType => {
                            if reservation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationType"));
                            }
                            reservation_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReservableItemId => {
                            if reservable_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservableItemId"));
                            }
                            reservable_item_id__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::InitialStatus => {
                            if initial_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStatus"));
                            }
                            initial_status__ = map_.next_value()?;
                        }
                        GeneratedField::TotalAmount => {
                            if total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmount"));
                            }
                            total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::BookedByUserId => {
                            if booked_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookedByUserId"));
                            }
                            booked_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::BookedViaSalesChannelId => {
                            if booked_via_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookedViaSalesChannelId"));
                            }
                            booked_via_sales_channel_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateReservationRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    customer_id: customer_id__,
                    room_type_id: room_type_id__.unwrap_or_default(),
                    check_in_date: check_in_date__.unwrap_or_default(),
                    check_out_date: check_out_date__.unwrap_or_default(),
                    num_adults: num_adults__.unwrap_or_default(),
                    num_children: num_children__,
                    reservation_type: reservation_type__.unwrap_or_default(),
                    reservable_item_id: reservable_item_id__,
                    start_time: start_time__,
                    end_time: end_time__,
                    initial_status: initial_status__,
                    total_amount: total_amount__,
                    currency_code: currency_code__,
                    notes: notes__,
                    booked_by_user_id: booked_by_user_id__,
                    booked_via_sales_channel_id: booked_via_sales_channel_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CreateReservationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRoomRequest {
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
        if !self.room_type_id.is_empty() {
            len += 1;
        }
        if !self.room_number.is_empty() {
            len += 1;
        }
        if self.floor_number.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.hk_status.is_some() {
            len += 1;
        }
        if self.features.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CreateRoomRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.room_type_id.is_empty() {
            struct_ser.serialize_field("roomTypeId", &self.room_type_id)?;
        }
        if !self.room_number.is_empty() {
            struct_ser.serialize_field("roomNumber", &self.room_number)?;
        }
        if let Some(v) = self.floor_number.as_ref() {
            struct_ser.serialize_field("floorNumber", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.hk_status.as_ref() {
            struct_ser.serialize_field("hkStatus", v)?;
        }
        if let Some(v) = self.features.as_ref() {
            struct_ser.serialize_field("features", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRoomRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "room_type_id",
            "roomTypeId",
            "room_number",
            "roomNumber",
            "floor_number",
            "floorNumber",
            "status",
            "hk_status",
            "hkStatus",
            "features",
            "is_active",
            "isActive",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            RoomTypeId,
            RoomNumber,
            FloorNumber,
            Status,
            HkStatus,
            Features,
            IsActive,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "roomTypeId" | "room_type_id" => Ok(GeneratedField::RoomTypeId),
                            "roomNumber" | "room_number" => Ok(GeneratedField::RoomNumber),
                            "floorNumber" | "floor_number" => Ok(GeneratedField::FloorNumber),
                            "status" => Ok(GeneratedField::Status),
                            "hkStatus" | "hk_status" => Ok(GeneratedField::HkStatus),
                            "features" => Ok(GeneratedField::Features),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = CreateRoomRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CreateRoomRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRoomRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut room_type_id__ = None;
                let mut room_number__ = None;
                let mut floor_number__ = None;
                let mut status__ = None;
                let mut hk_status__ = None;
                let mut features__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomTypeId => {
                            if room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeId"));
                            }
                            room_type_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomNumber => {
                            if room_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomNumber"));
                            }
                            room_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FloorNumber => {
                            if floor_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floorNumber"));
                            }
                            floor_number__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::HkStatus => {
                            if hk_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hkStatus"));
                            }
                            hk_status__ = map_.next_value()?;
                        }
                        GeneratedField::Features => {
                            if features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            features__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateRoomRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    room_type_id: room_type_id__.unwrap_or_default(),
                    room_number: room_number__.unwrap_or_default(),
                    floor_number: floor_number__,
                    status: status__,
                    hk_status: hk_status__,
                    features: features__,
                    is_active: is_active__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CreateRoomRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRoomTypeRequest {
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
        if !self.code.is_empty() {
            len += 1;
        }
        if self.capacity_adults != 0 {
            len += 1;
        }
        if self.capacity_children.is_some() {
            len += 1;
        }
        if self.base_price.is_some() {
            len += 1;
        }
        if self.amenities.is_some() {
            len += 1;
        }
        if !self.image_urls.is_empty() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.CreateRoomTypeRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if self.capacity_adults != 0 {
            struct_ser.serialize_field("capacityAdults", &self.capacity_adults)?;
        }
        if let Some(v) = self.capacity_children.as_ref() {
            struct_ser.serialize_field("capacityChildren", v)?;
        }
        if let Some(v) = self.base_price.as_ref() {
            struct_ser.serialize_field("basePrice", v)?;
        }
        if let Some(v) = self.amenities.as_ref() {
            struct_ser.serialize_field("amenities", v)?;
        }
        if !self.image_urls.is_empty() {
            struct_ser.serialize_field("imageUrls", &self.image_urls)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateRoomTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "code",
            "capacity_adults",
            "capacityAdults",
            "capacity_children",
            "capacityChildren",
            "base_price",
            "basePrice",
            "amenities",
            "image_urls",
            "imageUrls",
            "sort_order",
            "sortOrder",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            Code,
            CapacityAdults,
            CapacityChildren,
            BasePrice,
            Amenities,
            ImageUrls,
            SortOrder,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "code" => Ok(GeneratedField::Code),
                            "capacityAdults" | "capacity_adults" => Ok(GeneratedField::CapacityAdults),
                            "capacityChildren" | "capacity_children" => Ok(GeneratedField::CapacityChildren),
                            "basePrice" | "base_price" => Ok(GeneratedField::BasePrice),
                            "amenities" => Ok(GeneratedField::Amenities),
                            "imageUrls" | "image_urls" => Ok(GeneratedField::ImageUrls),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
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
            type Value = CreateRoomTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.CreateRoomTypeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRoomTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut code__ = None;
                let mut capacity_adults__ = None;
                let mut capacity_children__ = None;
                let mut base_price__ = None;
                let mut amenities__ = None;
                let mut image_urls__ = None;
                let mut sort_order__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CapacityAdults => {
                            if capacity_adults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacityAdults"));
                            }
                            capacity_adults__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CapacityChildren => {
                            if capacity_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacityChildren"));
                            }
                            capacity_children__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BasePrice => {
                            if base_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basePrice"));
                            }
                            base_price__ = map_.next_value()?;
                        }
                        GeneratedField::Amenities => {
                            if amenities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amenities"));
                            }
                            amenities__ = map_.next_value()?;
                        }
                        GeneratedField::ImageUrls => {
                            if image_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrls"));
                            }
                            image_urls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
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
                Ok(CreateRoomTypeRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    capacity_adults: capacity_adults__.unwrap_or_default(),
                    capacity_children: capacity_children__,
                    base_price: base_price__,
                    amenities: amenities__,
                    image_urls: image_urls__.unwrap_or_default(),
                    sort_order: sort_order__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.CreateRoomTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetHotelServiceZoneResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.zone.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.GetHotelServiceZoneResponse", len)?;
        if let Some(v) = self.zone.as_ref() {
            struct_ser.serialize_field("zone", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHotelServiceZoneResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Zone,
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
                            "zone" => Ok(GeneratedField::Zone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHotelServiceZoneResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.GetHotelServiceZoneResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHotelServiceZoneResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Zone => {
                            if zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zone"));
                            }
                            zone__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetHotelServiceZoneResponse {
                    zone: zone__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.GetHotelServiceZoneResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReservationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reservation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.GetReservationResponse", len)?;
        if let Some(v) = self.reservation.as_ref() {
            struct_ser.serialize_field("reservation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReservationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reservation,
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
                            "reservation" => Ok(GeneratedField::Reservation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetReservationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.GetReservationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReservationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reservation => {
                            if reservation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservation"));
                            }
                            reservation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetReservationResponse {
                    reservation: reservation__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.GetReservationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRoomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.GetRoomResponse", len)?;
        if let Some(v) = self.room.as_ref() {
            struct_ser.serialize_field("room", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRoomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Room,
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
                            "room" => Ok(GeneratedField::Room),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRoomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.GetRoomResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRoomResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Room => {
                            if room__.is_some() {
                                return Err(serde::de::Error::duplicate_field("room"));
                            }
                            room__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRoomResponse {
                    room: room__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.GetRoomResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRoomTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.GetRoomTypeResponse", len)?;
        if let Some(v) = self.room_type.as_ref() {
            struct_ser.serialize_field("roomType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRoomTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_type",
            "roomType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomType,
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
                            "roomType" | "room_type" => Ok(GeneratedField::RoomType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRoomTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.GetRoomTypeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRoomTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomType => {
                            if room_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomType"));
                            }
                            room_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRoomTypeResponse {
                    room_type: room_type__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.GetRoomTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HotelServiceZone {
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
        if !self.code.is_empty() {
            len += 1;
        }
        if self.is_order_point {
            len += 1;
        }
        if self.default_kitchen_id.is_some() {
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
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.HotelServiceZone", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if self.is_order_point {
            struct_ser.serialize_field("isOrderPoint", &self.is_order_point)?;
        }
        if let Some(v) = self.default_kitchen_id.as_ref() {
            struct_ser.serialize_field("defaultKitchenId", v)?;
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
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HotelServiceZone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "code",
            "is_order_point",
            "isOrderPoint",
            "default_kitchen_id",
            "defaultKitchenId",
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
            "warehouse_summary",
            "warehouseSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            Code,
            IsOrderPoint,
            DefaultKitchenId,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            AllTranslations,
            WarehouseSummary,
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
                            "code" => Ok(GeneratedField::Code),
                            "isOrderPoint" | "is_order_point" => Ok(GeneratedField::IsOrderPoint),
                            "defaultKitchenId" | "default_kitchen_id" => Ok(GeneratedField::DefaultKitchenId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HotelServiceZone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.HotelServiceZone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HotelServiceZone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut code__ = None;
                let mut is_order_point__ = None;
                let mut default_kitchen_id__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut all_translations__ = None;
                let mut warehouse_summary__ = None;
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
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsOrderPoint => {
                            if is_order_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOrderPoint"));
                            }
                            is_order_point__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultKitchenId => {
                            if default_kitchen_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultKitchenId"));
                            }
                            default_kitchen_id__ = map_.next_value()?;
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
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HotelServiceZone {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    is_order_point: is_order_point__.unwrap_or_default(),
                    default_kitchen_id: default_kitchen_id__,
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    warehouse_summary: warehouse_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.HotelServiceZone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListHotelServiceZonesRequest {
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
        if !self.filter_by_warehouse_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListHotelServiceZonesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if !self.filter_by_warehouse_id.is_empty() {
            struct_ser.serialize_field("filterByWarehouseId", &self.filter_by_warehouse_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListHotelServiceZonesRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
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
            type Value = ListHotelServiceZonesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListHotelServiceZonesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListHotelServiceZonesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
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
                            filter_by_warehouse_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListHotelServiceZonesRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListHotelServiceZonesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListHotelServiceZonesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zones.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListHotelServiceZonesResponse", len)?;
        if !self.zones.is_empty() {
            struct_ser.serialize_field("zones", &self.zones)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListHotelServiceZonesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zones",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Zones,
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
                            "zones" => Ok(GeneratedField::Zones),
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
            type Value = ListHotelServiceZonesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListHotelServiceZonesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListHotelServiceZonesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zones__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Zones => {
                            if zones__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zones"));
                            }
                            zones__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListHotelServiceZonesResponse {
                    zones: zones__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListHotelServiceZonesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListReservationsRequest {
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
        if !self.filter_by_warehouse_id.is_empty() {
            len += 1;
        }
        if self.filter_by_customer_id.is_some() {
            len += 1;
        }
        if self.filter_by_room_id.is_some() {
            len += 1;
        }
        if self.filter_by_room_type_id.is_some() {
            len += 1;
        }
        if self.filter_by_check_in_date_range.is_some() {
            len += 1;
        }
        if self.filter_by_check_out_date_range.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListReservationsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if !self.filter_by_warehouse_id.is_empty() {
            struct_ser.serialize_field("filterByWarehouseId", &self.filter_by_warehouse_id)?;
        }
        if let Some(v) = self.filter_by_customer_id.as_ref() {
            struct_ser.serialize_field("filterByCustomerId", v)?;
        }
        if let Some(v) = self.filter_by_room_id.as_ref() {
            struct_ser.serialize_field("filterByRoomId", v)?;
        }
        if let Some(v) = self.filter_by_room_type_id.as_ref() {
            struct_ser.serialize_field("filterByRoomTypeId", v)?;
        }
        if let Some(v) = self.filter_by_check_in_date_range.as_ref() {
            struct_ser.serialize_field("filterByCheckInDateRange", v)?;
        }
        if let Some(v) = self.filter_by_check_out_date_range.as_ref() {
            struct_ser.serialize_field("filterByCheckOutDateRange", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListReservationsRequest {
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
            "filter_by_customer_id",
            "filterByCustomerId",
            "filter_by_room_id",
            "filterByRoomId",
            "filter_by_room_type_id",
            "filterByRoomTypeId",
            "filter_by_check_in_date_range",
            "filterByCheckInDateRange",
            "filter_by_check_out_date_range",
            "filterByCheckOutDateRange",
            "filter_by_status",
            "filterByStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByCustomerId,
            FilterByRoomId,
            FilterByRoomTypeId,
            FilterByCheckInDateRange,
            FilterByCheckOutDateRange,
            FilterByStatus,
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
                            "filterByCustomerId" | "filter_by_customer_id" => Ok(GeneratedField::FilterByCustomerId),
                            "filterByRoomId" | "filter_by_room_id" => Ok(GeneratedField::FilterByRoomId),
                            "filterByRoomTypeId" | "filter_by_room_type_id" => Ok(GeneratedField::FilterByRoomTypeId),
                            "filterByCheckInDateRange" | "filter_by_check_in_date_range" => Ok(GeneratedField::FilterByCheckInDateRange),
                            "filterByCheckOutDateRange" | "filter_by_check_out_date_range" => Ok(GeneratedField::FilterByCheckOutDateRange),
                            "filterByStatus" | "filter_by_status" => Ok(GeneratedField::FilterByStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListReservationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListReservationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListReservationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_customer_id__ = None;
                let mut filter_by_room_id__ = None;
                let mut filter_by_room_type_id__ = None;
                let mut filter_by_check_in_date_range__ = None;
                let mut filter_by_check_out_date_range__ = None;
                let mut filter_by_status__ = None;
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
                            filter_by_warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterByCustomerId => {
                            if filter_by_customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByCustomerId"));
                            }
                            filter_by_customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByRoomId => {
                            if filter_by_room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByRoomId"));
                            }
                            filter_by_room_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByRoomTypeId => {
                            if filter_by_room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByRoomTypeId"));
                            }
                            filter_by_room_type_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByCheckInDateRange => {
                            if filter_by_check_in_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByCheckInDateRange"));
                            }
                            filter_by_check_in_date_range__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByCheckOutDateRange => {
                            if filter_by_check_out_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByCheckOutDateRange"));
                            }
                            filter_by_check_out_date_range__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListReservationsRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__.unwrap_or_default(),
                    filter_by_customer_id: filter_by_customer_id__,
                    filter_by_room_id: filter_by_room_id__,
                    filter_by_room_type_id: filter_by_room_type_id__,
                    filter_by_check_in_date_range: filter_by_check_in_date_range__,
                    filter_by_check_out_date_range: filter_by_check_out_date_range__,
                    filter_by_status: filter_by_status__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListReservationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListReservationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reservations.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListReservationsResponse", len)?;
        if !self.reservations.is_empty() {
            struct_ser.serialize_field("reservations", &self.reservations)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListReservationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservations",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reservations,
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
                            "reservations" => Ok(GeneratedField::Reservations),
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
            type Value = ListReservationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListReservationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListReservationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservations__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reservations => {
                            if reservations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservations"));
                            }
                            reservations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListReservationsResponse {
                    reservations: reservations__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListReservationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRoomTypesRequest {
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
        if !self.filter_by_warehouse_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListRoomTypesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if !self.filter_by_warehouse_id.is_empty() {
            struct_ser.serialize_field("filterByWarehouseId", &self.filter_by_warehouse_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRoomTypesRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
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
            type Value = ListRoomTypesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListRoomTypesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRoomTypesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
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
                            filter_by_warehouse_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListRoomTypesRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListRoomTypesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRoomTypesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.room_types.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListRoomTypesResponse", len)?;
        if !self.room_types.is_empty() {
            struct_ser.serialize_field("roomTypes", &self.room_types)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRoomTypesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_types",
            "roomTypes",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomTypes,
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
                            "roomTypes" | "room_types" => Ok(GeneratedField::RoomTypes),
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
            type Value = ListRoomTypesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListRoomTypesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRoomTypesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_types__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomTypes => {
                            if room_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypes"));
                            }
                            room_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListRoomTypesResponse {
                    room_types: room_types__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListRoomTypesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRoomsRequest {
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
        if !self.filter_by_warehouse_id.is_empty() {
            len += 1;
        }
        if self.filter_by_room_type_id.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        if self.filter_by_hk_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListRoomsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if !self.filter_by_warehouse_id.is_empty() {
            struct_ser.serialize_field("filterByWarehouseId", &self.filter_by_warehouse_id)?;
        }
        if let Some(v) = self.filter_by_room_type_id.as_ref() {
            struct_ser.serialize_field("filterByRoomTypeId", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        if let Some(v) = self.filter_by_hk_status.as_ref() {
            struct_ser.serialize_field("filterByHkStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRoomsRequest {
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
            "filter_by_room_type_id",
            "filterByRoomTypeId",
            "filter_by_status",
            "filterByStatus",
            "filter_by_hk_status",
            "filterByHkStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByRoomTypeId,
            FilterByStatus,
            FilterByHkStatus,
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
                            "filterByRoomTypeId" | "filter_by_room_type_id" => Ok(GeneratedField::FilterByRoomTypeId),
                            "filterByStatus" | "filter_by_status" => Ok(GeneratedField::FilterByStatus),
                            "filterByHkStatus" | "filter_by_hk_status" => Ok(GeneratedField::FilterByHkStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRoomsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListRoomsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRoomsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_room_type_id__ = None;
                let mut filter_by_status__ = None;
                let mut filter_by_hk_status__ = None;
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
                            filter_by_warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterByRoomTypeId => {
                            if filter_by_room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByRoomTypeId"));
                            }
                            filter_by_room_type_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByHkStatus => {
                            if filter_by_hk_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByHkStatus"));
                            }
                            filter_by_hk_status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListRoomsRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__.unwrap_or_default(),
                    filter_by_room_type_id: filter_by_room_type_id__,
                    filter_by_status: filter_by_status__,
                    filter_by_hk_status: filter_by_hk_status__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListRoomsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRoomsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rooms.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.ListRoomsResponse", len)?;
        if !self.rooms.is_empty() {
            struct_ser.serialize_field("rooms", &self.rooms)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRoomsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rooms",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rooms,
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
                            "rooms" => Ok(GeneratedField::Rooms),
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
            type Value = ListRoomsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.ListRoomsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRoomsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rooms__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rooms => {
                            if rooms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rooms"));
                            }
                            rooms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListRoomsResponse {
                    rooms: rooms__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.ListRoomsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Reservation {
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
        if self.reservation_number.is_some() {
            len += 1;
        }
        if self.customer_id.is_some() {
            len += 1;
        }
        if self.room_id.is_some() {
            len += 1;
        }
        if !self.room_type_id.is_empty() {
            len += 1;
        }
        if !self.check_in_date.is_empty() {
            len += 1;
        }
        if !self.check_out_date.is_empty() {
            len += 1;
        }
        if self.num_adults != 0 {
            len += 1;
        }
        if self.num_children != 0 {
            len += 1;
        }
        if !self.reservation_type.is_empty() {
            len += 1;
        }
        if self.reservable_item_id.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.total_amount.is_some() {
            len += 1;
        }
        if self.amount_paid.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.booked_by_user_id.is_some() {
            len += 1;
        }
        if self.booked_via_sales_channel_id.is_some() {
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
        if self.customer_details.is_some() {
            len += 1;
        }
        if self.room_details.is_some() {
            len += 1;
        }
        if self.room_type_details_requested.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.Reservation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.reservation_number.as_ref() {
            struct_ser.serialize_field("reservationNumber", v)?;
        }
        if let Some(v) = self.customer_id.as_ref() {
            struct_ser.serialize_field("customerId", v)?;
        }
        if let Some(v) = self.room_id.as_ref() {
            struct_ser.serialize_field("roomId", v)?;
        }
        if !self.room_type_id.is_empty() {
            struct_ser.serialize_field("roomTypeId", &self.room_type_id)?;
        }
        if !self.check_in_date.is_empty() {
            struct_ser.serialize_field("checkInDate", &self.check_in_date)?;
        }
        if !self.check_out_date.is_empty() {
            struct_ser.serialize_field("checkOutDate", &self.check_out_date)?;
        }
        if self.num_adults != 0 {
            struct_ser.serialize_field("numAdults", &self.num_adults)?;
        }
        if self.num_children != 0 {
            struct_ser.serialize_field("numChildren", &self.num_children)?;
        }
        if !self.reservation_type.is_empty() {
            struct_ser.serialize_field("reservationType", &self.reservation_type)?;
        }
        if let Some(v) = self.reservable_item_id.as_ref() {
            struct_ser.serialize_field("reservableItemId", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.total_amount.as_ref() {
            struct_ser.serialize_field("totalAmount", v)?;
        }
        if let Some(v) = self.amount_paid.as_ref() {
            struct_ser.serialize_field("amountPaid", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.booked_by_user_id.as_ref() {
            struct_ser.serialize_field("bookedByUserId", v)?;
        }
        if let Some(v) = self.booked_via_sales_channel_id.as_ref() {
            struct_ser.serialize_field("bookedViaSalesChannelId", v)?;
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
        if let Some(v) = self.customer_details.as_ref() {
            struct_ser.serialize_field("customerDetails", v)?;
        }
        if let Some(v) = self.room_details.as_ref() {
            struct_ser.serialize_field("roomDetails", v)?;
        }
        if let Some(v) = self.room_type_details_requested.as_ref() {
            struct_ser.serialize_field("roomTypeDetailsRequested", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Reservation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "reservation_number",
            "reservationNumber",
            "customer_id",
            "customerId",
            "room_id",
            "roomId",
            "room_type_id",
            "roomTypeId",
            "check_in_date",
            "checkInDate",
            "check_out_date",
            "checkOutDate",
            "num_adults",
            "numAdults",
            "num_children",
            "numChildren",
            "reservation_type",
            "reservationType",
            "reservable_item_id",
            "reservableItemId",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "status",
            "total_amount",
            "totalAmount",
            "amount_paid",
            "amountPaid",
            "notes",
            "booked_by_user_id",
            "bookedByUserId",
            "booked_via_sales_channel_id",
            "bookedViaSalesChannelId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "warehouse_summary",
            "warehouseSummary",
            "customer_details",
            "customerDetails",
            "room_details",
            "roomDetails",
            "room_type_details_requested",
            "roomTypeDetailsRequested",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            ReservationNumber,
            CustomerId,
            RoomId,
            RoomTypeId,
            CheckInDate,
            CheckOutDate,
            NumAdults,
            NumChildren,
            ReservationType,
            ReservableItemId,
            StartTime,
            EndTime,
            Status,
            TotalAmount,
            AmountPaid,
            Notes,
            BookedByUserId,
            BookedViaSalesChannelId,
            CreatedAt,
            UpdatedAt,
            WarehouseSummary,
            CustomerDetails,
            RoomDetails,
            RoomTypeDetailsRequested,
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
                            "reservationNumber" | "reservation_number" => Ok(GeneratedField::ReservationNumber),
                            "customerId" | "customer_id" => Ok(GeneratedField::CustomerId),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            "roomTypeId" | "room_type_id" => Ok(GeneratedField::RoomTypeId),
                            "checkInDate" | "check_in_date" => Ok(GeneratedField::CheckInDate),
                            "checkOutDate" | "check_out_date" => Ok(GeneratedField::CheckOutDate),
                            "numAdults" | "num_adults" => Ok(GeneratedField::NumAdults),
                            "numChildren" | "num_children" => Ok(GeneratedField::NumChildren),
                            "reservationType" | "reservation_type" => Ok(GeneratedField::ReservationType),
                            "reservableItemId" | "reservable_item_id" => Ok(GeneratedField::ReservableItemId),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "status" => Ok(GeneratedField::Status),
                            "totalAmount" | "total_amount" => Ok(GeneratedField::TotalAmount),
                            "amountPaid" | "amount_paid" => Ok(GeneratedField::AmountPaid),
                            "notes" => Ok(GeneratedField::Notes),
                            "bookedByUserId" | "booked_by_user_id" => Ok(GeneratedField::BookedByUserId),
                            "bookedViaSalesChannelId" | "booked_via_sales_channel_id" => Ok(GeneratedField::BookedViaSalesChannelId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            "customerDetails" | "customer_details" => Ok(GeneratedField::CustomerDetails),
                            "roomDetails" | "room_details" => Ok(GeneratedField::RoomDetails),
                            "roomTypeDetailsRequested" | "room_type_details_requested" => Ok(GeneratedField::RoomTypeDetailsRequested),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Reservation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.Reservation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Reservation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut reservation_number__ = None;
                let mut customer_id__ = None;
                let mut room_id__ = None;
                let mut room_type_id__ = None;
                let mut check_in_date__ = None;
                let mut check_out_date__ = None;
                let mut num_adults__ = None;
                let mut num_children__ = None;
                let mut reservation_type__ = None;
                let mut reservable_item_id__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut status__ = None;
                let mut total_amount__ = None;
                let mut amount_paid__ = None;
                let mut notes__ = None;
                let mut booked_by_user_id__ = None;
                let mut booked_via_sales_channel_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut warehouse_summary__ = None;
                let mut customer_details__ = None;
                let mut room_details__ = None;
                let mut room_type_details_requested__ = None;
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
                        GeneratedField::ReservationNumber => {
                            if reservation_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationNumber"));
                            }
                            reservation_number__ = map_.next_value()?;
                        }
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = map_.next_value()?;
                        }
                        GeneratedField::RoomTypeId => {
                            if room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeId"));
                            }
                            room_type_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckInDate => {
                            if check_in_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkInDate"));
                            }
                            check_in_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckOutDate => {
                            if check_out_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkOutDate"));
                            }
                            check_out_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumAdults => {
                            if num_adults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numAdults"));
                            }
                            num_adults__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumChildren => {
                            if num_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numChildren"));
                            }
                            num_children__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReservationType => {
                            if reservation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationType"));
                            }
                            reservation_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReservableItemId => {
                            if reservable_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservableItemId"));
                            }
                            reservable_item_id__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalAmount => {
                            if total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmount"));
                            }
                            total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::AmountPaid => {
                            if amount_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountPaid"));
                            }
                            amount_paid__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::BookedByUserId => {
                            if booked_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookedByUserId"));
                            }
                            booked_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::BookedViaSalesChannelId => {
                            if booked_via_sales_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookedViaSalesChannelId"));
                            }
                            booked_via_sales_channel_id__ = map_.next_value()?;
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
                        GeneratedField::CustomerDetails => {
                            if customer_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerDetails"));
                            }
                            customer_details__ = map_.next_value()?;
                        }
                        GeneratedField::RoomDetails => {
                            if room_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomDetails"));
                            }
                            room_details__ = map_.next_value()?;
                        }
                        GeneratedField::RoomTypeDetailsRequested => {
                            if room_type_details_requested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeDetailsRequested"));
                            }
                            room_type_details_requested__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Reservation {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    reservation_number: reservation_number__,
                    customer_id: customer_id__,
                    room_id: room_id__,
                    room_type_id: room_type_id__.unwrap_or_default(),
                    check_in_date: check_in_date__.unwrap_or_default(),
                    check_out_date: check_out_date__.unwrap_or_default(),
                    num_adults: num_adults__.unwrap_or_default(),
                    num_children: num_children__.unwrap_or_default(),
                    reservation_type: reservation_type__.unwrap_or_default(),
                    reservable_item_id: reservable_item_id__,
                    start_time: start_time__,
                    end_time: end_time__,
                    status: status__.unwrap_or_default(),
                    total_amount: total_amount__,
                    amount_paid: amount_paid__,
                    notes: notes__,
                    booked_by_user_id: booked_by_user_id__,
                    booked_via_sales_channel_id: booked_via_sales_channel_id__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    warehouse_summary: warehouse_summary__,
                    customer_details: customer_details__,
                    room_details: room_details__,
                    room_type_details_requested: room_type_details_requested__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.Reservation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Room {
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
        if !self.room_type_id.is_empty() {
            len += 1;
        }
        if !self.room_number.is_empty() {
            len += 1;
        }
        if self.floor_number.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.hk_status.is_some() {
            len += 1;
        }
        if self.features.is_some() {
            len += 1;
        }
        if self.is_active {
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
        if self.room_type_details.is_some() {
            len += 1;
        }
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.Room", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.room_type_id.is_empty() {
            struct_ser.serialize_field("roomTypeId", &self.room_type_id)?;
        }
        if !self.room_number.is_empty() {
            struct_ser.serialize_field("roomNumber", &self.room_number)?;
        }
        if let Some(v) = self.floor_number.as_ref() {
            struct_ser.serialize_field("floorNumber", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.hk_status.as_ref() {
            struct_ser.serialize_field("hkStatus", v)?;
        }
        if let Some(v) = self.features.as_ref() {
            struct_ser.serialize_field("features", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
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
        if let Some(v) = self.room_type_details.as_ref() {
            struct_ser.serialize_field("roomTypeDetails", v)?;
        }
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Room {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "room_type_id",
            "roomTypeId",
            "room_number",
            "roomNumber",
            "floor_number",
            "floorNumber",
            "status",
            "hk_status",
            "hkStatus",
            "features",
            "is_active",
            "isActive",
            "notes",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "room_type_details",
            "roomTypeDetails",
            "warehouse_summary",
            "warehouseSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            RoomTypeId,
            RoomNumber,
            FloorNumber,
            Status,
            HkStatus,
            Features,
            IsActive,
            Notes,
            CreatedAt,
            UpdatedAt,
            RoomTypeDetails,
            WarehouseSummary,
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
                            "roomTypeId" | "room_type_id" => Ok(GeneratedField::RoomTypeId),
                            "roomNumber" | "room_number" => Ok(GeneratedField::RoomNumber),
                            "floorNumber" | "floor_number" => Ok(GeneratedField::FloorNumber),
                            "status" => Ok(GeneratedField::Status),
                            "hkStatus" | "hk_status" => Ok(GeneratedField::HkStatus),
                            "features" => Ok(GeneratedField::Features),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "roomTypeDetails" | "room_type_details" => Ok(GeneratedField::RoomTypeDetails),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Room;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.Room")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Room, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut room_type_id__ = None;
                let mut room_number__ = None;
                let mut floor_number__ = None;
                let mut status__ = None;
                let mut hk_status__ = None;
                let mut features__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut room_type_details__ = None;
                let mut warehouse_summary__ = None;
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
                        GeneratedField::RoomTypeId => {
                            if room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeId"));
                            }
                            room_type_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomNumber => {
                            if room_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomNumber"));
                            }
                            room_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FloorNumber => {
                            if floor_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floorNumber"));
                            }
                            floor_number__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HkStatus => {
                            if hk_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hkStatus"));
                            }
                            hk_status__ = map_.next_value()?;
                        }
                        GeneratedField::Features => {
                            if features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            features__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
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
                        GeneratedField::RoomTypeDetails => {
                            if room_type_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeDetails"));
                            }
                            room_type_details__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Room {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    room_type_id: room_type_id__.unwrap_or_default(),
                    room_number: room_number__.unwrap_or_default(),
                    floor_number: floor_number__,
                    status: status__.unwrap_or_default(),
                    hk_status: hk_status__,
                    features: features__,
                    is_active: is_active__.unwrap_or_default(),
                    notes: notes__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    room_type_details: room_type_details__,
                    warehouse_summary: warehouse_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.Room", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoomType {
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
        if !self.code.is_empty() {
            len += 1;
        }
        if self.capacity_adults != 0 {
            len += 1;
        }
        if self.capacity_children != 0 {
            len += 1;
        }
        if self.base_price.is_some() {
            len += 1;
        }
        if self.amenities.is_some() {
            len += 1;
        }
        if !self.image_urls.is_empty() {
            len += 1;
        }
        if self.sort_order != 0 {
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
        if !self.policy_details.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.RoomType", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if self.capacity_adults != 0 {
            struct_ser.serialize_field("capacityAdults", &self.capacity_adults)?;
        }
        if self.capacity_children != 0 {
            struct_ser.serialize_field("capacityChildren", &self.capacity_children)?;
        }
        if let Some(v) = self.base_price.as_ref() {
            struct_ser.serialize_field("basePrice", v)?;
        }
        if let Some(v) = self.amenities.as_ref() {
            struct_ser.serialize_field("amenities", v)?;
        }
        if !self.image_urls.is_empty() {
            struct_ser.serialize_field("imageUrls", &self.image_urls)?;
        }
        if self.sort_order != 0 {
            struct_ser.serialize_field("sortOrder", &self.sort_order)?;
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
        if !self.policy_details.is_empty() {
            struct_ser.serialize_field("policyDetails", &self.policy_details)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoomType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "code",
            "capacity_adults",
            "capacityAdults",
            "capacity_children",
            "capacityChildren",
            "base_price",
            "basePrice",
            "amenities",
            "image_urls",
            "imageUrls",
            "sort_order",
            "sortOrder",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "description",
            "policy_details",
            "policyDetails",
            "all_translations",
            "allTranslations",
            "warehouse_summary",
            "warehouseSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            Code,
            CapacityAdults,
            CapacityChildren,
            BasePrice,
            Amenities,
            ImageUrls,
            SortOrder,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            PolicyDetails,
            AllTranslations,
            WarehouseSummary,
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
                            "code" => Ok(GeneratedField::Code),
                            "capacityAdults" | "capacity_adults" => Ok(GeneratedField::CapacityAdults),
                            "capacityChildren" | "capacity_children" => Ok(GeneratedField::CapacityChildren),
                            "basePrice" | "base_price" => Ok(GeneratedField::BasePrice),
                            "amenities" => Ok(GeneratedField::Amenities),
                            "imageUrls" | "image_urls" => Ok(GeneratedField::ImageUrls),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "policyDetails" | "policy_details" => Ok(GeneratedField::PolicyDetails),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoomType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.RoomType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoomType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut code__ = None;
                let mut capacity_adults__ = None;
                let mut capacity_children__ = None;
                let mut base_price__ = None;
                let mut amenities__ = None;
                let mut image_urls__ = None;
                let mut sort_order__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut policy_details__ = None;
                let mut all_translations__ = None;
                let mut warehouse_summary__ = None;
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
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CapacityAdults => {
                            if capacity_adults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacityAdults"));
                            }
                            capacity_adults__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CapacityChildren => {
                            if capacity_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacityChildren"));
                            }
                            capacity_children__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BasePrice => {
                            if base_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basePrice"));
                            }
                            base_price__ = map_.next_value()?;
                        }
                        GeneratedField::Amenities => {
                            if amenities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amenities"));
                            }
                            amenities__ = map_.next_value()?;
                        }
                        GeneratedField::ImageUrls => {
                            if image_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrls"));
                            }
                            image_urls__ = Some(map_.next_value()?);
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
                        GeneratedField::PolicyDetails => {
                            if policy_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyDetails"));
                            }
                            policy_details__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RoomType {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    capacity_adults: capacity_adults__.unwrap_or_default(),
                    capacity_children: capacity_children__.unwrap_or_default(),
                    base_price: base_price__,
                    amenities: amenities__,
                    image_urls: image_urls__.unwrap_or_default(),
                    sort_order: sort_order__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    policy_details: policy_details__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    warehouse_summary: warehouse_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.RoomType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateHotelServiceZoneRequest {
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
        if self.is_order_point.is_some() {
            len += 1;
        }
        if self.default_kitchen_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.UpdateHotelServiceZoneRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.is_order_point.as_ref() {
            struct_ser.serialize_field("isOrderPoint", v)?;
        }
        if let Some(v) = self.default_kitchen_id.as_ref() {
            struct_ser.serialize_field("defaultKitchenId", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateHotelServiceZoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "is_order_point",
            "isOrderPoint",
            "default_kitchen_id",
            "defaultKitchenId",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            IsOrderPoint,
            DefaultKitchenId,
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
                            "isOrderPoint" | "is_order_point" => Ok(GeneratedField::IsOrderPoint),
                            "defaultKitchenId" | "default_kitchen_id" => Ok(GeneratedField::DefaultKitchenId),
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
            type Value = UpdateHotelServiceZoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.UpdateHotelServiceZoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateHotelServiceZoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut is_order_point__ = None;
                let mut default_kitchen_id__ = None;
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
                        GeneratedField::IsOrderPoint => {
                            if is_order_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOrderPoint"));
                            }
                            is_order_point__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultKitchenId => {
                            if default_kitchen_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultKitchenId"));
                            }
                            default_kitchen_id__ = map_.next_value()?;
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
                Ok(UpdateHotelServiceZoneRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    is_order_point: is_order_point__,
                    default_kitchen_id: default_kitchen_id__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.UpdateHotelServiceZoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateReservationRequest {
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
        if self.customer_id.is_some() {
            len += 1;
        }
        if self.room_id.is_some() {
            len += 1;
        }
        if self.room_type_id.is_some() {
            len += 1;
        }
        if self.check_in_date.is_some() {
            len += 1;
        }
        if self.check_out_date.is_some() {
            len += 1;
        }
        if self.num_adults.is_some() {
            len += 1;
        }
        if self.num_children.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.total_amount.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.UpdateReservationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.customer_id.as_ref() {
            struct_ser.serialize_field("customerId", v)?;
        }
        if let Some(v) = self.room_id.as_ref() {
            struct_ser.serialize_field("roomId", v)?;
        }
        if let Some(v) = self.room_type_id.as_ref() {
            struct_ser.serialize_field("roomTypeId", v)?;
        }
        if let Some(v) = self.check_in_date.as_ref() {
            struct_ser.serialize_field("checkInDate", v)?;
        }
        if let Some(v) = self.check_out_date.as_ref() {
            struct_ser.serialize_field("checkOutDate", v)?;
        }
        if let Some(v) = self.num_adults.as_ref() {
            struct_ser.serialize_field("numAdults", v)?;
        }
        if let Some(v) = self.num_children.as_ref() {
            struct_ser.serialize_field("numChildren", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.total_amount.as_ref() {
            struct_ser.serialize_field("totalAmount", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateReservationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "customer_id",
            "customerId",
            "room_id",
            "roomId",
            "room_type_id",
            "roomTypeId",
            "check_in_date",
            "checkInDate",
            "check_out_date",
            "checkOutDate",
            "num_adults",
            "numAdults",
            "num_children",
            "numChildren",
            "status",
            "total_amount",
            "totalAmount",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CustomerId,
            RoomId,
            RoomTypeId,
            CheckInDate,
            CheckOutDate,
            NumAdults,
            NumChildren,
            Status,
            TotalAmount,
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
                            "id" => Ok(GeneratedField::Id),
                            "customerId" | "customer_id" => Ok(GeneratedField::CustomerId),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            "roomTypeId" | "room_type_id" => Ok(GeneratedField::RoomTypeId),
                            "checkInDate" | "check_in_date" => Ok(GeneratedField::CheckInDate),
                            "checkOutDate" | "check_out_date" => Ok(GeneratedField::CheckOutDate),
                            "numAdults" | "num_adults" => Ok(GeneratedField::NumAdults),
                            "numChildren" | "num_children" => Ok(GeneratedField::NumChildren),
                            "status" => Ok(GeneratedField::Status),
                            "totalAmount" | "total_amount" => Ok(GeneratedField::TotalAmount),
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
            type Value = UpdateReservationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.UpdateReservationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateReservationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut customer_id__ = None;
                let mut room_id__ = None;
                let mut room_type_id__ = None;
                let mut check_in_date__ = None;
                let mut check_out_date__ = None;
                let mut num_adults__ = None;
                let mut num_children__ = None;
                let mut status__ = None;
                let mut total_amount__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = map_.next_value()?;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = map_.next_value()?;
                        }
                        GeneratedField::RoomTypeId => {
                            if room_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomTypeId"));
                            }
                            room_type_id__ = map_.next_value()?;
                        }
                        GeneratedField::CheckInDate => {
                            if check_in_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkInDate"));
                            }
                            check_in_date__ = map_.next_value()?;
                        }
                        GeneratedField::CheckOutDate => {
                            if check_out_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkOutDate"));
                            }
                            check_out_date__ = map_.next_value()?;
                        }
                        GeneratedField::NumAdults => {
                            if num_adults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numAdults"));
                            }
                            num_adults__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::NumChildren => {
                            if num_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numChildren"));
                            }
                            num_children__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::TotalAmount => {
                            if total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmount"));
                            }
                            total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateReservationRequest {
                    id: id__.unwrap_or_default(),
                    customer_id: customer_id__,
                    room_id: room_id__,
                    room_type_id: room_type_id__,
                    check_in_date: check_in_date__,
                    check_out_date: check_out_date__,
                    num_adults: num_adults__,
                    num_children: num_children__,
                    status: status__,
                    total_amount: total_amount__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.UpdateReservationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRoomRequest {
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
        if self.room_number.is_some() {
            len += 1;
        }
        if self.floor_number.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.hk_status.is_some() {
            len += 1;
        }
        if self.features.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.UpdateRoomRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.room_number.as_ref() {
            struct_ser.serialize_field("roomNumber", v)?;
        }
        if let Some(v) = self.floor_number.as_ref() {
            struct_ser.serialize_field("floorNumber", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.hk_status.as_ref() {
            struct_ser.serialize_field("hkStatus", v)?;
        }
        if let Some(v) = self.features.as_ref() {
            struct_ser.serialize_field("features", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRoomRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "room_number",
            "roomNumber",
            "floor_number",
            "floorNumber",
            "status",
            "hk_status",
            "hkStatus",
            "features",
            "is_active",
            "isActive",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            RoomNumber,
            FloorNumber,
            Status,
            HkStatus,
            Features,
            IsActive,
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
                            "id" => Ok(GeneratedField::Id),
                            "roomNumber" | "room_number" => Ok(GeneratedField::RoomNumber),
                            "floorNumber" | "floor_number" => Ok(GeneratedField::FloorNumber),
                            "status" => Ok(GeneratedField::Status),
                            "hkStatus" | "hk_status" => Ok(GeneratedField::HkStatus),
                            "features" => Ok(GeneratedField::Features),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = UpdateRoomRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.UpdateRoomRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRoomRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut room_number__ = None;
                let mut floor_number__ = None;
                let mut status__ = None;
                let mut hk_status__ = None;
                let mut features__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomNumber => {
                            if room_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomNumber"));
                            }
                            room_number__ = map_.next_value()?;
                        }
                        GeneratedField::FloorNumber => {
                            if floor_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floorNumber"));
                            }
                            floor_number__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::HkStatus => {
                            if hk_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hkStatus"));
                            }
                            hk_status__ = map_.next_value()?;
                        }
                        GeneratedField::Features => {
                            if features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            features__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRoomRequest {
                    id: id__.unwrap_or_default(),
                    room_number: room_number__,
                    floor_number: floor_number__,
                    status: status__,
                    hk_status: hk_status__,
                    features: features__,
                    is_active: is_active__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.UpdateRoomRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRoomStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.room_id.is_empty() {
            len += 1;
        }
        if !self.new_status.is_empty() {
            len += 1;
        }
        if self.new_hk_status.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.UpdateRoomStatusRequest", len)?;
        if !self.room_id.is_empty() {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        if !self.new_status.is_empty() {
            struct_ser.serialize_field("newStatus", &self.new_status)?;
        }
        if let Some(v) = self.new_hk_status.as_ref() {
            struct_ser.serialize_field("newHkStatus", v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRoomStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_id",
            "roomId",
            "new_status",
            "newStatus",
            "new_hk_status",
            "newHkStatus",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
            NewStatus,
            NewHkStatus,
            Reason,
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
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            "newStatus" | "new_status" => Ok(GeneratedField::NewStatus),
                            "newHkStatus" | "new_hk_status" => Ok(GeneratedField::NewHkStatus),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRoomStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.UpdateRoomStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRoomStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                let mut new_status__ = None;
                let mut new_hk_status__ = None;
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewStatus => {
                            if new_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newStatus"));
                            }
                            new_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewHkStatus => {
                            if new_hk_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newHkStatus"));
                            }
                            new_hk_status__ = map_.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRoomStatusRequest {
                    room_id: room_id__.unwrap_or_default(),
                    new_status: new_status__.unwrap_or_default(),
                    new_hk_status: new_hk_status__,
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.UpdateRoomStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRoomTypeRequest {
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
        if self.capacity_adults.is_some() {
            len += 1;
        }
        if self.capacity_children.is_some() {
            len += 1;
        }
        if self.base_price.is_some() {
            len += 1;
        }
        if self.amenities.is_some() {
            len += 1;
        }
        if !self.image_urls.is_empty() {
            len += 1;
        }
        if self.sort_order.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.hotel.v1.UpdateRoomTypeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.capacity_adults.as_ref() {
            struct_ser.serialize_field("capacityAdults", v)?;
        }
        if let Some(v) = self.capacity_children.as_ref() {
            struct_ser.serialize_field("capacityChildren", v)?;
        }
        if let Some(v) = self.base_price.as_ref() {
            struct_ser.serialize_field("basePrice", v)?;
        }
        if let Some(v) = self.amenities.as_ref() {
            struct_ser.serialize_field("amenities", v)?;
        }
        if !self.image_urls.is_empty() {
            struct_ser.serialize_field("imageUrls", &self.image_urls)?;
        }
        if let Some(v) = self.sort_order.as_ref() {
            struct_ser.serialize_field("sortOrder", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateRoomTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "capacity_adults",
            "capacityAdults",
            "capacity_children",
            "capacityChildren",
            "base_price",
            "basePrice",
            "amenities",
            "image_urls",
            "imageUrls",
            "sort_order",
            "sortOrder",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            CapacityAdults,
            CapacityChildren,
            BasePrice,
            Amenities,
            ImageUrls,
            SortOrder,
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
                            "capacityAdults" | "capacity_adults" => Ok(GeneratedField::CapacityAdults),
                            "capacityChildren" | "capacity_children" => Ok(GeneratedField::CapacityChildren),
                            "basePrice" | "base_price" => Ok(GeneratedField::BasePrice),
                            "amenities" => Ok(GeneratedField::Amenities),
                            "imageUrls" | "image_urls" => Ok(GeneratedField::ImageUrls),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
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
            type Value = UpdateRoomTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.hotel.v1.UpdateRoomTypeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRoomTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut capacity_adults__ = None;
                let mut capacity_children__ = None;
                let mut base_price__ = None;
                let mut amenities__ = None;
                let mut image_urls__ = None;
                let mut sort_order__ = None;
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
                        GeneratedField::CapacityAdults => {
                            if capacity_adults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacityAdults"));
                            }
                            capacity_adults__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CapacityChildren => {
                            if capacity_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacityChildren"));
                            }
                            capacity_children__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BasePrice => {
                            if base_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basePrice"));
                            }
                            base_price__ = map_.next_value()?;
                        }
                        GeneratedField::Amenities => {
                            if amenities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amenities"));
                            }
                            amenities__ = map_.next_value()?;
                        }
                        GeneratedField::ImageUrls => {
                            if image_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrls"));
                            }
                            image_urls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortOrder => {
                            if sort_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortOrder"));
                            }
                            sort_order__ = 
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
                Ok(UpdateRoomTypeRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    capacity_adults: capacity_adults__,
                    capacity_children: capacity_children__,
                    base_price: base_price__,
                    amenities: amenities__,
                    image_urls: image_urls__.unwrap_or_default(),
                    sort_order: sort_order__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.hotel.v1.UpdateRoomTypeRequest", FIELDS, GeneratedVisitor)
    }
}
