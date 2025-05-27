// @generated
impl serde::Serialize for AddCustomerAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.customer_id.is_empty() {
            len += 1;
        }
        if !self.address_type.is_empty() {
            len += 1;
        }
        if self.address_details.is_some() {
            len += 1;
        }
        if self.is_default_shipping.is_some() {
            len += 1;
        }
        if self.is_default_billing.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.AddCustomerAddressRequest", len)?;
        if !self.customer_id.is_empty() {
            struct_ser.serialize_field("customerId", &self.customer_id)?;
        }
        if !self.address_type.is_empty() {
            struct_ser.serialize_field("addressType", &self.address_type)?;
        }
        if let Some(v) = self.address_details.as_ref() {
            struct_ser.serialize_field("addressDetails", v)?;
        }
        if let Some(v) = self.is_default_shipping.as_ref() {
            struct_ser.serialize_field("isDefaultShipping", v)?;
        }
        if let Some(v) = self.is_default_billing.as_ref() {
            struct_ser.serialize_field("isDefaultBilling", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddCustomerAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer_id",
            "customerId",
            "address_type",
            "addressType",
            "address_details",
            "addressDetails",
            "is_default_shipping",
            "isDefaultShipping",
            "is_default_billing",
            "isDefaultBilling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomerId,
            AddressType,
            AddressDetails,
            IsDefaultShipping,
            IsDefaultBilling,
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
                            "addressType" | "address_type" => Ok(GeneratedField::AddressType),
                            "addressDetails" | "address_details" => Ok(GeneratedField::AddressDetails),
                            "isDefaultShipping" | "is_default_shipping" => Ok(GeneratedField::IsDefaultShipping),
                            "isDefaultBilling" | "is_default_billing" => Ok(GeneratedField::IsDefaultBilling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddCustomerAddressRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.AddCustomerAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddCustomerAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer_id__ = None;
                let mut address_type__ = None;
                let mut address_details__ = None;
                let mut is_default_shipping__ = None;
                let mut is_default_billing__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressType => {
                            if address_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressType"));
                            }
                            address_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressDetails => {
                            if address_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressDetails"));
                            }
                            address_details__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefaultShipping => {
                            if is_default_shipping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultShipping"));
                            }
                            is_default_shipping__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefaultBilling => {
                            if is_default_billing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultBilling"));
                            }
                            is_default_billing__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddCustomerAddressRequest {
                    customer_id: customer_id__.unwrap_or_default(),
                    address_type: address_type__.unwrap_or_default(),
                    address_details: address_details__,
                    is_default_shipping: is_default_shipping__,
                    is_default_billing: is_default_billing__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.AddCustomerAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCustomerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.customer_code.is_some() {
            len += 1;
        }
        if self.first_name.is_some() {
            len += 1;
        }
        if self.last_name.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.company_name.is_some() {
            len += 1;
        }
        if self.tax_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.CreateCustomerRequest", len)?;
        if let Some(v) = self.customer_code.as_ref() {
            struct_ser.serialize_field("customerCode", v)?;
        }
        if let Some(v) = self.first_name.as_ref() {
            struct_ser.serialize_field("firstName", v)?;
        }
        if let Some(v) = self.last_name.as_ref() {
            struct_ser.serialize_field("lastName", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.company_name.as_ref() {
            struct_ser.serialize_field("companyName", v)?;
        }
        if let Some(v) = self.tax_id.as_ref() {
            struct_ser.serialize_field("taxId", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateCustomerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer_code",
            "customerCode",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "email",
            "phone",
            "company_name",
            "companyName",
            "tax_id",
            "taxId",
            "is_active",
            "isActive",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomerCode,
            FirstName,
            LastName,
            Email,
            Phone,
            CompanyName,
            TaxId,
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
                            "customerCode" | "customer_code" => Ok(GeneratedField::CustomerCode),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "companyName" | "company_name" => Ok(GeneratedField::CompanyName),
                            "taxId" | "tax_id" => Ok(GeneratedField::TaxId),
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
            type Value = CreateCustomerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.CreateCustomerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCustomerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer_code__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut company_name__ = None;
                let mut tax_id__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CustomerCode => {
                            if customer_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerCode"));
                            }
                            customer_code__ = map_.next_value()?;
                        }
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = map_.next_value()?;
                        }
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = map_.next_value()?;
                        }
                        GeneratedField::CompanyName => {
                            if company_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("companyName"));
                            }
                            company_name__ = map_.next_value()?;
                        }
                        GeneratedField::TaxId => {
                            if tax_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxId"));
                            }
                            tax_id__ = map_.next_value()?;
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
                Ok(CreateCustomerRequest {
                    customer_code: customer_code__,
                    first_name: first_name__,
                    last_name: last_name__,
                    email: email__,
                    phone: phone__,
                    company_name: company_name__,
                    tax_id: tax_id__,
                    is_active: is_active__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.CreateCustomerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Customer {
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
        if self.customer_code.is_some() {
            len += 1;
        }
        if self.first_name.is_some() {
            len += 1;
        }
        if self.last_name.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.company_name.is_some() {
            len += 1;
        }
        if self.tax_id.is_some() {
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
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.Customer", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.customer_code.as_ref() {
            struct_ser.serialize_field("customerCode", v)?;
        }
        if let Some(v) = self.first_name.as_ref() {
            struct_ser.serialize_field("firstName", v)?;
        }
        if let Some(v) = self.last_name.as_ref() {
            struct_ser.serialize_field("lastName", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.company_name.as_ref() {
            struct_ser.serialize_field("companyName", v)?;
        }
        if let Some(v) = self.tax_id.as_ref() {
            struct_ser.serialize_field("taxId", v)?;
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
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Customer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "customer_code",
            "customerCode",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "email",
            "phone",
            "company_name",
            "companyName",
            "tax_id",
            "taxId",
            "is_active",
            "isActive",
            "notes",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "addresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CustomerCode,
            FirstName,
            LastName,
            Email,
            Phone,
            CompanyName,
            TaxId,
            IsActive,
            Notes,
            CreatedAt,
            UpdatedAt,
            Addresses,
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
                            "customerCode" | "customer_code" => Ok(GeneratedField::CustomerCode),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "companyName" | "company_name" => Ok(GeneratedField::CompanyName),
                            "taxId" | "tax_id" => Ok(GeneratedField::TaxId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Customer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.Customer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Customer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut customer_code__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut company_name__ = None;
                let mut tax_id__ = None;
                let mut is_active__ = None;
                let mut notes__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomerCode => {
                            if customer_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerCode"));
                            }
                            customer_code__ = map_.next_value()?;
                        }
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = map_.next_value()?;
                        }
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = map_.next_value()?;
                        }
                        GeneratedField::CompanyName => {
                            if company_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("companyName"));
                            }
                            company_name__ = map_.next_value()?;
                        }
                        GeneratedField::TaxId => {
                            if tax_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxId"));
                            }
                            tax_id__ = map_.next_value()?;
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
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Customer {
                    id: id__.unwrap_or_default(),
                    customer_code: customer_code__,
                    first_name: first_name__,
                    last_name: last_name__,
                    email: email__,
                    phone: phone__,
                    company_name: company_name__,
                    tax_id: tax_id__,
                    is_active: is_active__.unwrap_or_default(),
                    notes: notes__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.Customer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomerAddress {
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
        if !self.address_type.is_empty() {
            len += 1;
        }
        if self.address_details.is_some() {
            len += 1;
        }
        if self.is_default_shipping {
            len += 1;
        }
        if self.is_default_billing {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.CustomerAddress", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.address_type.is_empty() {
            struct_ser.serialize_field("addressType", &self.address_type)?;
        }
        if let Some(v) = self.address_details.as_ref() {
            struct_ser.serialize_field("addressDetails", v)?;
        }
        if self.is_default_shipping {
            struct_ser.serialize_field("isDefaultShipping", &self.is_default_shipping)?;
        }
        if self.is_default_billing {
            struct_ser.serialize_field("isDefaultBilling", &self.is_default_billing)?;
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
impl<'de> serde::Deserialize<'de> for CustomerAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "address_type",
            "addressType",
            "address_details",
            "addressDetails",
            "is_default_shipping",
            "isDefaultShipping",
            "is_default_billing",
            "isDefaultBilling",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AddressType,
            AddressDetails,
            IsDefaultShipping,
            IsDefaultBilling,
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
                            "addressType" | "address_type" => Ok(GeneratedField::AddressType),
                            "addressDetails" | "address_details" => Ok(GeneratedField::AddressDetails),
                            "isDefaultShipping" | "is_default_shipping" => Ok(GeneratedField::IsDefaultShipping),
                            "isDefaultBilling" | "is_default_billing" => Ok(GeneratedField::IsDefaultBilling),
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
            type Value = CustomerAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.CustomerAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CustomerAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut address_type__ = None;
                let mut address_details__ = None;
                let mut is_default_shipping__ = None;
                let mut is_default_billing__ = None;
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
                        GeneratedField::AddressType => {
                            if address_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressType"));
                            }
                            address_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressDetails => {
                            if address_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressDetails"));
                            }
                            address_details__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefaultShipping => {
                            if is_default_shipping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultShipping"));
                            }
                            is_default_shipping__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefaultBilling => {
                            if is_default_billing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultBilling"));
                            }
                            is_default_billing__ = Some(map_.next_value()?);
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
                Ok(CustomerAddress {
                    id: id__.unwrap_or_default(),
                    address_type: address_type__.unwrap_or_default(),
                    address_details: address_details__,
                    is_default_shipping: is_default_shipping__.unwrap_or_default(),
                    is_default_billing: is_default_billing__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.CustomerAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteCustomerAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.customer_id.is_empty() {
            len += 1;
        }
        if !self.customer_address_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.DeleteCustomerAddressRequest", len)?;
        if !self.customer_id.is_empty() {
            struct_ser.serialize_field("customerId", &self.customer_id)?;
        }
        if !self.customer_address_id.is_empty() {
            struct_ser.serialize_field("customerAddressId", &self.customer_address_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteCustomerAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer_id",
            "customerId",
            "customer_address_id",
            "customerAddressId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomerId,
            CustomerAddressId,
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
                            "customerAddressId" | "customer_address_id" => Ok(GeneratedField::CustomerAddressId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteCustomerAddressRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.DeleteCustomerAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteCustomerAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer_id__ = None;
                let mut customer_address_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomerAddressId => {
                            if customer_address_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerAddressId"));
                            }
                            customer_address_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteCustomerAddressRequest {
                    customer_id: customer_id__.unwrap_or_default(),
                    customer_address_id: customer_address_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.DeleteCustomerAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCustomerByEmailRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if self.locale_input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.GetCustomerByEmailRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCustomerByEmailRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "locale_input",
            "localeInput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
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
                            "email" => Ok(GeneratedField::Email),
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
            type Value = GetCustomerByEmailRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.GetCustomerByEmailRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCustomerByEmailRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut locale_input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCustomerByEmailRequest {
                    email: email__.unwrap_or_default(),
                    locale_input: locale_input__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.GetCustomerByEmailRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCustomerByPhoneRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone.is_empty() {
            len += 1;
        }
        if self.locale_input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.GetCustomerByPhoneRequest", len)?;
        if !self.phone.is_empty() {
            struct_ser.serialize_field("phone", &self.phone)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCustomerByPhoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phone",
            "locale_input",
            "localeInput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phone,
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
                            "phone" => Ok(GeneratedField::Phone),
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
            type Value = GetCustomerByPhoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.GetCustomerByPhoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCustomerByPhoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone__ = None;
                let mut locale_input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCustomerByPhoneRequest {
                    phone: phone__.unwrap_or_default(),
                    locale_input: locale_input__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.GetCustomerByPhoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCustomerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.customer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.GetCustomerResponse", len)?;
        if let Some(v) = self.customer.as_ref() {
            struct_ser.serialize_field("customer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCustomerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Customer,
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
                            "customer" => Ok(GeneratedField::Customer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCustomerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.GetCustomerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCustomerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Customer => {
                            if customer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customer"));
                            }
                            customer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCustomerResponse {
                    customer: customer__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.GetCustomerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCustomersRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.ListCustomersRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCustomersRequest {
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
            type Value = ListCustomersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.ListCustomersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCustomersRequest, V::Error>
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
                Ok(ListCustomersRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.ListCustomersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCustomersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.customers.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.ListCustomersResponse", len)?;
        if !self.customers.is_empty() {
            struct_ser.serialize_field("customers", &self.customers)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCustomersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customers",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Customers,
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
                            "customers" => Ok(GeneratedField::Customers),
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
            type Value = ListCustomersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.ListCustomersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCustomersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customers__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Customers => {
                            if customers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customers"));
                            }
                            customers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListCustomersResponse {
                    customers: customers__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.ListCustomersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetDefaultCustomerAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.customer_id.is_empty() {
            len += 1;
        }
        if !self.customer_address_id.is_empty() {
            len += 1;
        }
        if !self.address_type_to_set_default.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.SetDefaultCustomerAddressRequest", len)?;
        if !self.customer_id.is_empty() {
            struct_ser.serialize_field("customerId", &self.customer_id)?;
        }
        if !self.customer_address_id.is_empty() {
            struct_ser.serialize_field("customerAddressId", &self.customer_address_id)?;
        }
        if !self.address_type_to_set_default.is_empty() {
            struct_ser.serialize_field("addressTypeToSetDefault", &self.address_type_to_set_default)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetDefaultCustomerAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer_id",
            "customerId",
            "customer_address_id",
            "customerAddressId",
            "address_type_to_set_default",
            "addressTypeToSetDefault",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomerId,
            CustomerAddressId,
            AddressTypeToSetDefault,
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
                            "customerAddressId" | "customer_address_id" => Ok(GeneratedField::CustomerAddressId),
                            "addressTypeToSetDefault" | "address_type_to_set_default" => Ok(GeneratedField::AddressTypeToSetDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetDefaultCustomerAddressRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.SetDefaultCustomerAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetDefaultCustomerAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer_id__ = None;
                let mut customer_address_id__ = None;
                let mut address_type_to_set_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CustomerId => {
                            if customer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerId"));
                            }
                            customer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomerAddressId => {
                            if customer_address_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerAddressId"));
                            }
                            customer_address_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressTypeToSetDefault => {
                            if address_type_to_set_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressTypeToSetDefault"));
                            }
                            address_type_to_set_default__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetDefaultCustomerAddressRequest {
                    customer_id: customer_id__.unwrap_or_default(),
                    customer_address_id: customer_address_id__.unwrap_or_default(),
                    address_type_to_set_default: address_type_to_set_default__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.SetDefaultCustomerAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCustomerAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.customer_address_id.is_empty() {
            len += 1;
        }
        if self.address_type.is_some() {
            len += 1;
        }
        if self.address_details.is_some() {
            len += 1;
        }
        if self.is_default_shipping.is_some() {
            len += 1;
        }
        if self.is_default_billing.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.UpdateCustomerAddressRequest", len)?;
        if !self.customer_address_id.is_empty() {
            struct_ser.serialize_field("customerAddressId", &self.customer_address_id)?;
        }
        if let Some(v) = self.address_type.as_ref() {
            struct_ser.serialize_field("addressType", v)?;
        }
        if let Some(v) = self.address_details.as_ref() {
            struct_ser.serialize_field("addressDetails", v)?;
        }
        if let Some(v) = self.is_default_shipping.as_ref() {
            struct_ser.serialize_field("isDefaultShipping", v)?;
        }
        if let Some(v) = self.is_default_billing.as_ref() {
            struct_ser.serialize_field("isDefaultBilling", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCustomerAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customer_address_id",
            "customerAddressId",
            "address_type",
            "addressType",
            "address_details",
            "addressDetails",
            "is_default_shipping",
            "isDefaultShipping",
            "is_default_billing",
            "isDefaultBilling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomerAddressId,
            AddressType,
            AddressDetails,
            IsDefaultShipping,
            IsDefaultBilling,
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
                            "customerAddressId" | "customer_address_id" => Ok(GeneratedField::CustomerAddressId),
                            "addressType" | "address_type" => Ok(GeneratedField::AddressType),
                            "addressDetails" | "address_details" => Ok(GeneratedField::AddressDetails),
                            "isDefaultShipping" | "is_default_shipping" => Ok(GeneratedField::IsDefaultShipping),
                            "isDefaultBilling" | "is_default_billing" => Ok(GeneratedField::IsDefaultBilling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCustomerAddressRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.UpdateCustomerAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCustomerAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customer_address_id__ = None;
                let mut address_type__ = None;
                let mut address_details__ = None;
                let mut is_default_shipping__ = None;
                let mut is_default_billing__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CustomerAddressId => {
                            if customer_address_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerAddressId"));
                            }
                            customer_address_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressType => {
                            if address_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressType"));
                            }
                            address_type__ = map_.next_value()?;
                        }
                        GeneratedField::AddressDetails => {
                            if address_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressDetails"));
                            }
                            address_details__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefaultShipping => {
                            if is_default_shipping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultShipping"));
                            }
                            is_default_shipping__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefaultBilling => {
                            if is_default_billing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefaultBilling"));
                            }
                            is_default_billing__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCustomerAddressRequest {
                    customer_address_id: customer_address_id__.unwrap_or_default(),
                    address_type: address_type__,
                    address_details: address_details__,
                    is_default_shipping: is_default_shipping__,
                    is_default_billing: is_default_billing__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.UpdateCustomerAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCustomerRequest {
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
        if self.customer_code.is_some() {
            len += 1;
        }
        if self.first_name.is_some() {
            len += 1;
        }
        if self.last_name.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.company_name.is_some() {
            len += 1;
        }
        if self.tax_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.customer.v1.UpdateCustomerRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.customer_code.as_ref() {
            struct_ser.serialize_field("customerCode", v)?;
        }
        if let Some(v) = self.first_name.as_ref() {
            struct_ser.serialize_field("firstName", v)?;
        }
        if let Some(v) = self.last_name.as_ref() {
            struct_ser.serialize_field("lastName", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.company_name.as_ref() {
            struct_ser.serialize_field("companyName", v)?;
        }
        if let Some(v) = self.tax_id.as_ref() {
            struct_ser.serialize_field("taxId", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateCustomerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "customer_code",
            "customerCode",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "email",
            "phone",
            "company_name",
            "companyName",
            "tax_id",
            "taxId",
            "is_active",
            "isActive",
            "notes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CustomerCode,
            FirstName,
            LastName,
            Email,
            Phone,
            CompanyName,
            TaxId,
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
                            "customerCode" | "customer_code" => Ok(GeneratedField::CustomerCode),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "companyName" | "company_name" => Ok(GeneratedField::CompanyName),
                            "taxId" | "tax_id" => Ok(GeneratedField::TaxId),
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
            type Value = UpdateCustomerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.customer.v1.UpdateCustomerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCustomerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut customer_code__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut company_name__ = None;
                let mut tax_id__ = None;
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
                        GeneratedField::CustomerCode => {
                            if customer_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customerCode"));
                            }
                            customer_code__ = map_.next_value()?;
                        }
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = map_.next_value()?;
                        }
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = map_.next_value()?;
                        }
                        GeneratedField::CompanyName => {
                            if company_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("companyName"));
                            }
                            company_name__ = map_.next_value()?;
                        }
                        GeneratedField::TaxId => {
                            if tax_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxId"));
                            }
                            tax_id__ = map_.next_value()?;
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
                Ok(UpdateCustomerRequest {
                    id: id__.unwrap_or_default(),
                    customer_code: customer_code__,
                    first_name: first_name__,
                    last_name: last_name__,
                    email: email__,
                    phone: phone__,
                    company_name: company_name__,
                    tax_id: tax_id__,
                    is_active: is_active__,
                    notes: notes__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.customer.v1.UpdateCustomerRequest", FIELDS, GeneratedVisitor)
    }
}
