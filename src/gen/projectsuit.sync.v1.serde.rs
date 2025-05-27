// @generated
impl serde::Serialize for ClientChangeItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_id.is_empty() {
            len += 1;
        }
        if self.occurred_at.is_some() {
            len += 1;
        }
        if self.command.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.ClientChangeItem", len)?;
        if !self.local_id.is_empty() {
            struct_ser.serialize_field("localId", &self.local_id)?;
        }
        if let Some(v) = self.occurred_at.as_ref() {
            struct_ser.serialize_field("occurredAt", v)?;
        }
        if let Some(v) = self.command.as_ref() {
            struct_ser.serialize_field("command", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientChangeItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_id",
            "localId",
            "occurred_at",
            "occurredAt",
            "command",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalId,
            OccurredAt,
            Command,
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
                            "localId" | "local_id" => Ok(GeneratedField::LocalId),
                            "occurredAt" | "occurred_at" => Ok(GeneratedField::OccurredAt),
                            "command" => Ok(GeneratedField::Command),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientChangeItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.ClientChangeItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientChangeItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_id__ = None;
                let mut occurred_at__ = None;
                let mut command__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalId => {
                            if local_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localId"));
                            }
                            local_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OccurredAt => {
                            if occurred_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurredAt"));
                            }
                            occurred_at__ = map_.next_value()?;
                        }
                        GeneratedField::Command => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("command"));
                            }
                            command__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClientChangeItem {
                    local_id: local_id__.unwrap_or_default(),
                    occurred_at: occurred_at__,
                    command: command__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.ClientChangeItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientChangeResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_id.is_empty() {
            len += 1;
        }
        if self.success {
            len += 1;
        }
        if self.server_id.is_some() {
            len += 1;
        }
        if self.error_message.is_some() {
            len += 1;
        }
        if self.processed_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.ClientChangeResult", len)?;
        if !self.local_id.is_empty() {
            struct_ser.serialize_field("localId", &self.local_id)?;
        }
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.server_id.as_ref() {
            struct_ser.serialize_field("serverId", v)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.processed_at.as_ref() {
            struct_ser.serialize_field("processedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientChangeResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_id",
            "localId",
            "success",
            "server_id",
            "serverId",
            "error_message",
            "errorMessage",
            "processed_at",
            "processedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalId,
            Success,
            ServerId,
            ErrorMessage,
            ProcessedAt,
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
                            "localId" | "local_id" => Ok(GeneratedField::LocalId),
                            "success" => Ok(GeneratedField::Success),
                            "serverId" | "server_id" => Ok(GeneratedField::ServerId),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "processedAt" | "processed_at" => Ok(GeneratedField::ProcessedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientChangeResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.ClientChangeResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientChangeResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_id__ = None;
                let mut success__ = None;
                let mut server_id__ = None;
                let mut error_message__ = None;
                let mut processed_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalId => {
                            if local_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localId"));
                            }
                            local_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServerId => {
                            if server_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverId"));
                            }
                            server_id__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = map_.next_value()?;
                        }
                        GeneratedField::ProcessedAt => {
                            if processed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processedAt"));
                            }
                            processed_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClientChangeResult {
                    local_id: local_id__.unwrap_or_default(),
                    success: success__.unwrap_or_default(),
                    server_id: server_id__,
                    error_message: error_message__,
                    processed_at: processed_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.ClientChangeResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandPayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.command_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.CommandPayload", len)?;
        if let Some(v) = self.command_data.as_ref() {
            match v {
                command_payload::CommandData::CreateProduct(v) => {
                    struct_ser.serialize_field("createProduct", v)?;
                }
                command_payload::CommandData::UpdateProduct(v) => {
                    struct_ser.serialize_field("updateProduct", v)?;
                }
                command_payload::CommandData::CreateOrder(v) => {
                    struct_ser.serialize_field("createOrder", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandPayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "create_product",
            "createProduct",
            "update_product",
            "updateProduct",
            "create_order",
            "createOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreateProduct,
            UpdateProduct,
            CreateOrder,
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
                            "createProduct" | "create_product" => Ok(GeneratedField::CreateProduct),
                            "updateProduct" | "update_product" => Ok(GeneratedField::UpdateProduct),
                            "createOrder" | "create_order" => Ok(GeneratedField::CreateOrder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandPayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.CommandPayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommandPayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut command_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreateProduct => {
                            if command_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createProduct"));
                            }
                            command_data__ = map_.next_value::<::std::option::Option<_>>()?.map(command_payload::CommandData::CreateProduct)
;
                        }
                        GeneratedField::UpdateProduct => {
                            if command_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateProduct"));
                            }
                            command_data__ = map_.next_value::<::std::option::Option<_>>()?.map(command_payload::CommandData::UpdateProduct)
;
                        }
                        GeneratedField::CreateOrder => {
                            if command_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createOrder"));
                            }
                            command_data__ = map_.next_value::<::std::option::Option<_>>()?.map(command_payload::CommandData::CreateOrder)
;
                        }
                    }
                }
                Ok(CommandPayload {
                    command_data: command_data__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.CommandPayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeletedEntityReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_type.is_empty() {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.deleted_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.DeletedEntityReference", len)?;
        if !self.entity_type.is_empty() {
            struct_ser.serialize_field("entityType", &self.entity_type)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if let Some(v) = self.deleted_at.as_ref() {
            struct_ser.serialize_field("deletedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeletedEntityReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_type",
            "entityType",
            "entity_id",
            "entityId",
            "deleted_at",
            "deletedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityType,
            EntityId,
            DeletedAt,
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
                            "entityType" | "entity_type" => Ok(GeneratedField::EntityType),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "deletedAt" | "deleted_at" => Ok(GeneratedField::DeletedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeletedEntityReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.DeletedEntityReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeletedEntityReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_type__ = None;
                let mut entity_id__ = None;
                let mut deleted_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityType => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityType"));
                            }
                            entity_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeletedAt => {
                            if deleted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedAt"));
                            }
                            deleted_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeletedEntityReference {
                    entity_type: entity_type__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                    deleted_at: deleted_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.DeletedEntityReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityPayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.EntityPayload", len)?;
        if let Some(v) = self.data.as_ref() {
            match v {
                entity_payload::Data::Unit(v) => {
                    struct_ser.serialize_field("unit", v)?;
                }
                entity_payload::Data::Category(v) => {
                    struct_ser.serialize_field("category", v)?;
                }
                entity_payload::Data::Product(v) => {
                    struct_ser.serialize_field("product", v)?;
                }
                entity_payload::Data::ProductUnit(v) => {
                    struct_ser.serialize_field("productUnit", v)?;
                }
                entity_payload::Data::ProductTag(v) => {
                    struct_ser.serialize_field("productTag", v)?;
                }
                entity_payload::Data::ProductOption(v) => {
                    struct_ser.serialize_field("productOption", v)?;
                }
                entity_payload::Data::ProductOptionValue(v) => {
                    struct_ser.serialize_field("productOptionValue", v)?;
                }
                entity_payload::Data::ProductVariant(v) => {
                    struct_ser.serialize_field("productVariant", v)?;
                }
                entity_payload::Data::Material(v) => {
                    struct_ser.serialize_field("material", v)?;
                }
                entity_payload::Data::ProductMaterial(v) => {
                    struct_ser.serialize_field("productMaterial", v)?;
                }
                entity_payload::Data::TaxCategory(v) => {
                    struct_ser.serialize_field("taxCategory", v)?;
                }
                entity_payload::Data::TaxRate(v) => {
                    struct_ser.serialize_field("taxRate", v)?;
                }
                entity_payload::Data::TaxAssignment(v) => {
                    struct_ser.serialize_field("taxAssignment", v)?;
                }
                entity_payload::Data::Menu(v) => {
                    struct_ser.serialize_field("menu", v)?;
                }
                entity_payload::Data::MenuItem(v) => {
                    struct_ser.serialize_field("menuItem", v)?;
                }
                entity_payload::Data::PriceList(v) => {
                    struct_ser.serialize_field("priceList", v)?;
                }
                entity_payload::Data::PriceListItem(v) => {
                    struct_ser.serialize_field("priceListItem", v)?;
                }
                entity_payload::Data::PriceListAssignment(v) => {
                    struct_ser.serialize_field("priceListAssignment", v)?;
                }
                entity_payload::Data::Promotion(v) => {
                    struct_ser.serialize_field("promotion", v)?;
                }
                entity_payload::Data::Coupon(v) => {
                    struct_ser.serialize_field("coupon", v)?;
                }
                entity_payload::Data::Order(v) => {
                    struct_ser.serialize_field("order", v)?;
                }
                entity_payload::Data::OrderItem(v) => {
                    struct_ser.serialize_field("orderItem", v)?;
                }
                entity_payload::Data::Customer(v) => {
                    struct_ser.serialize_field("customer", v)?;
                }
                entity_payload::Data::CustomerAddress(v) => {
                    struct_ser.serialize_field("customerAddress", v)?;
                }
                entity_payload::Data::Warehouse(v) => {
                    struct_ser.serialize_field("warehouse", v)?;
                }
                entity_payload::Data::StockLevel(v) => {
                    struct_ser.serialize_field("stockLevel", v)?;
                }
                entity_payload::Data::Device(v) => {
                    struct_ser.serialize_field("device", v)?;
                }
                entity_payload::Data::FloorPlanSection(v) => {
                    struct_ser.serialize_field("floorPlanSection", v)?;
                }
                entity_payload::Data::RestaurantTable(v) => {
                    struct_ser.serialize_field("restaurantTable", v)?;
                }
                entity_payload::Data::OrderSession(v) => {
                    struct_ser.serialize_field("orderSession", v)?;
                }
                entity_payload::Data::HotelServiceZone(v) => {
                    struct_ser.serialize_field("hotelServiceZone", v)?;
                }
                entity_payload::Data::RoomType(v) => {
                    struct_ser.serialize_field("roomType", v)?;
                }
                entity_payload::Data::Room(v) => {
                    struct_ser.serialize_field("room", v)?;
                }
                entity_payload::Data::Reservation(v) => {
                    struct_ser.serialize_field("reservation", v)?;
                }
                entity_payload::Data::ChartOfAccount(v) => {
                    struct_ser.serialize_field("chartOfAccount", v)?;
                }
                entity_payload::Data::Vendor(v) => {
                    struct_ser.serialize_field("vendor", v)?;
                }
                entity_payload::Data::ExpenseCategory(v) => {
                    struct_ser.serialize_field("expenseCategory", v)?;
                }
                entity_payload::Data::Expense(v) => {
                    struct_ser.serialize_field("expense", v)?;
                }
                entity_payload::Data::PaymentMethod(v) => {
                    struct_ser.serialize_field("paymentMethod", v)?;
                }
                entity_payload::Data::Payment(v) => {
                    struct_ser.serialize_field("payment", v)?;
                }
                entity_payload::Data::GenericData(v) => {
                    struct_ser.serialize_field("genericData", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityPayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unit",
            "category",
            "product",
            "product_unit",
            "productUnit",
            "product_tag",
            "productTag",
            "product_option",
            "productOption",
            "product_option_value",
            "productOptionValue",
            "product_variant",
            "productVariant",
            "material",
            "product_material",
            "productMaterial",
            "tax_category",
            "taxCategory",
            "tax_rate",
            "taxRate",
            "tax_assignment",
            "taxAssignment",
            "menu",
            "menu_item",
            "menuItem",
            "price_list",
            "priceList",
            "price_list_item",
            "priceListItem",
            "price_list_assignment",
            "priceListAssignment",
            "promotion",
            "coupon",
            "order",
            "order_item",
            "orderItem",
            "customer",
            "customer_address",
            "customerAddress",
            "warehouse",
            "stock_level",
            "stockLevel",
            "device",
            "floor_plan_section",
            "floorPlanSection",
            "restaurant_table",
            "restaurantTable",
            "order_session",
            "orderSession",
            "hotel_service_zone",
            "hotelServiceZone",
            "room_type",
            "roomType",
            "room",
            "reservation",
            "chart_of_account",
            "chartOfAccount",
            "vendor",
            "expense_category",
            "expenseCategory",
            "expense",
            "payment_method",
            "paymentMethod",
            "payment",
            "generic_data",
            "genericData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Unit,
            Category,
            Product,
            ProductUnit,
            ProductTag,
            ProductOption,
            ProductOptionValue,
            ProductVariant,
            Material,
            ProductMaterial,
            TaxCategory,
            TaxRate,
            TaxAssignment,
            Menu,
            MenuItem,
            PriceList,
            PriceListItem,
            PriceListAssignment,
            Promotion,
            Coupon,
            Order,
            OrderItem,
            Customer,
            CustomerAddress,
            Warehouse,
            StockLevel,
            Device,
            FloorPlanSection,
            RestaurantTable,
            OrderSession,
            HotelServiceZone,
            RoomType,
            Room,
            Reservation,
            ChartOfAccount,
            Vendor,
            ExpenseCategory,
            Expense,
            PaymentMethod,
            Payment,
            GenericData,
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
                            "unit" => Ok(GeneratedField::Unit),
                            "category" => Ok(GeneratedField::Category),
                            "product" => Ok(GeneratedField::Product),
                            "productUnit" | "product_unit" => Ok(GeneratedField::ProductUnit),
                            "productTag" | "product_tag" => Ok(GeneratedField::ProductTag),
                            "productOption" | "product_option" => Ok(GeneratedField::ProductOption),
                            "productOptionValue" | "product_option_value" => Ok(GeneratedField::ProductOptionValue),
                            "productVariant" | "product_variant" => Ok(GeneratedField::ProductVariant),
                            "material" => Ok(GeneratedField::Material),
                            "productMaterial" | "product_material" => Ok(GeneratedField::ProductMaterial),
                            "taxCategory" | "tax_category" => Ok(GeneratedField::TaxCategory),
                            "taxRate" | "tax_rate" => Ok(GeneratedField::TaxRate),
                            "taxAssignment" | "tax_assignment" => Ok(GeneratedField::TaxAssignment),
                            "menu" => Ok(GeneratedField::Menu),
                            "menuItem" | "menu_item" => Ok(GeneratedField::MenuItem),
                            "priceList" | "price_list" => Ok(GeneratedField::PriceList),
                            "priceListItem" | "price_list_item" => Ok(GeneratedField::PriceListItem),
                            "priceListAssignment" | "price_list_assignment" => Ok(GeneratedField::PriceListAssignment),
                            "promotion" => Ok(GeneratedField::Promotion),
                            "coupon" => Ok(GeneratedField::Coupon),
                            "order" => Ok(GeneratedField::Order),
                            "orderItem" | "order_item" => Ok(GeneratedField::OrderItem),
                            "customer" => Ok(GeneratedField::Customer),
                            "customerAddress" | "customer_address" => Ok(GeneratedField::CustomerAddress),
                            "warehouse" => Ok(GeneratedField::Warehouse),
                            "stockLevel" | "stock_level" => Ok(GeneratedField::StockLevel),
                            "device" => Ok(GeneratedField::Device),
                            "floorPlanSection" | "floor_plan_section" => Ok(GeneratedField::FloorPlanSection),
                            "restaurantTable" | "restaurant_table" => Ok(GeneratedField::RestaurantTable),
                            "orderSession" | "order_session" => Ok(GeneratedField::OrderSession),
                            "hotelServiceZone" | "hotel_service_zone" => Ok(GeneratedField::HotelServiceZone),
                            "roomType" | "room_type" => Ok(GeneratedField::RoomType),
                            "room" => Ok(GeneratedField::Room),
                            "reservation" => Ok(GeneratedField::Reservation),
                            "chartOfAccount" | "chart_of_account" => Ok(GeneratedField::ChartOfAccount),
                            "vendor" => Ok(GeneratedField::Vendor),
                            "expenseCategory" | "expense_category" => Ok(GeneratedField::ExpenseCategory),
                            "expense" => Ok(GeneratedField::Expense),
                            "paymentMethod" | "payment_method" => Ok(GeneratedField::PaymentMethod),
                            "payment" => Ok(GeneratedField::Payment),
                            "genericData" | "generic_data" => Ok(GeneratedField::GenericData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityPayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.EntityPayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityPayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Unit => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Unit)
;
                        }
                        GeneratedField::Category => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Category)
;
                        }
                        GeneratedField::Product => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Product)
;
                        }
                        GeneratedField::ProductUnit => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productUnit"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ProductUnit)
;
                        }
                        GeneratedField::ProductTag => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productTag"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ProductTag)
;
                        }
                        GeneratedField::ProductOption => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOption"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ProductOption)
;
                        }
                        GeneratedField::ProductOptionValue => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOptionValue"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ProductOptionValue)
;
                        }
                        GeneratedField::ProductVariant => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productVariant"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ProductVariant)
;
                        }
                        GeneratedField::Material => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Material)
;
                        }
                        GeneratedField::ProductMaterial => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productMaterial"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ProductMaterial)
;
                        }
                        GeneratedField::TaxCategory => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxCategory"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::TaxCategory)
;
                        }
                        GeneratedField::TaxRate => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxRate"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::TaxRate)
;
                        }
                        GeneratedField::TaxAssignment => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxAssignment"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::TaxAssignment)
;
                        }
                        GeneratedField::Menu => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menu"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Menu)
