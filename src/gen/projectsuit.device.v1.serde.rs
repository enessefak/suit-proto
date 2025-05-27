// @generated
impl serde::Serialize for CreateDeviceRequest {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.model.is_some() {
            len += 1;
        }
        if self.serial_number.is_some() {
            len += 1;
        }
        if self.ip_address.is_some() {
            len += 1;
        }
        if self.mac_address.is_some() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if self.connection_type.is_some() {
            len += 1;
        }
        if self.initial_status.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.driver_details.is_some() {
            len += 1;
        }
        if self.installed_at.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.CreateDeviceRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.model.as_ref() {
            struct_ser.serialize_field("model", v)?;
        }
        if let Some(v) = self.serial_number.as_ref() {
            struct_ser.serialize_field("serialNumber", v)?;
        }
        if let Some(v) = self.ip_address.as_ref() {
            struct_ser.serialize_field("ipAddress", v)?;
        }
        if let Some(v) = self.mac_address.as_ref() {
            struct_ser.serialize_field("macAddress", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.connection_type.as_ref() {
            struct_ser.serialize_field("connectionType", v)?;
        }
        if let Some(v) = self.initial_status.as_ref() {
            struct_ser.serialize_field("initialStatus", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.driver_details.as_ref() {
            struct_ser.serialize_field("driverDetails", v)?;
        }
        if let Some(v) = self.installed_at.as_ref() {
            struct_ser.serialize_field("installedAt", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDeviceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "name",
            "type",
            "model",
            "serial_number",
            "serialNumber",
            "ip_address",
            "ipAddress",
            "mac_address",
            "macAddress",
            "port",
            "connection_type",
            "connectionType",
            "initial_status",
            "initialStatus",
            "is_active",
            "isActive",
            "configuration",
            "driver_details",
            "driverDetails",
            "installed_at",
            "installedAt",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            Name,
            Type,
            Model,
            SerialNumber,
            IpAddress,
            MacAddress,
            Port,
            ConnectionType,
            InitialStatus,
            IsActive,
            Configuration,
            DriverDetails,
            InstalledAt,
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
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "model" => Ok(GeneratedField::Model),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            "macAddress" | "mac_address" => Ok(GeneratedField::MacAddress),
                            "port" => Ok(GeneratedField::Port),
                            "connectionType" | "connection_type" => Ok(GeneratedField::ConnectionType),
                            "initialStatus" | "initial_status" => Ok(GeneratedField::InitialStatus),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "driverDetails" | "driver_details" => Ok(GeneratedField::DriverDetails),
                            "installedAt" | "installed_at" => Ok(GeneratedField::InstalledAt),
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
            type Value = CreateDeviceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.CreateDeviceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDeviceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut name__ = None;
                let mut r#type__ = None;
                let mut model__ = None;
                let mut serial_number__ = None;
                let mut ip_address__ = None;
                let mut mac_address__ = None;
                let mut port__ = None;
                let mut connection_type__ = None;
                let mut initial_status__ = None;
                let mut is_active__ = None;
                let mut configuration__ = None;
                let mut driver_details__ = None;
                let mut installed_at__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = map_.next_value()?;
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = map_.next_value()?;
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = map_.next_value()?;
                        }
                        GeneratedField::MacAddress => {
                            if mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("macAddress"));
                            }
                            mac_address__ = map_.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ConnectionType => {
                            if connection_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionType"));
                            }
                            connection_type__ = map_.next_value()?;
                        }
                        GeneratedField::InitialStatus => {
                            if initial_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStatus"));
                            }
                            initial_status__ = map_.next_value()?;
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
                        GeneratedField::DriverDetails => {
                            if driver_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driverDetails"));
                            }
                            driver_details__ = map_.next_value()?;
                        }
                        GeneratedField::InstalledAt => {
                            if installed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("installedAt"));
                            }
                            installed_at__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateDeviceRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    model: model__,
                    serial_number: serial_number__,
                    ip_address: ip_address__,
                    mac_address: mac_address__,
                    port: port__,
                    connection_type: connection_type__,
                    initial_status: initial_status__,
                    is_active: is_active__,
                    configuration: configuration__,
                    driver_details: driver_details__,
                    installed_at: installed_at__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.CreateDeviceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePrintJobRoutingRuleRequest {
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
        if self.product_category_id.is_some() {
            len += 1;
        }
        if !self.target_printer_id.is_empty() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.CreatePrintJobRoutingRuleRequest", len)?;
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.product_category_id.as_ref() {
            struct_ser.serialize_field("productCategoryId", v)?;
        }
        if !self.target_printer_id.is_empty() {
            struct_ser.serialize_field("targetPrinterId", &self.target_printer_id)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePrintJobRoutingRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "product_category_id",
            "productCategoryId",
            "target_printer_id",
            "targetPrinterId",
            "is_active",
            "isActive",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            ProductCategoryId,
            TargetPrinterId,
            IsActive,
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
                            "warehouseId" | "warehouse_id" => Ok(GeneratedField::WarehouseId),
                            "productCategoryId" | "product_category_id" => Ok(GeneratedField::ProductCategoryId),
                            "targetPrinterId" | "target_printer_id" => Ok(GeneratedField::TargetPrinterId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = CreatePrintJobRoutingRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.CreatePrintJobRoutingRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePrintJobRoutingRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut product_category_id__ = None;
                let mut target_printer_id__ = None;
                let mut is_active__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductCategoryId => {
                            if product_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCategoryId"));
                            }
                            product_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::TargetPrinterId => {
                            if target_printer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPrinterId"));
                            }
                            target_printer_id__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(CreatePrintJobRoutingRuleRequest {
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_category_id: product_category_id__,
                    target_printer_id: target_printer_id__.unwrap_or_default(),
                    is_active: is_active__,
                    priority: priority__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.CreatePrintJobRoutingRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Device {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.model.is_some() {
            len += 1;
        }
        if self.serial_number.is_some() {
            len += 1;
        }
        if self.ip_address.is_some() {
            len += 1;
        }
        if self.mac_address.is_some() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if self.connection_type.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.driver_details.is_some() {
            len += 1;
        }
        if self.last_seen_at.is_some() {
            len += 1;
        }
        if self.installed_at.is_some() {
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
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.Device", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.model.as_ref() {
            struct_ser.serialize_field("model", v)?;
        }
        if let Some(v) = self.serial_number.as_ref() {
            struct_ser.serialize_field("serialNumber", v)?;
        }
        if let Some(v) = self.ip_address.as_ref() {
            struct_ser.serialize_field("ipAddress", v)?;
        }
        if let Some(v) = self.mac_address.as_ref() {
            struct_ser.serialize_field("macAddress", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.connection_type.as_ref() {
            struct_ser.serialize_field("connectionType", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.driver_details.as_ref() {
            struct_ser.serialize_field("driverDetails", v)?;
        }
        if let Some(v) = self.last_seen_at.as_ref() {
            struct_ser.serialize_field("lastSeenAt", v)?;
        }
        if let Some(v) = self.installed_at.as_ref() {
            struct_ser.serialize_field("installedAt", v)?;
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
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Device {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "name",
            "type",
            "model",
            "serial_number",
            "serialNumber",
            "ip_address",
            "ipAddress",
            "mac_address",
            "macAddress",
            "port",
            "connection_type",
            "connectionType",
            "status",
            "is_active",
            "isActive",
            "configuration",
            "driver_details",
            "driverDetails",
            "last_seen_at",
            "lastSeenAt",
            "installed_at",
            "installedAt",
            "notes",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "warehouse_summary",
            "warehouseSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            Name,
            Type,
            Model,
            SerialNumber,
            IpAddress,
            MacAddress,
            Port,
            ConnectionType,
            Status,
            IsActive,
            Configuration,
            DriverDetails,
            LastSeenAt,
            InstalledAt,
            Notes,
            CreatedAt,
            UpdatedAt,
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
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "model" => Ok(GeneratedField::Model),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            "macAddress" | "mac_address" => Ok(GeneratedField::MacAddress),
                            "port" => Ok(GeneratedField::Port),
                            "connectionType" | "connection_type" => Ok(GeneratedField::ConnectionType),
                            "status" => Ok(GeneratedField::Status),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "driverDetails" | "driver_details" => Ok(GeneratedField::DriverDetails),
                            "lastSeenAt" | "last_seen_at" => Ok(GeneratedField::LastSeenAt),
                            "installedAt" | "installed_at" => Ok(GeneratedField::InstalledAt),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
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
            type Value = Device;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.Device")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Device, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut name__ = None;
                let mut r#type__ = None;
                let mut model__ = None;
                let mut serial_number__ = None;
                let mut ip_address__ = None;
                let mut mac_address__ = None;
                let mut port__ = None;
                let mut connection_type__ = None;
                let mut status__ = None;
                let mut is_active__ = None;
                let mut configuration__ = None;
                let mut driver_details__ = None;
                let mut last_seen_at__ = None;
                let mut installed_at__ = None;
                let mut notes__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = map_.next_value()?;
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = map_.next_value()?;
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = map_.next_value()?;
                        }
                        GeneratedField::MacAddress => {
                            if mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("macAddress"));
                            }
                            mac_address__ = map_.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ConnectionType => {
                            if connection_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionType"));
                            }
                            connection_type__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
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
                        GeneratedField::DriverDetails => {
                            if driver_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driverDetails"));
                            }
                            driver_details__ = map_.next_value()?;
                        }
                        GeneratedField::LastSeenAt => {
                            if last_seen_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastSeenAt"));
                            }
                            last_seen_at__ = map_.next_value()?;
                        }
                        GeneratedField::InstalledAt => {
                            if installed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("installedAt"));
                            }
                            installed_at__ = map_.next_value()?;
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
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Device {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    model: model__,
                    serial_number: serial_number__,
                    ip_address: ip_address__,
                    mac_address: mac_address__,
                    port: port__,
                    connection_type: connection_type__,
                    status: status__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    configuration: configuration__,
                    driver_details: driver_details__,
                    last_seen_at: last_seen_at__,
                    installed_at: installed_at__,
                    notes: notes__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    warehouse_summary: warehouse_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.Device", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDeviceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.GetDeviceResponse", len)?;
        if let Some(v) = self.device.as_ref() {
            struct_ser.serialize_field("device", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDeviceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Device,
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
                            "device" => Ok(GeneratedField::Device),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDeviceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.GetDeviceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDeviceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Device => {
                            if device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            device__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDeviceResponse {
                    device: device__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.GetDeviceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDevicesRequest {
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
        if self.filter_by_type.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.ListDevicesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_type.as_ref() {
            struct_ser.serialize_field("filterByType", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDevicesRequest {
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
            "filter_by_type",
            "filterByType",
            "filter_by_status",
            "filterByStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByType,
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
                            "filterByType" | "filter_by_type" => Ok(GeneratedField::FilterByType),
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
            type Value = ListDevicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.ListDevicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDevicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_type__ = None;
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
                            filter_by_warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByType => {
                            if filter_by_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByType"));
                            }
                            filter_by_type__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListDevicesRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_type: filter_by_type__,
                    filter_by_status: filter_by_status__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.ListDevicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDevicesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.devices.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.ListDevicesResponse", len)?;
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDevicesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "devices",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Devices,
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
                            "devices" => Ok(GeneratedField::Devices),
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
            type Value = ListDevicesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.ListDevicesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDevicesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut devices__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListDevicesResponse {
                    devices: devices__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.ListDevicesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrintJobRoutingRulesRequest {
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
        if self.filter_by_printer_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.ListPrintJobRoutingRulesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_printer_id.as_ref() {
            struct_ser.serialize_field("filterByPrinterId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPrintJobRoutingRulesRequest {
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
            "filter_by_printer_id",
            "filterByPrinterId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByPrinterId,
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
                            "filterByPrinterId" | "filter_by_printer_id" => Ok(GeneratedField::FilterByPrinterId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPrintJobRoutingRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.ListPrintJobRoutingRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrintJobRoutingRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_printer_id__ = None;
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
                        GeneratedField::FilterByPrinterId => {
                            if filter_by_printer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByPrinterId"));
                            }
                            filter_by_printer_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPrintJobRoutingRulesRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_printer_id: filter_by_printer_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.ListPrintJobRoutingRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrintJobRoutingRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.ListPrintJobRoutingRulesResponse", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPrintJobRoutingRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
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
            type Value = ListPrintJobRoutingRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.ListPrintJobRoutingRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrintJobRoutingRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPrintJobRoutingRulesResponse {
                    rules: rules__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.ListPrintJobRoutingRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrintJobRoutingRule {
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
        if self.product_category_id.is_some() {
            len += 1;
        }
        if !self.target_printer_id.is_empty() {
            len += 1;
        }
        if self.is_active {
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
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        if self.product_category_summary.is_some() {
            len += 1;
        }
        if self.target_printer_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.PrintJobRoutingRule", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.warehouse_id.is_empty() {
            struct_ser.serialize_field("warehouseId", &self.warehouse_id)?;
        }
        if let Some(v) = self.product_category_id.as_ref() {
            struct_ser.serialize_field("productCategoryId", v)?;
        }
        if !self.target_printer_id.is_empty() {
            struct_ser.serialize_field("targetPrinterId", &self.target_printer_id)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
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
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        if let Some(v) = self.product_category_summary.as_ref() {
            struct_ser.serialize_field("productCategorySummary", v)?;
        }
        if let Some(v) = self.target_printer_summary.as_ref() {
            struct_ser.serialize_field("targetPrinterSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrintJobRoutingRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "product_category_id",
            "productCategoryId",
            "target_printer_id",
            "targetPrinterId",
            "is_active",
            "isActive",
            "priority",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "warehouse_summary",
            "warehouseSummary",
            "product_category_summary",
            "productCategorySummary",
            "target_printer_summary",
            "targetPrinterSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            ProductCategoryId,
            TargetPrinterId,
            IsActive,
            Priority,
            CreatedAt,
            UpdatedAt,
            WarehouseSummary,
            ProductCategorySummary,
            TargetPrinterSummary,
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
                            "productCategoryId" | "product_category_id" => Ok(GeneratedField::ProductCategoryId),
                            "targetPrinterId" | "target_printer_id" => Ok(GeneratedField::TargetPrinterId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "priority" => Ok(GeneratedField::Priority),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            "productCategorySummary" | "product_category_summary" => Ok(GeneratedField::ProductCategorySummary),
                            "targetPrinterSummary" | "target_printer_summary" => Ok(GeneratedField::TargetPrinterSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrintJobRoutingRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.PrintJobRoutingRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrintJobRoutingRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut product_category_id__ = None;
                let mut target_printer_id__ = None;
                let mut is_active__ = None;
                let mut priority__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut warehouse_summary__ = None;
                let mut product_category_summary__ = None;
                let mut target_printer_summary__ = None;
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
                        GeneratedField::ProductCategoryId => {
                            if product_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCategoryId"));
                            }
                            product_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::TargetPrinterId => {
                            if target_printer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPrinterId"));
                            }
                            target_printer_id__ = Some(map_.next_value()?);
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
                        GeneratedField::ProductCategorySummary => {
                            if product_category_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCategorySummary"));
                            }
                            product_category_summary__ = map_.next_value()?;
                        }
                        GeneratedField::TargetPrinterSummary => {
                            if target_printer_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPrinterSummary"));
                            }
                            target_printer_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PrintJobRoutingRule {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__.unwrap_or_default(),
                    product_category_id: product_category_id__,
                    target_printer_id: target_printer_id__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    warehouse_summary: warehouse_summary__,
                    product_category_summary: product_category_summary__,
                    target_printer_summary: target_printer_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.PrintJobRoutingRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDeviceRequest {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.model.is_some() {
            len += 1;
        }
        if self.serial_number.is_some() {
            len += 1;
        }
        if self.ip_address.is_some() {
            len += 1;
        }
        if self.mac_address.is_some() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if self.connection_type.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.driver_details.is_some() {
            len += 1;
        }
        if self.installed_at.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.UpdateDeviceRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.model.as_ref() {
            struct_ser.serialize_field("model", v)?;
        }
        if let Some(v) = self.serial_number.as_ref() {
            struct_ser.serialize_field("serialNumber", v)?;
        }
        if let Some(v) = self.ip_address.as_ref() {
            struct_ser.serialize_field("ipAddress", v)?;
        }
        if let Some(v) = self.mac_address.as_ref() {
            struct_ser.serialize_field("macAddress", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.connection_type.as_ref() {
            struct_ser.serialize_field("connectionType", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.driver_details.as_ref() {
            struct_ser.serialize_field("driverDetails", v)?;
        }
        if let Some(v) = self.installed_at.as_ref() {
            struct_ser.serialize_field("installedAt", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDeviceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "model",
            "serial_number",
            "serialNumber",
            "ip_address",
            "ipAddress",
            "mac_address",
            "macAddress",
            "port",
            "connection_type",
            "connectionType",
            "status",
            "is_active",
            "isActive",
            "configuration",
            "driver_details",
            "driverDetails",
            "installed_at",
            "installedAt",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Model,
            SerialNumber,
            IpAddress,
            MacAddress,
            Port,
            ConnectionType,
            Status,
            IsActive,
            Configuration,
            DriverDetails,
            InstalledAt,
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
                            "name" => Ok(GeneratedField::Name),
                            "model" => Ok(GeneratedField::Model),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            "macAddress" | "mac_address" => Ok(GeneratedField::MacAddress),
                            "port" => Ok(GeneratedField::Port),
                            "connectionType" | "connection_type" => Ok(GeneratedField::ConnectionType),
                            "status" => Ok(GeneratedField::Status),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "driverDetails" | "driver_details" => Ok(GeneratedField::DriverDetails),
                            "installedAt" | "installed_at" => Ok(GeneratedField::InstalledAt),
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
            type Value = UpdateDeviceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.UpdateDeviceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDeviceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut model__ = None;
                let mut serial_number__ = None;
                let mut ip_address__ = None;
                let mut mac_address__ = None;
                let mut port__ = None;
                let mut connection_type__ = None;
                let mut status__ = None;
                let mut is_active__ = None;
                let mut configuration__ = None;
                let mut driver_details__ = None;
                let mut installed_at__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = map_.next_value()?;
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = map_.next_value()?;
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = map_.next_value()?;
                        }
                        GeneratedField::MacAddress => {
                            if mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("macAddress"));
                            }
                            mac_address__ = map_.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ConnectionType => {
                            if connection_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionType"));
                            }
                            connection_type__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
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
                        GeneratedField::DriverDetails => {
                            if driver_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driverDetails"));
                            }
                            driver_details__ = map_.next_value()?;
                        }
                        GeneratedField::InstalledAt => {
                            if installed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("installedAt"));
                            }
                            installed_at__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateDeviceRequest {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    model: model__,
                    serial_number: serial_number__,
                    ip_address: ip_address__,
                    mac_address: mac_address__,
                    port: port__,
                    connection_type: connection_type__,
                    status: status__,
                    is_active: is_active__,
                    configuration: configuration__,
                    driver_details: driver_details__,
                    installed_at: installed_at__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.UpdateDeviceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePrintJobRoutingRuleRequest {
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
        if self.product_category_id.is_some() {
            len += 1;
        }
        if self.target_printer_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.device.v1.UpdatePrintJobRoutingRuleRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.product_category_id.as_ref() {
            struct_ser.serialize_field("productCategoryId", v)?;
        }
        if let Some(v) = self.target_printer_id.as_ref() {
            struct_ser.serialize_field("targetPrinterId", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePrintJobRoutingRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "product_category_id",
            "productCategoryId",
            "target_printer_id",
            "targetPrinterId",
            "is_active",
            "isActive",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProductCategoryId,
            TargetPrinterId,
            IsActive,
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
                            "id" => Ok(GeneratedField::Id),
                            "productCategoryId" | "product_category_id" => Ok(GeneratedField::ProductCategoryId),
                            "targetPrinterId" | "target_printer_id" => Ok(GeneratedField::TargetPrinterId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
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
            type Value = UpdatePrintJobRoutingRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.device.v1.UpdatePrintJobRoutingRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePrintJobRoutingRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut product_category_id__ = None;
                let mut target_printer_id__ = None;
                let mut is_active__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductCategoryId => {
                            if product_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCategoryId"));
                            }
                            product_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::TargetPrinterId => {
                            if target_printer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPrinterId"));
                            }
                            target_printer_id__ = map_.next_value()?;
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
                    }
                }
                Ok(UpdatePrintJobRoutingRuleRequest {
                    id: id__.unwrap_or_default(),
                    product_category_id: product_category_id__,
                    target_printer_id: target_printer_id__,
                    is_active: is_active__,
                    priority: priority__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.device.v1.UpdatePrintJobRoutingRuleRequest", FIELDS, GeneratedVisitor)
    }
}
