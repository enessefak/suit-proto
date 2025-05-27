// @generated
impl serde::Serialize for CreateUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.first_name.is_some() {
            len += 1;
        }
        if self.last_name.is_some() {
            len += 1;
        }
        if self.phone_number.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.is_system_admin.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.CreateUserRequest", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.first_name.as_ref() {
            struct_ser.serialize_field("firstName", v)?;
        }
        if let Some(v) = self.last_name.as_ref() {
            struct_ser.serialize_field("lastName", v)?;
        }
        if let Some(v) = self.phone_number.as_ref() {
            struct_ser.serialize_field("phoneNumber", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.is_system_admin.as_ref() {
            struct_ser.serialize_field("isSystemAdmin", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "password",
            "email",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "phone_number",
            "phoneNumber",
            "is_active",
            "isActive",
            "is_system_admin",
            "isSystemAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
            Password,
            Email,
            FirstName,
            LastName,
            PhoneNumber,
            IsActive,
            IsSystemAdmin,
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
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            "email" => Ok(GeneratedField::Email),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "phoneNumber" | "phone_number" => Ok(GeneratedField::PhoneNumber),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "isSystemAdmin" | "is_system_admin" => Ok(GeneratedField::IsSystemAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.CreateUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut password__ = None;
                let mut email__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut phone_number__ = None;
                let mut is_active__ = None;
                let mut is_system_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
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
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::IsSystemAdmin => {
                            if is_system_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSystemAdmin"));
                            }
                            is_system_admin__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateUserRequest {
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                    email: email__,
                    first_name: first_name__,
                    last_name: last_name__,
                    phone_number: phone_number__,
                    is_active: is_active__,
                    is_system_admin: is_system_admin__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.CreateUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserByUsernameRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if self.locale_input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.GetUserByUsernameRequest", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if let Some(v) = self.locale_input.as_ref() {
            struct_ser.serialize_field("localeInput", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserByUsernameRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "locale_input",
            "localeInput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
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
                            "username" => Ok(GeneratedField::Username),
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
            type Value = GetUserByUsernameRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.GetUserByUsernameRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserByUsernameRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut locale_input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocaleInput => {
                            if locale_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localeInput"));
                            }
                            locale_input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserByUsernameRequest {
                    username: username__.unwrap_or_default(),
                    locale_input: locale_input__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.GetUserByUsernameRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.GetUserResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
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
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.GetUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserResponse {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.GetUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.ListUsersRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersRequest {
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
            type Value = ListUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.ListUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersRequest, V::Error>
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
                Ok(ListUsersRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.ListUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.users.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.ListUsersResponse", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "users",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Users,
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
                            "users" => Ok(GeneratedField::Users),
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
            type Value = ListUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.ListUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut users__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListUsersResponse {
                    users: users__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.ListUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetUserPasswordRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.new_password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.SetUserPasswordRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.new_password.is_empty() {
            struct_ser.serialize_field("newPassword", &self.new_password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetUserPasswordRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "new_password",
            "newPassword",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            NewPassword,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "newPassword" | "new_password" => Ok(GeneratedField::NewPassword),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetUserPasswordRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.SetUserPasswordRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetUserPasswordRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut new_password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPassword => {
                            if new_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPassword"));
                            }
                            new_password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetUserPasswordRequest {
                    user_id: user_id__.unwrap_or_default(),
                    new_password: new_password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.SetUserPasswordRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserRequest {
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
        if self.username.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.first_name.is_some() {
            len += 1;
        }
        if self.last_name.is_some() {
            len += 1;
        }
        if self.phone_number.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.is_system_admin.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.UpdateUserRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.username.as_ref() {
            struct_ser.serialize_field("username", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.first_name.as_ref() {
            struct_ser.serialize_field("firstName", v)?;
        }
        if let Some(v) = self.last_name.as_ref() {
            struct_ser.serialize_field("lastName", v)?;
        }
        if let Some(v) = self.phone_number.as_ref() {
            struct_ser.serialize_field("phoneNumber", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.is_system_admin.as_ref() {
            struct_ser.serialize_field("isSystemAdmin", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "username",
            "email",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "phone_number",
            "phoneNumber",
            "is_active",
            "isActive",
            "is_system_admin",
            "isSystemAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Username,
            Email,
            FirstName,
            LastName,
            PhoneNumber,
            IsActive,
            IsSystemAdmin,
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
                            "username" => Ok(GeneratedField::Username),
                            "email" => Ok(GeneratedField::Email),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "phoneNumber" | "phone_number" => Ok(GeneratedField::PhoneNumber),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "isSystemAdmin" | "is_system_admin" => Ok(GeneratedField::IsSystemAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.UpdateUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut username__ = None;
                let mut email__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut phone_number__ = None;
                let mut is_active__ = None;
                let mut is_system_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
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
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::IsSystemAdmin => {
                            if is_system_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSystemAdmin"));
                            }
                            is_system_admin__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserRequest {
                    id: id__.unwrap_or_default(),
                    username: username__,
                    email: email__,
                    first_name: first_name__,
                    last_name: last_name__,
                    phone_number: phone_number__,
                    is_active: is_active__,
                    is_system_admin: is_system_admin__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.UpdateUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
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
        if !self.username.is_empty() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.first_name.is_some() {
            len += 1;
        }
        if self.last_name.is_some() {
            len += 1;
        }
        if self.phone_number.is_some() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.is_system_admin {
            len += 1;
        }
        if self.last_login_at.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.user.v1.User", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.first_name.as_ref() {
            struct_ser.serialize_field("firstName", v)?;
        }
        if let Some(v) = self.last_name.as_ref() {
            struct_ser.serialize_field("lastName", v)?;
        }
        if let Some(v) = self.phone_number.as_ref() {
            struct_ser.serialize_field("phoneNumber", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if self.is_system_admin {
            struct_ser.serialize_field("isSystemAdmin", &self.is_system_admin)?;
        }
        if let Some(v) = self.last_login_at.as_ref() {
            struct_ser.serialize_field("lastLoginAt", v)?;
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
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "username",
            "email",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "phone_number",
            "phoneNumber",
            "is_active",
            "isActive",
            "is_system_admin",
            "isSystemAdmin",
            "last_login_at",
            "lastLoginAt",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Username,
            Email,
            FirstName,
            LastName,
            PhoneNumber,
            IsActive,
            IsSystemAdmin,
            LastLoginAt,
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
                            "username" => Ok(GeneratedField::Username),
                            "email" => Ok(GeneratedField::Email),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "phoneNumber" | "phone_number" => Ok(GeneratedField::PhoneNumber),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "isSystemAdmin" | "is_system_admin" => Ok(GeneratedField::IsSystemAdmin),
                            "lastLoginAt" | "last_login_at" => Ok(GeneratedField::LastLoginAt),
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
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.user.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut username__ = None;
                let mut email__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut phone_number__ = None;
                let mut is_active__ = None;
                let mut is_system_admin__ = None;
                let mut last_login_at__ = None;
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
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
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
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSystemAdmin => {
                            if is_system_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSystemAdmin"));
                            }
                            is_system_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastLoginAt => {
                            if last_login_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastLoginAt"));
                            }
                            last_login_at__ = map_.next_value()?;
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
                Ok(User {
                    id: id__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    email: email__,
                    first_name: first_name__,
                    last_name: last_name__,
                    phone_number: phone_number__,
                    is_active: is_active__.unwrap_or_default(),
                    is_system_admin: is_system_admin__.unwrap_or_default(),
                    last_login_at: last_login_at__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.user.v1.User", FIELDS, GeneratedVisitor)
    }
}