;
                        }
                        GeneratedField::MenuItem => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("menuItem"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::MenuItem)
;
                        }
                        GeneratedField::PriceList => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceList"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::PriceList)
;
                        }
                        GeneratedField::PriceListItem => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceListItem"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::PriceListItem)
;
                        }
                        GeneratedField::PriceListAssignment => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceListAssignment"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::PriceListAssignment)
;
                        }
                        GeneratedField::Promotion => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("promotion"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Promotion)
;
                        }
                        GeneratedField::Coupon => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coupon"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Coupon)
;
                        }
                        GeneratedField::Order => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Order)
;
                        }
                        GeneratedField::OrderItem => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderItem"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::OrderItem)
;
                        }
                        GeneratedField::Customer => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customer"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Customer)
;
                        }
                        GeneratedField::CustomerAddress => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerAddress"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::CustomerAddress)
;
                        }
                        GeneratedField::Warehouse => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouse"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Warehouse)
;
                        }
                        GeneratedField::StockLevel => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stockLevel"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::StockLevel)
;
                        }
                        GeneratedField::Device => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Device)
;
                        }
                        GeneratedField::FloorPlanSection => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floorPlanSection"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::FloorPlanSection)
;
                        }
                        GeneratedField::RestaurantTable => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restaurantTable"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::RestaurantTable)
