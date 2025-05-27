// @generated
impl serde::Serialize for CloseOrderSessionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_id.is_empty() {
            len += 1;
        }
        if self.closed_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.CloseOrderSessionRequest", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if let Some(v) = self.closed_by_user_id.as_ref() {
            struct_ser.serialize_field("closedByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloseOrderSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "closed_by_user_id",
            "closedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            ClosedByUserId,
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
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            "closedByUserId" | "closed_by_user_id" => Ok(GeneratedField::ClosedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloseOrderSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.CloseOrderSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloseOrderSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut closed_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClosedByUserId => {
                            if closed_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closedByUserId"));
                            }
                            closed_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CloseOrderSessionRequest {
                    session_id: session_id__.unwrap_or_default(),
                    closed_by_user_id: closed_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.CloseOrderSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFloorPlanSectionRequest {
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
        if self.code.is_some() {
            len += 1;
        }
        if self.layout_map.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.CreateFloorPlanSectionRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.layout_map.as_ref() {
            struct_ser.serialize_field("layoutMap", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateFloorPlanSectionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "code",
            "layout_map",
            "layoutMap",
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
            LayoutMap,
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
                            "layoutMap" | "layout_map" => Ok(GeneratedField::LayoutMap),
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
            type Value = CreateFloorPlanSectionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.CreateFloorPlanSectionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFloorPlanSectionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut code__ = None;
                let mut layout_map__ = None;
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
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::LayoutMap => {
                            if layout_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layoutMap"));
                            }
                            layout_map__ = map_.next_value()?;
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
                Ok(CreateFloorPlanSectionRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    code: code__,
                    layout_map: layout_map__,
                    sort_order: sort_order__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.CreateFloorPlanSectionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrderSessionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_id.is_some() {
            len += 1;
        }
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if self.guest_count.is_some() {
            len += 1;
        }
        if self.opened_by_user_id.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.CreateOrderSessionRequest", len)?;
        if let Some(v) = self.table_id.as_ref() {
            struct_ser.serialize_field("tableId", v)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.guest_count.as_ref() {
            struct_ser.serialize_field("guestCount", v)?;
        }
        if let Some(v) = self.opened_by_user_id.as_ref() {
            struct_ser.serialize_field("openedByUserId", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrderSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_id",
            "tableId",
            "warehouse_id",
            "warehouseId",
            "guest_count",
            "guestCount",
            "opened_by_user_id",
            "openedByUserId",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableId,
            WarehouseId,
            GuestCount,
            OpenedByUserId,
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
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "guestCount" | "guest_count" => Ok(GeneratedField::GuestCount),
                            "openedByUserId" | "opened_by_user_id" => Ok(GeneratedField::OpenedByUserId),
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
            type Value = CreateOrderSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.CreateOrderSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrderSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_id__ = None;
                let mut warehouse_id__ = None;
                let mut guest_count__ = None;
                let mut opened_by_user_id__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GuestCount => {
                            if guest_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("guestCount"));
                            }
                            guest_count__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OpenedByUserId => {
                            if opened_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openedByUserId"));
                            }
                            opened_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOrderSessionRequest {
                    table_id: table_id__,
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    guest_count: guest_count__,
                    opened_by_user_id: opened_by_user_id__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.CreateOrderSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRestaurantTableRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.section_id.is_empty() {
            len += 1;
        }
        if !self.table_number.is_empty() {
            len += 1;
        }
        if self.capacity != 0 {
            len += 1;
        }
        if self.min_capacity.is_some() {
            len += 1;
        }
        if self.shape.is_some() {
            len += 1;
        }
        if self.pos_x.is_some() {
            len += 1;
        }
        if self.pos_y.is_some() {
            len += 1;
        }
        if self.width.is_some() {
            len += 1;
        }
        if self.height.is_some() {
            len += 1;
        }
        if self.rotation.is_some() {
            len += 1;
        }
        if self.is_mergeable.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.CreateRestaurantTableRequest", len)?;
        if !self.section_id.is_empty() {
            struct_ser.serialize_field("sectionId", &self.section_id)?;
        }
        if !self.table_number.is_empty() {
            struct_ser.serialize_field("tableNumber", &self.table_number)?;
        }
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if let Some(v) = self.min_capacity.as_ref() {
            struct_ser.serialize_field("minCapacity", v)?;
        }
        if let Some(v) = self.shape.as_ref() {
            struct_ser.serialize_field("shape", v)?;
        }
        if let Some(v) = self.pos_x.as_ref() {
            struct_ser.serialize_field("posX", v)?;
        }
        if let Some(v) = self.pos_y.as_ref() {
            struct_ser.serialize_field("posY", v)?;
        }
        if let Some(v) = self.width.as_ref() {
            struct_ser.serialize_field("width", v)?;
        }
        if let Some(v) = self.height.as_ref() {
            struct_ser.serialize_field("height", v)?;
        }
        if let Some(v) = self.rotation.as_ref() {
            struct_ser.serialize_field("rotation", v)?;
        }
        if let Some(v) = self.is_mergeable.as_ref() {
            struct_ser.serialize_field("isMergeable", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateRestaurantTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "section_id",
            "sectionId",
            "table_number",
            "tableNumber",
            "capacity",
            "min_capacity",
            "minCapacity",
            "shape",
            "pos_x",
            "posX",
            "pos_y",
            "posY",
            "width",
            "height",
            "rotation",
            "is_mergeable",
            "isMergeable",
            "is_active",
            "isActive",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SectionId,
            TableNumber,
            Capacity,
            MinCapacity,
            Shape,
            PosX,
            PosY,
            Width,
            Height,
            Rotation,
            IsMergeable,
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
                            "sectionId" | "section_id" => Ok(GeneratedField::SectionId),
                            "tableNumber" | "table_number" => Ok(GeneratedField::TableNumber),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "minCapacity" | "min_capacity" => Ok(GeneratedField::MinCapacity),
                            "shape" => Ok(GeneratedField::Shape),
                            "posX" | "pos_x" => Ok(GeneratedField::PosX),
                            "posY" | "pos_y" => Ok(GeneratedField::PosY),
                            "width" => Ok(GeneratedField::Width),
                            "height" => Ok(GeneratedField::Height),
                            "rotation" => Ok(GeneratedField::Rotation),
                            "isMergeable" | "is_mergeable" => Ok(GeneratedField::IsMergeable),
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
            type Value = CreateRestaurantTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.CreateRestaurantTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRestaurantTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut section_id__ = None;
                let mut table_number__ = None;
                let mut capacity__ = None;
                let mut min_capacity__ = None;
                let mut shape__ = None;
                let mut pos_x__ = None;
                let mut pos_y__ = None;
                let mut width__ = None;
                let mut height__ = None;
                let mut rotation__ = None;
                let mut is_mergeable__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SectionId => {
                            if section_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sectionId"));
                            }
                            section_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableNumber => {
                            if table_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableNumber"));
                            }
                            table_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinCapacity => {
                            if min_capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCapacity"));
                            }
                            min_capacity__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ = map_.next_value()?;
                        }
                        GeneratedField::PosX => {
                            if pos_x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("posX"));
                            }
                            pos_x__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PosY => {
                            if pos_y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("posY"));
                            }
                            pos_y__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Rotation => {
                            if rotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsMergeable => {
                            if is_mergeable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isMergeable"));
                            }
                            is_mergeable__ = map_.next_value()?;
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
                Ok(CreateRestaurantTableRequest {
                    section_id: section_id__.unwrap_or_default(),
                    table_number: table_number__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    min_capacity: min_capacity__,
                    shape: shape__,
                    pos_x: pos_x__,
                    pos_y: pos_y__,
                    width: width__,
                    height: height__,
                    rotation: rotation__,
                    is_mergeable: is_mergeable__,
                    is_active: is_active__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.CreateRestaurantTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloorPlanSection {
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
        if self.code.is_some() {
            len += 1;
        }
        if self.layout_map.is_some() {
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
        if !self.all_translations.is_empty() {
            len += 1;
        }
        if !self.tables.is_empty() {
            len += 1;
        }
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.FloorPlanSection", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.layout_map.as_ref() {
            struct_ser.serialize_field("layoutMap", v)?;
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
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloorPlanSection {
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
            "layout_map",
            "layoutMap",
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
            "all_translations",
            "allTranslations",
            "tables",
            "warehouse_summary",
            "warehouseSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            Code,
            LayoutMap,
            SortOrder,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            AllTranslations,
            Tables,
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
                            "layoutMap" | "layout_map" => Ok(GeneratedField::LayoutMap),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "allTranslations" | "all_translations" => Ok(GeneratedField::AllTranslations),
                            "tables" => Ok(GeneratedField::Tables),
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
            type Value = FloorPlanSection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.FloorPlanSection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FloorPlanSection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut code__ = None;
                let mut layout_map__ = None;
                let mut sort_order__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut all_translations__ = None;
                let mut tables__ = None;
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
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::LayoutMap => {
                            if layout_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layoutMap"));
                            }
                            layout_map__ = map_.next_value()?;
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
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FloorPlanSection {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    code: code__,
                    layout_map: layout_map__,
                    sort_order: sort_order__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                    tables: tables__.unwrap_or_default(),
                    warehouse_summary: warehouse_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.FloorPlanSection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFloorPlanSectionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.section.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.GetFloorPlanSectionResponse", len)?;
        if let Some(v) = self.section.as_ref() {
            struct_ser.serialize_field("section", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFloorPlanSectionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "section",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Section,
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
                            "section" => Ok(GeneratedField::Section),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFloorPlanSectionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.GetFloorPlanSectionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFloorPlanSectionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut section__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Section => {
                            if section__.is_some() {
                                return Err(serde::de::Error::duplicate_field("section"));
                            }
                            section__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetFloorPlanSectionResponse {
                    section: section__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.GetFloorPlanSectionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrderSessionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.session.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.GetOrderSessionResponse", len)?;
        if let Some(v) = self.session.as_ref() {
            struct_ser.serialize_field("session", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrderSessionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Session,
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
                            "session" => Ok(GeneratedField::Session),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrderSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.GetOrderSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrderSessionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Session => {
                            if session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("session"));
                            }
                            session__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOrderSessionResponse {
                    session: session__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.GetOrderSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRestaurantTableResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.GetRestaurantTableResponse", len)?;
        if let Some(v) = self.table.as_ref() {
            struct_ser.serialize_field("table", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRestaurantTableResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Table,
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
                            "table" => Ok(GeneratedField::Table),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRestaurantTableResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.GetRestaurantTableResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRestaurantTableResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRestaurantTableResponse {
                    table: table__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.GetRestaurantTableResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFloorPlanSectionsRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.ListFloorPlanSectionsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if !self.filter_by_warehouse_id.is_empty() {
            struct_ser.serialize_field("filterByWarehouseId", &self.filter_by_warehouse_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFloorPlanSectionsRequest {
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
            type Value = ListFloorPlanSectionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.ListFloorPlanSectionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFloorPlanSectionsRequest, V::Error>
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
                Ok(ListFloorPlanSectionsRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.ListFloorPlanSectionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFloorPlanSectionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sections.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.ListFloorPlanSectionsResponse", len)?;
        if !self.sections.is_empty() {
            struct_ser.serialize_field("sections", &self.sections)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFloorPlanSectionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sections",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sections,
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
                            "sections" => Ok(GeneratedField::Sections),
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
            type Value = ListFloorPlanSectionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.ListFloorPlanSectionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFloorPlanSectionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sections__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sections => {
                            if sections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sections"));
                            }
                            sections__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListFloorPlanSectionsResponse {
                    sections: sections__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.ListFloorPlanSectionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrderSessionsRequest {
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
        if self.filter_by_table_id.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        if self.filter_by_opened_date_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.ListOrderSessionsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_table_id.as_ref() {
            struct_ser.serialize_field("filterByTableId", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        if let Some(v) = self.filter_by_opened_date_range.as_ref() {
            struct_ser.serialize_field("filterByOpenedDateRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrderSessionsRequest {
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
            "filter_by_table_id",
            "filterByTableId",
            "filter_by_status",
            "filterByStatus",
            "filter_by_opened_date_range",
            "filterByOpenedDateRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByTableId,
            FilterByStatus,
            FilterByOpenedDateRange,
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
                            "filterByTableId" | "filter_by_table_id" => Ok(GeneratedField::FilterByTableId),
                            "filterByStatus" | "filter_by_status" => Ok(GeneratedField::FilterByStatus),
                            "filterByOpenedDateRange" | "filter_by_opened_date_range" => Ok(GeneratedField::FilterByOpenedDateRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrderSessionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.ListOrderSessionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrderSessionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_table_id__ = None;
                let mut filter_by_status__ = None;
                let mut filter_by_opened_date_range__ = None;
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
                        GeneratedField::FilterByTableId => {
                            if filter_by_table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByTableId"));
                            }
                            filter_by_table_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByOpenedDateRange => {
                            if filter_by_opened_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByOpenedDateRange"));
                            }
                            filter_by_opened_date_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOrderSessionsRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_table_id: filter_by_table_id__,
                    filter_by_status: filter_by_status__,
                    filter_by_opened_date_range: filter_by_opened_date_range__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.ListOrderSessionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrderSessionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sessions.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.ListOrderSessionsResponse", len)?;
        if !self.sessions.is_empty() {
            struct_ser.serialize_field("sessions", &self.sessions)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrderSessionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sessions",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sessions,
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
                            "sessions" => Ok(GeneratedField::Sessions),
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
            type Value = ListOrderSessionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.ListOrderSessionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrderSessionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sessions__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sessions => {
                            if sessions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessions"));
                            }
                            sessions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOrderSessionsResponse {
                    sessions: sessions__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.ListOrderSessionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRestaurantTablesRequest {
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
        if self.filter_by_section_id.is_some() {
            len += 1;
        }
        if self.filter_by_warehouse_id.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.ListRestaurantTablesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_section_id.as_ref() {
            struct_ser.serialize_field("filterBySectionId", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRestaurantTablesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_section_id",
            "filterBySectionId",
            "filter_by_warehouse_id",
            "filterByWarehouseId",
            "filter_by_status",
            "filterByStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterBySectionId,
            FilterByWarehouseId,
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
                            "filterBySectionId" | "filter_by_section_id" => Ok(GeneratedField::FilterBySectionId),
                            "filterByWarehouseId" | "filter_by_warehouse_id" => Ok(GeneratedField::FilterByWarehouseId),
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
            type Value = ListRestaurantTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.ListRestaurantTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRestaurantTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_section_id__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterBySectionId => {
                            if filter_by_section_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterBySectionId"));
                            }
                            filter_by_section_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByWarehouseId => {
                            if filter_by_warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByWarehouseId"));
                            }
                            filter_by_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListRestaurantTablesRequest {
                    base_request: base_request__,
                    filter_by_section_id: filter_by_section_id__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_status: filter_by_status__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.ListRestaurantTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRestaurantTablesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tables.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.ListRestaurantTablesResponse", len)?;
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRestaurantTablesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tables",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tables,
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
                            "tables" => Ok(GeneratedField::Tables),
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
            type Value = ListRestaurantTablesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.ListRestaurantTablesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRestaurantTablesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tables__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListRestaurantTablesResponse {
                    tables: tables__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.ListRestaurantTablesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MergeRestaurantTablesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.primary_table_id.is_empty() {
            len += 1;
        }
        if !self.secondary_table_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.MergeRestaurantTablesRequest", len)?;
        if !self.primary_table_id.is_empty() {
            struct_ser.serialize_field("primaryTableId", &self.primary_table_id)?;
        }
        if !self.secondary_table_ids.is_empty() {
            struct_ser.serialize_field("secondaryTableIds", &self.secondary_table_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MergeRestaurantTablesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "primary_table_id",
            "primaryTableId",
            "secondary_table_ids",
            "secondaryTableIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrimaryTableId,
            SecondaryTableIds,
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
                            "primaryTableId" | "primary_table_id" => Ok(GeneratedField::PrimaryTableId),
                            "secondaryTableIds" | "secondary_table_ids" => Ok(GeneratedField::SecondaryTableIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MergeRestaurantTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.MergeRestaurantTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MergeRestaurantTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut primary_table_id__ = None;
                let mut secondary_table_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrimaryTableId => {
                            if primary_table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryTableId"));
                            }
                            primary_table_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SecondaryTableIds => {
                            if secondary_table_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryTableIds"));
                            }
                            secondary_table_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MergeRestaurantTablesRequest {
                    primary_table_id: primary_table_id__.unwrap_or_default(),
                    secondary_table_ids: secondary_table_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.MergeRestaurantTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderSession {
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
        if self.table_id.is_some() {
            len += 1;
        }
        if !self.warehouse_id.is_empty() {
            len += 1;
        }
        if self.session_code.is_some() {
            len += 1;
        }
        if self.guest_count.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.opened_by_user_id.is_some() {
            len += 1;
        }
        if self.closed_by_user_id.is_some() {
            len += 1;
        }
        if self.opened_at.is_some() {
            len += 1;
        }
        if self.closed_at.is_some() {
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
        if self.table_details.is_some() {
            len += 1;
        }
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.OrderSession", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.table_id.as_ref() {
            struct_ser.serialize_field("tableId", v)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.session_code.as_ref() {
            struct_ser.serialize_field("sessionCode", v)?;
        }
        if let Some(v) = self.guest_count.as_ref() {
            struct_ser.serialize_field("guestCount", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.opened_by_user_id.as_ref() {
            struct_ser.serialize_field("openedByUserId", v)?;
        }
        if let Some(v) = self.closed_by_user_id.as_ref() {
            struct_ser.serialize_field("closedByUserId", v)?;
        }
        if let Some(v) = self.opened_at.as_ref() {
            struct_ser.serialize_field("openedAt", v)?;
        }
        if let Some(v) = self.closed_at.as_ref() {
            struct_ser.serialize_field("closedAt", v)?;
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
        if let Some(v) = self.table_details.as_ref() {
            struct_ser.serialize_field("tableDetails", v)?;
        }
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderSession {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "table_id",
            "tableId",
            "warehouse_id",
            "warehouseId",
            "session_code",
            "sessionCode",
            "guest_count",
            "guestCount",
            "status",
            "opened_by_user_id",
            "openedByUserId",
            "closed_by_user_id",
            "closedByUserId",
            "opened_at",
            "openedAt",
            "closed_at",
            "closedAt",
            "notes",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "table_details",
            "tableDetails",
            "warehouse_summary",
            "warehouseSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TableId,
            WarehouseId,
            SessionCode,
            GuestCount,
            Status,
            OpenedByUserId,
            ClosedByUserId,
            OpenedAt,
            ClosedAt,
            Notes,
            CreatedAt,
            UpdatedAt,
            TableDetails,
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
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "sessionCode" | "session_code" => Ok(GeneratedField::SessionCode),
                            "guestCount" | "guest_count" => Ok(GeneratedField::GuestCount),
                            "status" => Ok(GeneratedField::Status),
                            "openedByUserId" | "opened_by_user_id" => Ok(GeneratedField::OpenedByUserId),
                            "closedByUserId" | "closed_by_user_id" => Ok(GeneratedField::ClosedByUserId),
                            "openedAt" | "opened_at" => Ok(GeneratedField::OpenedAt),
                            "closedAt" | "closed_at" => Ok(GeneratedField::ClosedAt),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "tableDetails" | "table_details" => Ok(GeneratedField::TableDetails),
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
            type Value = OrderSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.OrderSession")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderSession, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut table_id__ = None;
                let mut warehouse_id__ = None;
                let mut session_code__ = None;
                let mut guest_count__ = None;
                let mut status__ = None;
                let mut opened_by_user_id__ = None;
                let mut closed_by_user_id__ = None;
                let mut opened_at__ = None;
                let mut closed_at__ = None;
                let mut notes__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut table_details__ = None;
                let mut warehouse_summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionCode => {
                            if session_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionCode"));
                            }
                            session_code__ = map_.next_value()?;
                        }
                        GeneratedField::GuestCount => {
                            if guest_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("guestCount"));
                            }
                            guest_count__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OpenedByUserId => {
                            if opened_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openedByUserId"));
                            }
                            opened_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::ClosedByUserId => {
                            if closed_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closedByUserId"));
                            }
                            closed_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::OpenedAt => {
                            if opened_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openedAt"));
                            }
                            opened_at__ = map_.next_value()?;
                        }
                        GeneratedField::ClosedAt => {
                            if closed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closedAt"));
                            }
                            closed_at__ = map_.next_value()?;
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
                        GeneratedField::TableDetails => {
                            if table_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableDetails"));
                            }
                            table_details__ = map_.next_value()?;
                        }
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrderSession {
                    id: id__.unwrap_or_default(),
                    table_id: table_id__,
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    session_code: session_code__,
                    guest_count: guest_count__,
                    status: status__.unwrap_or_default(),
                    opened_by_user_id: opened_by_user_id__,
                    closed_by_user_id: closed_by_user_id__,
                    opened_at: opened_at__,
                    closed_at: closed_at__,
                    notes: notes__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    table_details: table_details__,
                    warehouse_summary: warehouse_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.OrderSession", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestaurantTable {
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
        if !self.section_id.is_empty() {
            len += 1;
        }
        if !self.table_number.is_empty() {
            len += 1;
        }
        if self.capacity != 0 {
            len += 1;
        }
        if self.min_capacity.is_some() {
            len += 1;
        }
        if self.shape.is_some() {
            len += 1;
        }
        if self.pos_x.is_some() {
            len += 1;
        }
        if self.pos_y.is_some() {
            len += 1;
        }
        if self.width.is_some() {
            len += 1;
        }
        if self.height.is_some() {
            len += 1;
        }
        if self.rotation.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.is_mergeable {
            len += 1;
        }
        if self.merged_into_table_id.is_some() {
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
        if self.section_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.RestaurantTable", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.section_id.is_empty() {
            struct_ser.serialize_field("sectionId", &self.section_id)?;
        }
        if !self.table_number.is_empty() {
            struct_ser.serialize_field("tableNumber", &self.table_number)?;
        }
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if let Some(v) = self.min_capacity.as_ref() {
            struct_ser.serialize_field("minCapacity", v)?;
        }
        if let Some(v) = self.shape.as_ref() {
            struct_ser.serialize_field("shape", v)?;
        }
        if let Some(v) = self.pos_x.as_ref() {
            struct_ser.serialize_field("posX", v)?;
        }
        if let Some(v) = self.pos_y.as_ref() {
            struct_ser.serialize_field("posY", v)?;
        }
        if let Some(v) = self.width.as_ref() {
            struct_ser.serialize_field("width", v)?;
        }
        if let Some(v) = self.height.as_ref() {
            struct_ser.serialize_field("height", v)?;
        }
        if let Some(v) = self.rotation.as_ref() {
            struct_ser.serialize_field("rotation", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.is_mergeable {
            struct_ser.serialize_field("isMergeable", &self.is_mergeable)?;
        }
        if let Some(v) = self.merged_into_table_id.as_ref() {
            struct_ser.serialize_field("mergedIntoTableId", v)?;
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
        if let Some(v) = self.section_summary.as_ref() {
            struct_ser.serialize_field("sectionSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestaurantTable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "section_id",
            "sectionId",
            "table_number",
            "tableNumber",
            "capacity",
            "min_capacity",
            "minCapacity",
            "shape",
            "pos_x",
            "posX",
            "pos_y",
            "posY",
            "width",
            "height",
            "rotation",
            "status",
            "is_mergeable",
            "isMergeable",
            "merged_into_table_id",
            "mergedIntoTableId",
            "is_active",
            "isActive",
            "notes",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "section_summary",
            "sectionSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            SectionId,
            TableNumber,
            Capacity,
            MinCapacity,
            Shape,
            PosX,
            PosY,
            Width,
            Height,
            Rotation,
            Status,
            IsMergeable,
            MergedIntoTableId,
            IsActive,
            Notes,
            CreatedAt,
            UpdatedAt,
            SectionSummary,
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
                            "sectionId" | "section_id" => Ok(GeneratedField::SectionId),
                            "tableNumber" | "table_number" => Ok(GeneratedField::TableNumber),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "minCapacity" | "min_capacity" => Ok(GeneratedField::MinCapacity),
                            "shape" => Ok(GeneratedField::Shape),
                            "posX" | "pos_x" => Ok(GeneratedField::PosX),
                            "posY" | "pos_y" => Ok(GeneratedField::PosY),
                            "width" => Ok(GeneratedField::Width),
                            "height" => Ok(GeneratedField::Height),
                            "rotation" => Ok(GeneratedField::Rotation),
                            "status" => Ok(GeneratedField::Status),
                            "isMergeable" | "is_mergeable" => Ok(GeneratedField::IsMergeable),
                            "mergedIntoTableId" | "merged_into_table_id" => Ok(GeneratedField::MergedIntoTableId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "sectionSummary" | "section_summary" => Ok(GeneratedField::SectionSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestaurantTable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.RestaurantTable")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RestaurantTable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut section_id__ = None;
                let mut table_number__ = None;
                let mut capacity__ = None;
                let mut min_capacity__ = None;
                let mut shape__ = None;
                let mut pos_x__ = None;
                let mut pos_y__ = None;
                let mut width__ = None;
                let mut height__ = None;
                let mut rotation__ = None;
                let mut status__ = None;
                let mut is_mergeable__ = None;
                let mut merged_into_table_id__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut section_summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SectionId => {
                            if section_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sectionId"));
                            }
                            section_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableNumber => {
                            if table_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableNumber"));
                            }
                            table_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinCapacity => {
                            if min_capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCapacity"));
                            }
                            min_capacity__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ = map_.next_value()?;
                        }
                        GeneratedField::PosX => {
                            if pos_x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("posX"));
                            }
                            pos_x__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PosY => {
                            if pos_y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("posY"));
                            }
                            pos_y__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Rotation => {
                            if rotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsMergeable => {
                            if is_mergeable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isMergeable"));
                            }
                            is_mergeable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MergedIntoTableId => {
                            if merged_into_table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergedIntoTableId"));
                            }
                            merged_into_table_id__ = map_.next_value()?;
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
                        GeneratedField::SectionSummary => {
                            if section_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sectionSummary"));
                            }
                            section_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RestaurantTable {
                    id: id__.unwrap_or_default(),
                    section_id: section_id__.unwrap_or_default(),
                    table_number: table_number__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    min_capacity: min_capacity__,
                    shape: shape__,
                    pos_x: pos_x__,
                    pos_y: pos_y__,
                    width: width__,
                    height: height__,
                    rotation: rotation__,
                    status: status__.unwrap_or_default(),
                    is_mergeable: is_mergeable__.unwrap_or_default(),
                    merged_into_table_id: merged_into_table_id__,
                    is_active: is_active__.unwrap_or_default(),
                    notes: notes__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    section_summary: section_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.RestaurantTable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SeparateRestaurantTablesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previously_merged_table_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.SeparateRestaurantTablesRequest", len)?;
        if !self.previously_merged_table_id.is_empty() {
            struct_ser.serialize_field("previouslyMergedTableId", &self.previously_merged_table_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SeparateRestaurantTablesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previously_merged_table_id",
            "previouslyMergedTableId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviouslyMergedTableId,
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
                            "previouslyMergedTableId" | "previously_merged_table_id" => Ok(GeneratedField::PreviouslyMergedTableId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SeparateRestaurantTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.SeparateRestaurantTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SeparateRestaurantTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut previously_merged_table_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviouslyMergedTableId => {
                            if previously_merged_table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previouslyMergedTableId"));
                            }
                            previously_merged_table_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SeparateRestaurantTablesRequest {
                    previously_merged_table_id: previously_merged_table_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.SeparateRestaurantTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFloorPlanSectionRequest {
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
        if self.layout_map.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.UpdateFloorPlanSectionRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.layout_map.as_ref() {
            struct_ser.serialize_field("layoutMap", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateFloorPlanSectionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "layout_map",
            "layoutMap",
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
            LayoutMap,
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
                            "layoutMap" | "layout_map" => Ok(GeneratedField::LayoutMap),
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
            type Value = UpdateFloorPlanSectionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.UpdateFloorPlanSectionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateFloorPlanSectionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut layout_map__ = None;
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
                        GeneratedField::LayoutMap => {
                            if layout_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layoutMap"));
                            }
                            layout_map__ = map_.next_value()?;
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
                Ok(UpdateFloorPlanSectionRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    layout_map: layout_map__,
                    sort_order: sort_order__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.UpdateFloorPlanSectionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrderSessionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_id.is_empty() {
            len += 1;
        }
        if self.table_id.is_some() {
            len += 1;
        }
        if self.guest_count.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.UpdateOrderSessionRequest", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if let Some(v) = self.table_id.as_ref() {
            struct_ser.serialize_field("tableId", v)?;
        }
        if let Some(v) = self.guest_count.as_ref() {
            struct_ser.serialize_field("guestCount", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrderSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "table_id",
            "tableId",
            "guest_count",
            "guestCount",
            "status",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            TableId,
            GuestCount,
            Status,
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
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            "guestCount" | "guest_count" => Ok(GeneratedField::GuestCount),
                            "status" => Ok(GeneratedField::Status),
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
            type Value = UpdateOrderSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.UpdateOrderSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrderSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut table_id__ = None;
                let mut guest_count__ = None;
                let mut status__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = map_.next_value()?;
                        }
                        GeneratedField::GuestCount => {
                            if guest_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("guestCount"));
                            }
                            guest_count__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrderSessionRequest {
                    session_id: session_id__.unwrap_or_default(),
                    table_id: table_id__,
                    guest_count: guest_count__,
                    status: status__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.UpdateOrderSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRestaurantTableRequest {
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
        if self.table_number.is_some() {
            len += 1;
        }
        if self.capacity.is_some() {
            len += 1;
        }
        if self.min_capacity.is_some() {
            len += 1;
        }
        if self.shape.is_some() {
            len += 1;
        }
        if self.pos_x.is_some() {
            len += 1;
        }
        if self.pos_y.is_some() {
            len += 1;
        }
        if self.width.is_some() {
            len += 1;
        }
        if self.height.is_some() {
            len += 1;
        }
        if self.rotation.is_some() {
            len += 1;
        }
        if self.is_mergeable.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.UpdateRestaurantTableRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.table_number.as_ref() {
            struct_ser.serialize_field("tableNumber", v)?;
        }
        if let Some(v) = self.capacity.as_ref() {
            struct_ser.serialize_field("capacity", v)?;
        }
        if let Some(v) = self.min_capacity.as_ref() {
            struct_ser.serialize_field("minCapacity", v)?;
        }
        if let Some(v) = self.shape.as_ref() {
            struct_ser.serialize_field("shape", v)?;
        }
        if let Some(v) = self.pos_x.as_ref() {
            struct_ser.serialize_field("posX", v)?;
        }
        if let Some(v) = self.pos_y.as_ref() {
            struct_ser.serialize_field("posY", v)?;
        }
        if let Some(v) = self.width.as_ref() {
            struct_ser.serialize_field("width", v)?;
        }
        if let Some(v) = self.height.as_ref() {
            struct_ser.serialize_field("height", v)?;
        }
        if let Some(v) = self.rotation.as_ref() {
            struct_ser.serialize_field("rotation", v)?;
        }
        if let Some(v) = self.is_mergeable.as_ref() {
            struct_ser.serialize_field("isMergeable", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateRestaurantTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "table_number",
            "tableNumber",
            "capacity",
            "min_capacity",
            "minCapacity",
            "shape",
            "pos_x",
            "posX",
            "pos_y",
            "posY",
            "width",
            "height",
            "rotation",
            "is_mergeable",
            "isMergeable",
            "is_active",
            "isActive",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TableNumber,
            Capacity,
            MinCapacity,
            Shape,
            PosX,
            PosY,
            Width,
            Height,
            Rotation,
            IsMergeable,
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
                            "tableNumber" | "table_number" => Ok(GeneratedField::TableNumber),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "minCapacity" | "min_capacity" => Ok(GeneratedField::MinCapacity),
                            "shape" => Ok(GeneratedField::Shape),
                            "posX" | "pos_x" => Ok(GeneratedField::PosX),
                            "posY" | "pos_y" => Ok(GeneratedField::PosY),
                            "width" => Ok(GeneratedField::Width),
                            "height" => Ok(GeneratedField::Height),
                            "rotation" => Ok(GeneratedField::Rotation),
                            "isMergeable" | "is_mergeable" => Ok(GeneratedField::IsMergeable),
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
            type Value = UpdateRestaurantTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.UpdateRestaurantTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRestaurantTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut table_number__ = None;
                let mut capacity__ = None;
                let mut min_capacity__ = None;
                let mut shape__ = None;
                let mut pos_x__ = None;
                let mut pos_y__ = None;
                let mut width__ = None;
                let mut height__ = None;
                let mut rotation__ = None;
                let mut is_mergeable__ = None;
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
                        GeneratedField::TableNumber => {
                            if table_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableNumber"));
                            }
                            table_number__ = map_.next_value()?;
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinCapacity => {
                            if min_capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCapacity"));
                            }
                            min_capacity__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ = map_.next_value()?;
                        }
                        GeneratedField::PosX => {
                            if pos_x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("posX"));
                            }
                            pos_x__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PosY => {
                            if pos_y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("posY"));
                            }
                            pos_y__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Rotation => {
                            if rotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsMergeable => {
                            if is_mergeable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isMergeable"));
                            }
                            is_mergeable__ = map_.next_value()?;
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
                Ok(UpdateRestaurantTableRequest {
                    id: id__.unwrap_or_default(),
                    table_number: table_number__,
                    capacity: capacity__,
                    min_capacity: min_capacity__,
                    shape: shape__,
                    pos_x: pos_x__,
                    pos_y: pos_y__,
                    width: width__,
                    height: height__,
                    rotation: rotation__,
                    is_mergeable: is_mergeable__,
                    is_active: is_active__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.UpdateRestaurantTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRestaurantTableStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_id.is_empty() {
            len += 1;
        }
        if !self.new_status.is_empty() {
            len += 1;
        }
        if self.order_session_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.restaurant.v1.UpdateRestaurantTableStatusRequest", len)?;
        if !self.table_id.is_empty() {
            struct_ser.serialize_field("tableId", &self.table_id)?;
        }
        if !self.new_status.is_empty() {
            struct_ser.serialize_field("newStatus", &self.new_status)?;
        }
        if let Some(v) = self.order_session_id.as_ref() {
            struct_ser.serialize_field("orderSessionId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRestaurantTableStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_id",
            "tableId",
            "new_status",
            "newStatus",
            "order_session_id",
            "orderSessionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableId,
            NewStatus,
            OrderSessionId,
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
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            "newStatus" | "new_status" => Ok(GeneratedField::NewStatus),
                            "orderSessionId" | "order_session_id" => Ok(GeneratedField::OrderSessionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRestaurantTableStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.restaurant.v1.UpdateRestaurantTableStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRestaurantTableStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_id__ = None;
                let mut new_status__ = None;
                let mut order_session_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewStatus => {
                            if new_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newStatus"));
                            }
                            new_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderSessionId => {
                            if order_session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderSessionId"));
                            }
                            order_session_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRestaurantTableStatusRequest {
                    table_id: table_id__.unwrap_or_default(),
                    new_status: new_status__.unwrap_or_default(),
                    order_session_id: order_session_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.restaurant.v1.UpdateRestaurantTableStatusRequest", FIELDS, GeneratedVisitor)
    }
}