;
                        }
                        GeneratedField::OrderSession => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderSession"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::OrderSession)
;
                        }
                        GeneratedField::HotelServiceZone => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hotelServiceZone"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::HotelServiceZone)
;
                        }
                        GeneratedField::RoomType => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomType"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::RoomType)
;
                        }
                        GeneratedField::Room => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("room"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Room)
;
                        }
                        GeneratedField::Reservation => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservation"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Reservation)
;
                        }
                        GeneratedField::ChartOfAccount => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chartOfAccount"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ChartOfAccount)
;
                        }
                        GeneratedField::Vendor => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendor"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Vendor)
;
                        }
                        GeneratedField::ExpenseCategory => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseCategory"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::ExpenseCategory)
;
                        }
                        GeneratedField::Expense => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expense"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Expense)
;
                        }
                        GeneratedField::PaymentMethod => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethod"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::PaymentMethod)
;
                        }
                        GeneratedField::Payment => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::Payment)
;
                        }
                        GeneratedField::GenericData => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericData"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_payload::Data::GenericData)
;
                        }
                    }
                }
                Ok(EntityPayload {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.EntityPayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PullChangesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_types.is_empty() {
            len += 1;
        }
        if self.last_sync_timestamp.is_some() {
            len += 1;
        }
        if !self.device_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.PullChangesRequest", len)?;
        if !self.entity_types.is_empty() {
            struct_ser.serialize_field("entityTypes", &self.entity_types)?;
        }
        if let Some(v) = self.last_sync_timestamp.as_ref() {
            struct_ser.serialize_field("lastSyncTimestamp", v)?;
        }
        if !self.device_id.is_empty() {
            struct_ser.serialize_field("deviceId", &self.device_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PullChangesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_types",
            "entityTypes",
            "last_sync_timestamp",
            "lastSyncTimestamp",
            "device_id",
            "deviceId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityTypes,
            LastSyncTimestamp,
            DeviceId,
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
                            "entityTypes" | "entity_types" => Ok(GeneratedField::EntityTypes),
                            "lastSyncTimestamp" | "last_sync_timestamp" => Ok(GeneratedField::LastSyncTimestamp),
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
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
            type Value = PullChangesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.PullChangesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PullChangesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_types__ = None;
                let mut last_sync_timestamp__ = None;
                let mut device_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityTypes => {
                            if entity_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityTypes"));
                            }
                            entity_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastSyncTimestamp => {
                            if last_sync_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastSyncTimestamp"));
                            }
                            last_sync_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PullChangesRequest {
                    entity_types: entity_types__.unwrap_or_default(),
                    last_sync_timestamp: last_sync_timestamp__,
                    device_id: device_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.PullChangesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PullChangesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.new_sync_timestamp.is_some() {
            len += 1;
        }
        if !self.entities_changed.is_empty() {
            len += 1;
        }
        if !self.entities_deleted.is_empty() {
            len += 1;
        }
        if self.has_more {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.PullChangesResponse", len)?;
        if let Some(v) = self.new_sync_timestamp.as_ref() {
            struct_ser.serialize_field("newSyncTimestamp", v)?;
        }
        if !self.entities_changed.is_empty() {
            struct_ser.serialize_field("entitiesChanged", &self.entities_changed)?;
        }
        if !self.entities_deleted.is_empty() {
            struct_ser.serialize_field("entitiesDeleted", &self.entities_deleted)?;
        }
        if self.has_more {
            struct_ser.serialize_field("hasMore", &self.has_more)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PullChangesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "new_sync_timestamp",
            "newSyncTimestamp",
            "entities_changed",
            "entitiesChanged",
            "entities_deleted",
            "entitiesDeleted",
            "has_more",
            "hasMore",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewSyncTimestamp,
            EntitiesChanged,
            EntitiesDeleted,
            HasMore,
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
                            "newSyncTimestamp" | "new_sync_timestamp" => Ok(GeneratedField::NewSyncTimestamp),
                            "entitiesChanged" | "entities_changed" => Ok(GeneratedField::EntitiesChanged),
                            "entitiesDeleted" | "entities_deleted" => Ok(GeneratedField::EntitiesDeleted),
                            "hasMore" | "has_more" => Ok(GeneratedField::HasMore),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PullChangesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.PullChangesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PullChangesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut new_sync_timestamp__ = None;
                let mut entities_changed__ = None;
                let mut entities_deleted__ = None;
                let mut has_more__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NewSyncTimestamp => {
                            if new_sync_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newSyncTimestamp"));
                            }
                            new_sync_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::EntitiesChanged => {
                            if entities_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entitiesChanged"));
                            }
                            entities_changed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntitiesDeleted => {
                            if entities_deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entitiesDeleted"));
                            }
                            entities_deleted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasMore => {
                            if has_more__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasMore"));
                            }
                            has_more__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PullChangesResponse {
                    new_sync_timestamp: new_sync_timestamp__,
                    entities_changed: entities_changed__.unwrap_or_default(),
                    entities_deleted: entities_deleted__.unwrap_or_default(),
                    has_more: has_more__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.PullChangesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushChangesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.device_id.is_empty() {
            len += 1;
        }
        if !self.changes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.PushChangesRequest", len)?;
        if !self.device_id.is_empty() {
            struct_ser.serialize_field("deviceId", &self.device_id)?;
        }
        if !self.changes.is_empty() {
            struct_ser.serialize_field("changes", &self.changes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushChangesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "changes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            Changes,
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
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "changes" => Ok(GeneratedField::Changes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushChangesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.PushChangesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushChangesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut changes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Changes => {
                            if changes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changes"));
                            }
                            changes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushChangesRequest {
                    device_id: device_id__.unwrap_or_default(),
                    changes: changes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.PushChangesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushChangesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.PushChangesResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushChangesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
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
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushChangesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.PushChangesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushChangesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushChangesResponse {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.PushChangesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SyncedEntityData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_type.is_empty() {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.modified_at.is_some() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.sync.v1.SyncedEntityData", len)?;
        if !self.entity_type.is_empty() {
            struct_ser.serialize_field("entityType", &self.entity_type)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if let Some(v) = self.modified_at.as_ref() {
            struct_ser.serialize_field("modifiedAt", v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SyncedEntityData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_type",
            "entityType",
            "entity_id",
            "entityId",
            "modified_at",
            "modifiedAt",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityType,
            EntityId,
            ModifiedAt,
            Payload,
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
                            "entityType" | "entity_type" => Ok(GeneratedField::EntityType),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "modifiedAt" | "modified_at" => Ok(GeneratedField::ModifiedAt),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SyncedEntityData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.sync.v1.SyncedEntityData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SyncedEntityData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_type__ = None;
                let mut entity_id__ = None;
                let mut modified_at__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityType => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityType"));
                            }
                            entity_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModifiedAt => {
                            if modified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedAt"));
                            }
                            modified_at__ = map_.next_value()?;
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SyncedEntityData {
                    entity_type: entity_type__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                    modified_at: modified_at__,
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.sync.v1.SyncedEntityData", FIELDS, GeneratedVisitor)
    }
}
