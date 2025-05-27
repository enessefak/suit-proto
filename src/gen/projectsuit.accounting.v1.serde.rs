// @generated
impl serde::Serialize for ApproveExpenseRequest {
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
        if self.approved_by_user_id.is_some() {
            len += 1;
        }
        if self.post_to_gl_on_approve.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ApproveExpenseRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.approved_by_user_id.as_ref() {
            struct_ser.serialize_field("approvedByUserId", v)?;
        }
        if let Some(v) = self.post_to_gl_on_approve.as_ref() {
            struct_ser.serialize_field("postToGlOnApprove", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApproveExpenseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "approved_by_user_id",
            "approvedByUserId",
            "post_to_gl_on_approve",
            "postToGlOnApprove",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ApprovedByUserId,
            PostToGlOnApprove,
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
                            "approvedByUserId" | "approved_by_user_id" => Ok(GeneratedField::ApprovedByUserId),
                            "postToGlOnApprove" | "post_to_gl_on_approve" => Ok(GeneratedField::PostToGlOnApprove),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApproveExpenseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ApproveExpenseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApproveExpenseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut approved_by_user_id__ = None;
                let mut post_to_gl_on_approve__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApprovedByUserId => {
                            if approved_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("approvedByUserId"));
                            }
                            approved_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::PostToGlOnApprove => {
                            if post_to_gl_on_approve__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postToGlOnApprove"));
                            }
                            post_to_gl_on_approve__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ApproveExpenseRequest {
                    id: id__.unwrap_or_default(),
                    approved_by_user_id: approved_by_user_id__,
                    post_to_gl_on_approve: post_to_gl_on_approve__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ApproveExpenseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChartOfAccount {
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
        if !self.account_code.is_empty() {
            len += 1;
        }
        if !self.account_type.is_empty() {
            len += 1;
        }
        if self.parent_account_id.is_some() {
            len += 1;
        }
        if !self.normal_balance.is_empty() {
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
        if !self.account_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ChartOfAccount", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.account_code.is_empty() {
            struct_ser.serialize_field("accountCode", &self.account_code)?;
        }
        if !self.account_type.is_empty() {
            struct_ser.serialize_field("accountType", &self.account_type)?;
        }
        if let Some(v) = self.parent_account_id.as_ref() {
            struct_ser.serialize_field("parentAccountId", v)?;
        }
        if !self.normal_balance.is_empty() {
            struct_ser.serialize_field("normalBalance", &self.normal_balance)?;
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
        if !self.account_name.is_empty() {
            struct_ser.serialize_field("accountName", &self.account_name)?;
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
impl<'de> serde::Deserialize<'de> for ChartOfAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "account_code",
            "accountCode",
            "account_type",
            "accountType",
            "parent_account_id",
            "parentAccountId",
            "normal_balance",
            "normalBalance",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "account_name",
            "accountName",
            "description",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AccountCode,
            AccountType,
            ParentAccountId,
            NormalBalance,
            IsActive,
            CreatedAt,
            UpdatedAt,
            AccountName,
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
                            "accountCode" | "account_code" => Ok(GeneratedField::AccountCode),
                            "accountType" | "account_type" => Ok(GeneratedField::AccountType),
                            "parentAccountId" | "parent_account_id" => Ok(GeneratedField::ParentAccountId),
                            "normalBalance" | "normal_balance" => Ok(GeneratedField::NormalBalance),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "accountName" | "account_name" => Ok(GeneratedField::AccountName),
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
            type Value = ChartOfAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ChartOfAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChartOfAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut account_code__ = None;
                let mut account_type__ = None;
                let mut parent_account_id__ = None;
                let mut normal_balance__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut account_name__ = None;
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
                        GeneratedField::AccountCode => {
                            if account_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountCode"));
                            }
                            account_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccountType => {
                            if account_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountType"));
                            }
                            account_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentAccountId => {
                            if parent_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentAccountId"));
                            }
                            parent_account_id__ = map_.next_value()?;
                        }
                        GeneratedField::NormalBalance => {
                            if normal_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalBalance"));
                            }
                            normal_balance__ = Some(map_.next_value()?);
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
                        GeneratedField::AccountName => {
                            if account_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountName"));
                            }
                            account_name__ = Some(map_.next_value()?);
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
                Ok(ChartOfAccount {
                    id: id__.unwrap_or_default(),
                    account_code: account_code__.unwrap_or_default(),
                    account_type: account_type__.unwrap_or_default(),
                    parent_account_id: parent_account_id__,
                    normal_balance: normal_balance__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    account_name: account_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ChartOfAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateChartOfAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_code.is_empty() {
            len += 1;
        }
        if !self.account_type.is_empty() {
            len += 1;
        }
        if self.parent_account_id.is_some() {
            len += 1;
        }
        if !self.normal_balance.is_empty() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.CreateChartOfAccountRequest", len)?;
        if !self.account_code.is_empty() {
            struct_ser.serialize_field("accountCode", &self.account_code)?;
        }
        if !self.account_type.is_empty() {
            struct_ser.serialize_field("accountType", &self.account_type)?;
        }
        if let Some(v) = self.parent_account_id.as_ref() {
            struct_ser.serialize_field("parentAccountId", v)?;
        }
        if !self.normal_balance.is_empty() {
            struct_ser.serialize_field("normalBalance", &self.normal_balance)?;
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
impl<'de> serde::Deserialize<'de> for CreateChartOfAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_code",
            "accountCode",
            "account_type",
            "accountType",
            "parent_account_id",
            "parentAccountId",
            "normal_balance",
            "normalBalance",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountCode,
            AccountType,
            ParentAccountId,
            NormalBalance,
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
                            "accountCode" | "account_code" => Ok(GeneratedField::AccountCode),
                            "accountType" | "account_type" => Ok(GeneratedField::AccountType),
                            "parentAccountId" | "parent_account_id" => Ok(GeneratedField::ParentAccountId),
                            "normalBalance" | "normal_balance" => Ok(GeneratedField::NormalBalance),
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
            type Value = CreateChartOfAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.CreateChartOfAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateChartOfAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_code__ = None;
                let mut account_type__ = None;
                let mut parent_account_id__ = None;
                let mut normal_balance__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountCode => {
                            if account_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountCode"));
                            }
                            account_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccountType => {
                            if account_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountType"));
                            }
                            account_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentAccountId => {
                            if parent_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentAccountId"));
                            }
                            parent_account_id__ = map_.next_value()?;
                        }
                        GeneratedField::NormalBalance => {
                            if normal_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalBalance"));
                            }
                            normal_balance__ = Some(map_.next_value()?);
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
                Ok(CreateChartOfAccountRequest {
                    account_code: account_code__.unwrap_or_default(),
                    account_type: account_type__.unwrap_or_default(),
                    parent_account_id: parent_account_id__,
                    normal_balance: normal_balance__.unwrap_or_default(),
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.CreateChartOfAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateExpenseCategoryRequest {
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
        if self.parent_category_id.is_some() {
            len += 1;
        }
        if self.default_expense_account_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.CreateExpenseCategoryRequest", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.parent_category_id.as_ref() {
            struct_ser.serialize_field("parentCategoryId", v)?;
        }
        if let Some(v) = self.default_expense_account_id.as_ref() {
            struct_ser.serialize_field("defaultExpenseAccountId", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateExpenseCategoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "parent_category_id",
            "parentCategoryId",
            "default_expense_account_id",
            "defaultExpenseAccountId",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            ParentCategoryId,
            DefaultExpenseAccountId,
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
                            "parentCategoryId" | "parent_category_id" => Ok(GeneratedField::ParentCategoryId),
                            "defaultExpenseAccountId" | "default_expense_account_id" => Ok(GeneratedField::DefaultExpenseAccountId),
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
            type Value = CreateExpenseCategoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.CreateExpenseCategoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateExpenseCategoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut parent_category_id__ = None;
                let mut default_expense_account_id__ = None;
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
                        GeneratedField::ParentCategoryId => {
                            if parent_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentCategoryId"));
                            }
                            parent_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultExpenseAccountId => {
                            if default_expense_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultExpenseAccountId"));
                            }
                            default_expense_account_id__ = map_.next_value()?;
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
                Ok(CreateExpenseCategoryRequest {
                    code: code__,
                    parent_category_id: parent_category_id__,
                    default_expense_account_id: default_expense_account_id__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.CreateExpenseCategoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateExpenseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.warehouse_id.is_some() {
            len += 1;
        }
        if !self.expense_date.is_empty() {
            len += 1;
        }
        if self.vendor_id.is_some() {
            len += 1;
        }
        if !self.expense_category_id.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.tax_amount.is_some() {
            len += 1;
        }
        if self.payment_method_id.is_some() {
            len += 1;
        }
        if self.payment_date.is_some() {
            len += 1;
        }
        if self.reference_number.is_some() {
            len += 1;
        }
        if self.initial_status.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if !self.attachment_urls.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.CreateExpenseRequest", len)?;
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if !self.expense_date.is_empty() {
            struct_ser.serialize_field("expenseDate", &self.expense_date)?;
        }
        if let Some(v) = self.vendor_id.as_ref() {
            struct_ser.serialize_field("vendorId", v)?;
        }
        if !self.expense_category_id.is_empty() {
            struct_ser.serialize_field("expenseCategoryId", &self.expense_category_id)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.tax_amount.as_ref() {
            struct_ser.serialize_field("taxAmount", v)?;
        }
        if let Some(v) = self.payment_method_id.as_ref() {
            struct_ser.serialize_field("paymentMethodId", v)?;
        }
        if let Some(v) = self.payment_date.as_ref() {
            struct_ser.serialize_field("paymentDate", v)?;
        }
        if let Some(v) = self.reference_number.as_ref() {
            struct_ser.serialize_field("referenceNumber", v)?;
        }
        if let Some(v) = self.initial_status.as_ref() {
            struct_ser.serialize_field("initialStatus", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if !self.attachment_urls.is_empty() {
            struct_ser.serialize_field("attachmentUrls", &self.attachment_urls)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateExpenseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "warehouse_id",
            "warehouseId",
            "expense_date",
            "expenseDate",
            "vendor_id",
            "vendorId",
            "expense_category_id",
            "expenseCategoryId",
            "description",
            "amount",
            "tax_amount",
            "taxAmount",
            "payment_method_id",
            "paymentMethodId",
            "payment_date",
            "paymentDate",
            "reference_number",
            "referenceNumber",
            "initial_status",
            "initialStatus",
            "notes",
            "created_by_user_id",
            "createdByUserId",
            "attachment_urls",
            "attachmentUrls",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WarehouseId,
            ExpenseDate,
            VendorId,
            ExpenseCategoryId,
            Description,
            Amount,
            TaxAmount,
            PaymentMethodId,
            PaymentDate,
            ReferenceNumber,
            InitialStatus,
            Notes,
            CreatedByUserId,
            AttachmentUrls,
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
                            "expenseDate" | "expense_date" => Ok(GeneratedField::ExpenseDate),
                            "vendorId" | "vendor_id" => Ok(GeneratedField::VendorId),
                            "expenseCategoryId" | "expense_category_id" => Ok(GeneratedField::ExpenseCategoryId),
                            "description" => Ok(GeneratedField::Description),
                            "amount" => Ok(GeneratedField::Amount),
                            "taxAmount" | "tax_amount" => Ok(GeneratedField::TaxAmount),
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "paymentDate" | "payment_date" => Ok(GeneratedField::PaymentDate),
                            "referenceNumber" | "reference_number" => Ok(GeneratedField::ReferenceNumber),
                            "initialStatus" | "initial_status" => Ok(GeneratedField::InitialStatus),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "attachmentUrls" | "attachment_urls" => Ok(GeneratedField::AttachmentUrls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateExpenseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.CreateExpenseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateExpenseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut warehouse_id__ = None;
                let mut expense_date__ = None;
                let mut vendor_id__ = None;
                let mut expense_category_id__ = None;
                let mut description__ = None;
                let mut amount__ = None;
                let mut tax_amount__ = None;
                let mut payment_method_id__ = None;
                let mut payment_date__ = None;
                let mut reference_number__ = None;
                let mut initial_status__ = None;
                let mut notes__ = None;
                let mut created_by_user_id__ = None;
                let mut attachment_urls__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WarehouseId => {
                            if warehouse_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseId"));
                            }
                            warehouse_id__ = map_.next_value()?;
                        }
                        GeneratedField::ExpenseDate => {
                            if expense_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseDate"));
                            }
                            expense_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VendorId => {
                            if vendor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendorId"));
                            }
                            vendor_id__ = map_.next_value()?;
                        }
                        GeneratedField::ExpenseCategoryId => {
                            if expense_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseCategoryId"));
                            }
                            expense_category_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::TaxAmount => {
                            if tax_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxAmount"));
                            }
                            tax_amount__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethodId => {
                            if payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodId"));
                            }
                            payment_method_id__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentDate => {
                            if payment_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentDate"));
                            }
                            payment_date__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceNumber => {
                            if reference_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceNumber"));
                            }
                            reference_number__ = map_.next_value()?;
                        }
                        GeneratedField::InitialStatus => {
                            if initial_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStatus"));
                            }
                            initial_status__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::AttachmentUrls => {
                            if attachment_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentUrls"));
                            }
                            attachment_urls__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateExpenseRequest {
                    warehouse_id: warehouse_id__,
                    expense_date: expense_date__.unwrap_or_default(),
                    vendor_id: vendor_id__,
                    expense_category_id: expense_category_id__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    amount: amount__,
                    tax_amount: tax_amount__,
                    payment_method_id: payment_method_id__,
                    payment_date: payment_date__,
                    reference_number: reference_number__,
                    initial_status: initial_status__,
                    notes: notes__,
                    created_by_user_id: created_by_user_id__,
                    attachment_urls: attachment_urls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.CreateExpenseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateGeneralLedgerLineInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_id.is_empty() {
            len += 1;
        }
        if !self.debit_amount.is_empty() {
            len += 1;
        }
        if !self.credit_amount.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.CreateGeneralLedgerLineInput", len)?;
        if !self.account_id.is_empty() {
            struct_ser.serialize_field("accountId", &self.account_id)?;
        }
        if !self.debit_amount.is_empty() {
            struct_ser.serialize_field("debitAmount", &self.debit_amount)?;
        }
        if !self.credit_amount.is_empty() {
            struct_ser.serialize_field("creditAmount", &self.credit_amount)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateGeneralLedgerLineInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_id",
            "accountId",
            "debit_amount",
            "debitAmount",
            "credit_amount",
            "creditAmount",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountId,
            DebitAmount,
            CreditAmount,
            Description,
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
                            "accountId" | "account_id" => Ok(GeneratedField::AccountId),
                            "debitAmount" | "debit_amount" => Ok(GeneratedField::DebitAmount),
                            "creditAmount" | "credit_amount" => Ok(GeneratedField::CreditAmount),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateGeneralLedgerLineInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.CreateGeneralLedgerLineInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateGeneralLedgerLineInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_id__ = None;
                let mut debit_amount__ = None;
                let mut credit_amount__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountId => {
                            if account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountId"));
                            }
                            account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DebitAmount => {
                            if debit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debitAmount"));
                            }
                            debit_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreditAmount => {
                            if credit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditAmount"));
                            }
                            credit_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateGeneralLedgerLineInput {
                    account_id: account_id__.unwrap_or_default(),
                    debit_amount: debit_amount__.unwrap_or_default(),
                    credit_amount: credit_amount__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.CreateGeneralLedgerLineInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateGeneralLedgerTransactionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transaction_date.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.transaction_type.is_some() {
            len += 1;
        }
        if self.reference_document_id.is_some() {
            len += 1;
        }
        if self.reference_document_code.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if !self.lines.is_empty() {
            len += 1;
        }
        if self.post_immediately.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.CreateGeneralLedgerTransactionRequest", len)?;
        if !self.transaction_date.is_empty() {
            struct_ser.serialize_field("transactionDate", &self.transaction_date)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.transaction_type.as_ref() {
            struct_ser.serialize_field("transactionType", v)?;
        }
        if let Some(v) = self.reference_document_id.as_ref() {
            struct_ser.serialize_field("referenceDocumentId", v)?;
        }
        if let Some(v) = self.reference_document_code.as_ref() {
            struct_ser.serialize_field("referenceDocumentCode", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if !self.lines.is_empty() {
            struct_ser.serialize_field("lines", &self.lines)?;
        }
        if let Some(v) = self.post_immediately.as_ref() {
            struct_ser.serialize_field("postImmediately", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateGeneralLedgerTransactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_date",
            "transactionDate",
            "description",
            "transaction_type",
            "transactionType",
            "reference_document_id",
            "referenceDocumentId",
            "reference_document_code",
            "referenceDocumentCode",
            "created_by_user_id",
            "createdByUserId",
            "lines",
            "post_immediately",
            "postImmediately",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionDate,
            Description,
            TransactionType,
            ReferenceDocumentId,
            ReferenceDocumentCode,
            CreatedByUserId,
            Lines,
            PostImmediately,
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
                            "transactionDate" | "transaction_date" => Ok(GeneratedField::TransactionDate),
                            "description" => Ok(GeneratedField::Description),
                            "transactionType" | "transaction_type" => Ok(GeneratedField::TransactionType),
                            "referenceDocumentId" | "reference_document_id" => Ok(GeneratedField::ReferenceDocumentId),
                            "referenceDocumentCode" | "reference_document_code" => Ok(GeneratedField::ReferenceDocumentCode),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "lines" => Ok(GeneratedField::Lines),
                            "postImmediately" | "post_immediately" => Ok(GeneratedField::PostImmediately),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateGeneralLedgerTransactionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.CreateGeneralLedgerTransactionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateGeneralLedgerTransactionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_date__ = None;
                let mut description__ = None;
                let mut transaction_type__ = None;
                let mut reference_document_id__ = None;
                let mut reference_document_code__ = None;
                let mut created_by_user_id__ = None;
                let mut lines__ = None;
                let mut post_immediately__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransactionDate => {
                            if transaction_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionDate"));
                            }
                            transaction_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TransactionType => {
                            if transaction_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionType"));
                            }
                            transaction_type__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceDocumentId => {
                            if reference_document_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentId"));
                            }
                            reference_document_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceDocumentCode => {
                            if reference_document_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentCode"));
                            }
                            reference_document_code__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::Lines => {
                            if lines__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lines"));
                            }
                            lines__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PostImmediately => {
                            if post_immediately__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postImmediately"));
                            }
                            post_immediately__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateGeneralLedgerTransactionRequest {
                    transaction_date: transaction_date__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    transaction_type: transaction_type__,
                    reference_document_id: reference_document_id__,
                    reference_document_code: reference_document_code__,
                    created_by_user_id: created_by_user_id__,
                    lines: lines__.unwrap_or_default(),
                    post_immediately: post_immediately__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.CreateGeneralLedgerTransactionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateVendorRequest {
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
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.tax_id.is_some() {
            len += 1;
        }
        if self.bank_account_details.is_some() {
            len += 1;
        }
        if self.default_expense_account_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.CreateVendorRequest", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.tax_id.as_ref() {
            struct_ser.serialize_field("taxId", v)?;
        }
        if let Some(v) = self.bank_account_details.as_ref() {
            struct_ser.serialize_field("bankAccountDetails", v)?;
        }
        if let Some(v) = self.default_expense_account_id.as_ref() {
            struct_ser.serialize_field("defaultExpenseAccountId", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateVendorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "email",
            "phone",
            "address",
            "tax_id",
            "taxId",
            "bank_account_details",
            "bankAccountDetails",
            "default_expense_account_id",
            "defaultExpenseAccountId",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Email,
            Phone,
            Address,
            TaxId,
            BankAccountDetails,
            DefaultExpenseAccountId,
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
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "address" => Ok(GeneratedField::Address),
                            "taxId" | "tax_id" => Ok(GeneratedField::TaxId),
                            "bankAccountDetails" | "bank_account_details" => Ok(GeneratedField::BankAccountDetails),
                            "defaultExpenseAccountId" | "default_expense_account_id" => Ok(GeneratedField::DefaultExpenseAccountId),
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
            type Value = CreateVendorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.CreateVendorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateVendorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut address__ = None;
                let mut tax_id__ = None;
                let mut bank_account_details__ = None;
                let mut default_expense_account_id__ = None;
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
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::TaxId => {
                            if tax_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxId"));
                            }
                            tax_id__ = map_.next_value()?;
                        }
                        GeneratedField::BankAccountDetails => {
                            if bank_account_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankAccountDetails"));
                            }
                            bank_account_details__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultExpenseAccountId => {
                            if default_expense_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultExpenseAccountId"));
                            }
                            default_expense_account_id__ = map_.next_value()?;
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
                Ok(CreateVendorRequest {
                    code: code__,
                    email: email__,
                    phone: phone__,
                    address: address__,
                    tax_id: tax_id__,
                    bank_account_details: bank_account_details__,
                    default_expense_account_id: default_expense_account_id__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.CreateVendorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Expense {
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
        if !self.expense_date.is_empty() {
            len += 1;
        }
        if self.vendor_id.is_some() {
            len += 1;
        }
        if !self.expense_category_id.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.tax_amount.is_some() {
            len += 1;
        }
        if self.total_amount.is_some() {
            len += 1;
        }
        if self.payment_method_id.is_some() {
            len += 1;
        }
        if self.payment_date.is_some() {
            len += 1;
        }
        if self.reference_number.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if self.approved_by_user_id.is_some() {
            len += 1;
        }
        if self.related_gl_transaction_id.is_some() {
            len += 1;
        }
        if !self.attachment_urls.is_empty() {
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
        if self.warehouse_summary.is_some() {
            len += 1;
        }
        if self.vendor_summary.is_some() {
            len += 1;
        }
        if self.expense_category_summary.is_some() {
            len += 1;
        }
        if self.payment_method_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.Expense", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if !self.expense_date.is_empty() {
            struct_ser.serialize_field("expenseDate", &self.expense_date)?;
        }
        if let Some(v) = self.vendor_id.as_ref() {
            struct_ser.serialize_field("vendorId", v)?;
        }
        if !self.expense_category_id.is_empty() {
            struct_ser.serialize_field("expenseCategoryId", &self.expense_category_id)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.tax_amount.as_ref() {
            struct_ser.serialize_field("taxAmount", v)?;
        }
        if let Some(v) = self.total_amount.as_ref() {
            struct_ser.serialize_field("totalAmount", v)?;
        }
        if let Some(v) = self.payment_method_id.as_ref() {
            struct_ser.serialize_field("paymentMethodId", v)?;
        }
        if let Some(v) = self.payment_date.as_ref() {
            struct_ser.serialize_field("paymentDate", v)?;
        }
        if let Some(v) = self.reference_number.as_ref() {
            struct_ser.serialize_field("referenceNumber", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if let Some(v) = self.approved_by_user_id.as_ref() {
            struct_ser.serialize_field("approvedByUserId", v)?;
        }
        if let Some(v) = self.related_gl_transaction_id.as_ref() {
            struct_ser.serialize_field("relatedGlTransactionId", v)?;
        }
        if !self.attachment_urls.is_empty() {
            struct_ser.serialize_field("attachmentUrls", &self.attachment_urls)?;
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
        if let Some(v) = self.warehouse_summary.as_ref() {
            struct_ser.serialize_field("warehouseSummary", v)?;
        }
        if let Some(v) = self.vendor_summary.as_ref() {
            struct_ser.serialize_field("vendorSummary", v)?;
        }
        if let Some(v) = self.expense_category_summary.as_ref() {
            struct_ser.serialize_field("expenseCategorySummary", v)?;
        }
        if let Some(v) = self.payment_method_summary.as_ref() {
            struct_ser.serialize_field("paymentMethodSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Expense {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "expense_date",
            "expenseDate",
            "vendor_id",
            "vendorId",
            "expense_category_id",
            "expenseCategoryId",
            "description",
            "amount",
            "tax_amount",
            "taxAmount",
            "total_amount",
            "totalAmount",
            "payment_method_id",
            "paymentMethodId",
            "payment_date",
            "paymentDate",
            "reference_number",
            "referenceNumber",
            "status",
            "notes",
            "created_by_user_id",
            "createdByUserId",
            "approved_by_user_id",
            "approvedByUserId",
            "related_gl_transaction_id",
            "relatedGlTransactionId",
            "attachment_urls",
            "attachmentUrls",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "warehouse_summary",
            "warehouseSummary",
            "vendor_summary",
            "vendorSummary",
            "expense_category_summary",
            "expenseCategorySummary",
            "payment_method_summary",
            "paymentMethodSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            ExpenseDate,
            VendorId,
            ExpenseCategoryId,
            Description,
            Amount,
            TaxAmount,
            TotalAmount,
            PaymentMethodId,
            PaymentDate,
            ReferenceNumber,
            Status,
            Notes,
            CreatedByUserId,
            ApprovedByUserId,
            RelatedGlTransactionId,
            AttachmentUrls,
            IsActive,
            CreatedAt,
            UpdatedAt,
            WarehouseSummary,
            VendorSummary,
            ExpenseCategorySummary,
            PaymentMethodSummary,
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
                            "expenseDate" | "expense_date" => Ok(GeneratedField::ExpenseDate),
                            "vendorId" | "vendor_id" => Ok(GeneratedField::VendorId),
                            "expenseCategoryId" | "expense_category_id" => Ok(GeneratedField::ExpenseCategoryId),
                            "description" => Ok(GeneratedField::Description),
                            "amount" => Ok(GeneratedField::Amount),
                            "taxAmount" | "tax_amount" => Ok(GeneratedField::TaxAmount),
                            "totalAmount" | "total_amount" => Ok(GeneratedField::TotalAmount),
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "paymentDate" | "payment_date" => Ok(GeneratedField::PaymentDate),
                            "referenceNumber" | "reference_number" => Ok(GeneratedField::ReferenceNumber),
                            "status" => Ok(GeneratedField::Status),
                            "notes" => Ok(GeneratedField::Notes),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "approvedByUserId" | "approved_by_user_id" => Ok(GeneratedField::ApprovedByUserId),
                            "relatedGlTransactionId" | "related_gl_transaction_id" => Ok(GeneratedField::RelatedGlTransactionId),
                            "attachmentUrls" | "attachment_urls" => Ok(GeneratedField::AttachmentUrls),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "warehouseSummary" | "warehouse_summary" => Ok(GeneratedField::WarehouseSummary),
                            "vendorSummary" | "vendor_summary" => Ok(GeneratedField::VendorSummary),
                            "expenseCategorySummary" | "expense_category_summary" => Ok(GeneratedField::ExpenseCategorySummary),
                            "paymentMethodSummary" | "payment_method_summary" => Ok(GeneratedField::PaymentMethodSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Expense;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.Expense")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Expense, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut expense_date__ = None;
                let mut vendor_id__ = None;
                let mut expense_category_id__ = None;
                let mut description__ = None;
                let mut amount__ = None;
                let mut tax_amount__ = None;
                let mut total_amount__ = None;
                let mut payment_method_id__ = None;
                let mut payment_date__ = None;
                let mut reference_number__ = None;
                let mut status__ = None;
                let mut notes__ = None;
                let mut created_by_user_id__ = None;
                let mut approved_by_user_id__ = None;
                let mut related_gl_transaction_id__ = None;
                let mut attachment_urls__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut warehouse_summary__ = None;
                let mut vendor_summary__ = None;
                let mut expense_category_summary__ = None;
                let mut payment_method_summary__ = None;
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
                        GeneratedField::ExpenseDate => {
                            if expense_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseDate"));
                            }
                            expense_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VendorId => {
                            if vendor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendorId"));
                            }
                            vendor_id__ = map_.next_value()?;
                        }
                        GeneratedField::ExpenseCategoryId => {
                            if expense_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseCategoryId"));
                            }
                            expense_category_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::TaxAmount => {
                            if tax_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxAmount"));
                            }
                            tax_amount__ = map_.next_value()?;
                        }
                        GeneratedField::TotalAmount => {
                            if total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmount"));
                            }
                            total_amount__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethodId => {
                            if payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodId"));
                            }
                            payment_method_id__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentDate => {
                            if payment_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentDate"));
                            }
                            payment_date__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceNumber => {
                            if reference_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceNumber"));
                            }
                            reference_number__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::ApprovedByUserId => {
                            if approved_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("approvedByUserId"));
                            }
                            approved_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::RelatedGlTransactionId => {
                            if related_gl_transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relatedGlTransactionId"));
                            }
                            related_gl_transaction_id__ = map_.next_value()?;
                        }
                        GeneratedField::AttachmentUrls => {
                            if attachment_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentUrls"));
                            }
                            attachment_urls__ = Some(map_.next_value()?);
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
                        GeneratedField::WarehouseSummary => {
                            if warehouse_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouseSummary"));
                            }
                            warehouse_summary__ = map_.next_value()?;
                        }
                        GeneratedField::VendorSummary => {
                            if vendor_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendorSummary"));
                            }
                            vendor_summary__ = map_.next_value()?;
                        }
                        GeneratedField::ExpenseCategorySummary => {
                            if expense_category_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseCategorySummary"));
                            }
                            expense_category_summary__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethodSummary => {
                            if payment_method_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodSummary"));
                            }
                            payment_method_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Expense {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__,
                    expense_date: expense_date__.unwrap_or_default(),
                    vendor_id: vendor_id__,
                    expense_category_id: expense_category_id__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    amount: amount__,
                    tax_amount: tax_amount__,
                    total_amount: total_amount__,
                    payment_method_id: payment_method_id__,
                    payment_date: payment_date__,
                    reference_number: reference_number__,
                    status: status__.unwrap_or_default(),
                    notes: notes__,
                    created_by_user_id: created_by_user_id__,
                    approved_by_user_id: approved_by_user_id__,
                    related_gl_transaction_id: related_gl_transaction_id__,
                    attachment_urls: attachment_urls__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    warehouse_summary: warehouse_summary__,
                    vendor_summary: vendor_summary__,
                    expense_category_summary: expense_category_summary__,
                    payment_method_summary: payment_method_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.Expense", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpenseCategory {
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
        if self.parent_category_id.is_some() {
            len += 1;
        }
        if self.default_expense_account_id.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ExpenseCategory", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.parent_category_id.as_ref() {
            struct_ser.serialize_field("parentCategoryId", v)?;
        }
        if let Some(v) = self.default_expense_account_id.as_ref() {
            struct_ser.serialize_field("defaultExpenseAccountId", v)?;
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
impl<'de> serde::Deserialize<'de> for ExpenseCategory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "parent_category_id",
            "parentCategoryId",
            "default_expense_account_id",
            "defaultExpenseAccountId",
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
            ParentCategoryId,
            DefaultExpenseAccountId,
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
                            "parentCategoryId" | "parent_category_id" => Ok(GeneratedField::ParentCategoryId),
                            "defaultExpenseAccountId" | "default_expense_account_id" => Ok(GeneratedField::DefaultExpenseAccountId),
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
            type Value = ExpenseCategory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ExpenseCategory")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpenseCategory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut parent_category_id__ = None;
                let mut default_expense_account_id__ = None;
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
                        GeneratedField::ParentCategoryId => {
                            if parent_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentCategoryId"));
                            }
                            parent_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultExpenseAccountId => {
                            if default_expense_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultExpenseAccountId"));
                            }
                            default_expense_account_id__ = map_.next_value()?;
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
                Ok(ExpenseCategory {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    parent_category_id: parent_category_id__,
                    default_expense_account_id: default_expense_account_id__,
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ExpenseCategory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeneralLedgerLine {
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
        if !self.account_id.is_empty() {
            len += 1;
        }
        if self.debit_amount.is_some() {
            len += 1;
        }
        if self.credit_amount.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.account_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GeneralLedgerLine", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.account_id.is_empty() {
            struct_ser.serialize_field("accountId", &self.account_id)?;
        }
        if let Some(v) = self.debit_amount.as_ref() {
            struct_ser.serialize_field("debitAmount", v)?;
        }
        if let Some(v) = self.credit_amount.as_ref() {
            struct_ser.serialize_field("creditAmount", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.account_summary.as_ref() {
            struct_ser.serialize_field("accountSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeneralLedgerLine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "account_id",
            "accountId",
            "debit_amount",
            "debitAmount",
            "credit_amount",
            "creditAmount",
            "description",
            "created_at",
            "createdAt",
            "account_summary",
            "accountSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AccountId,
            DebitAmount,
            CreditAmount,
            Description,
            CreatedAt,
            AccountSummary,
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
                            "accountId" | "account_id" => Ok(GeneratedField::AccountId),
                            "debitAmount" | "debit_amount" => Ok(GeneratedField::DebitAmount),
                            "creditAmount" | "credit_amount" => Ok(GeneratedField::CreditAmount),
                            "description" => Ok(GeneratedField::Description),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "accountSummary" | "account_summary" => Ok(GeneratedField::AccountSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeneralLedgerLine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GeneralLedgerLine")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeneralLedgerLine, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut account_id__ = None;
                let mut debit_amount__ = None;
                let mut credit_amount__ = None;
                let mut description__ = None;
                let mut created_at__ = None;
                let mut account_summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccountId => {
                            if account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountId"));
                            }
                            account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DebitAmount => {
                            if debit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debitAmount"));
                            }
                            debit_amount__ = map_.next_value()?;
                        }
                        GeneratedField::CreditAmount => {
                            if credit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditAmount"));
                            }
                            credit_amount__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::AccountSummary => {
                            if account_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountSummary"));
                            }
                            account_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GeneralLedgerLine {
                    id: id__.unwrap_or_default(),
                    account_id: account_id__.unwrap_or_default(),
                    debit_amount: debit_amount__,
                    credit_amount: credit_amount__,
                    description: description__,
                    created_at: created_at__,
                    account_summary: account_summary__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GeneralLedgerLine", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeneralLedgerTransaction {
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
        if !self.transaction_date.is_empty() {
            len += 1;
        }
        if !self.posting_date.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.transaction_type.is_some() {
            len += 1;
        }
        if self.reference_document_id.is_some() {
            len += 1;
        }
        if self.reference_document_code.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.is_auto_generated {
            len += 1;
        }
        if self.void_reason.is_some() {
            len += 1;
        }
        if self.created_by_user_id.is_some() {
            len += 1;
        }
        if self.posted_by_user_id.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.posted_at.is_some() {
            len += 1;
        }
        if !self.lines.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GeneralLedgerTransaction", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.transaction_date.is_empty() {
            struct_ser.serialize_field("transactionDate", &self.transaction_date)?;
        }
        if !self.posting_date.is_empty() {
            struct_ser.serialize_field("postingDate", &self.posting_date)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.transaction_type.as_ref() {
            struct_ser.serialize_field("transactionType", v)?;
        }
        if let Some(v) = self.reference_document_id.as_ref() {
            struct_ser.serialize_field("referenceDocumentId", v)?;
        }
        if let Some(v) = self.reference_document_code.as_ref() {
            struct_ser.serialize_field("referenceDocumentCode", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.is_auto_generated {
            struct_ser.serialize_field("isAutoGenerated", &self.is_auto_generated)?;
        }
        if let Some(v) = self.void_reason.as_ref() {
            struct_ser.serialize_field("voidReason", v)?;
        }
        if let Some(v) = self.created_by_user_id.as_ref() {
            struct_ser.serialize_field("createdByUserId", v)?;
        }
        if let Some(v) = self.posted_by_user_id.as_ref() {
            struct_ser.serialize_field("postedByUserId", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.posted_at.as_ref() {
            struct_ser.serialize_field("postedAt", v)?;
        }
        if !self.lines.is_empty() {
            struct_ser.serialize_field("lines", &self.lines)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeneralLedgerTransaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "transaction_date",
            "transactionDate",
            "posting_date",
            "postingDate",
            "description",
            "transaction_type",
            "transactionType",
            "reference_document_id",
            "referenceDocumentId",
            "reference_document_code",
            "referenceDocumentCode",
            "status",
            "is_auto_generated",
            "isAutoGenerated",
            "void_reason",
            "voidReason",
            "created_by_user_id",
            "createdByUserId",
            "posted_by_user_id",
            "postedByUserId",
            "created_at",
            "createdAt",
            "posted_at",
            "postedAt",
            "lines",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TransactionDate,
            PostingDate,
            Description,
            TransactionType,
            ReferenceDocumentId,
            ReferenceDocumentCode,
            Status,
            IsAutoGenerated,
            VoidReason,
            CreatedByUserId,
            PostedByUserId,
            CreatedAt,
            PostedAt,
            Lines,
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
                            "transactionDate" | "transaction_date" => Ok(GeneratedField::TransactionDate),
                            "postingDate" | "posting_date" => Ok(GeneratedField::PostingDate),
                            "description" => Ok(GeneratedField::Description),
                            "transactionType" | "transaction_type" => Ok(GeneratedField::TransactionType),
                            "referenceDocumentId" | "reference_document_id" => Ok(GeneratedField::ReferenceDocumentId),
                            "referenceDocumentCode" | "reference_document_code" => Ok(GeneratedField::ReferenceDocumentCode),
                            "status" => Ok(GeneratedField::Status),
                            "isAutoGenerated" | "is_auto_generated" => Ok(GeneratedField::IsAutoGenerated),
                            "voidReason" | "void_reason" => Ok(GeneratedField::VoidReason),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "postedByUserId" | "posted_by_user_id" => Ok(GeneratedField::PostedByUserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "postedAt" | "posted_at" => Ok(GeneratedField::PostedAt),
                            "lines" => Ok(GeneratedField::Lines),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeneralLedgerTransaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GeneralLedgerTransaction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeneralLedgerTransaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut transaction_date__ = None;
                let mut posting_date__ = None;
                let mut description__ = None;
                let mut transaction_type__ = None;
                let mut reference_document_id__ = None;
                let mut reference_document_code__ = None;
                let mut status__ = None;
                let mut is_auto_generated__ = None;
                let mut void_reason__ = None;
                let mut created_by_user_id__ = None;
                let mut posted_by_user_id__ = None;
                let mut created_at__ = None;
                let mut posted_at__ = None;
                let mut lines__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TransactionDate => {
                            if transaction_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionDate"));
                            }
                            transaction_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PostingDate => {
                            if posting_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postingDate"));
                            }
                            posting_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TransactionType => {
                            if transaction_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionType"));
                            }
                            transaction_type__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceDocumentId => {
                            if reference_document_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentId"));
                            }
                            reference_document_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceDocumentCode => {
                            if reference_document_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocumentCode"));
                            }
                            reference_document_code__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAutoGenerated => {
                            if is_auto_generated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAutoGenerated"));
                            }
                            is_auto_generated__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VoidReason => {
                            if void_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voidReason"));
                            }
                            void_reason__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::PostedByUserId => {
                            if posted_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postedByUserId"));
                            }
                            posted_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::PostedAt => {
                            if posted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postedAt"));
                            }
                            posted_at__ = map_.next_value()?;
                        }
                        GeneratedField::Lines => {
                            if lines__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lines"));
                            }
                            lines__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GeneralLedgerTransaction {
                    id: id__.unwrap_or_default(),
                    transaction_date: transaction_date__.unwrap_or_default(),
                    posting_date: posting_date__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    transaction_type: transaction_type__,
                    reference_document_id: reference_document_id__,
                    reference_document_code: reference_document_code__,
                    status: status__.unwrap_or_default(),
                    is_auto_generated: is_auto_generated__.unwrap_or_default(),
                    void_reason: void_reason__,
                    created_by_user_id: created_by_user_id__,
                    posted_by_user_id: posted_by_user_id__,
                    created_at: created_at__,
                    posted_at: posted_at__,
                    lines: lines__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GeneralLedgerTransaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChartOfAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GetChartOfAccountResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChartOfAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChartOfAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GetChartOfAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChartOfAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetChartOfAccountResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GetChartOfAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExpenseCategoryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.category.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GetExpenseCategoryResponse", len)?;
        if let Some(v) = self.category.as_ref() {
            struct_ser.serialize_field("category", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExpenseCategoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
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
                            "category" => Ok(GeneratedField::Category),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExpenseCategoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GetExpenseCategoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExpenseCategoryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetExpenseCategoryResponse {
                    category: category__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GetExpenseCategoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExpenseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expense.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GetExpenseResponse", len)?;
        if let Some(v) = self.expense.as_ref() {
            struct_ser.serialize_field("expense", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExpenseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expense",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expense,
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
                            "expense" => Ok(GeneratedField::Expense),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExpenseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GetExpenseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExpenseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expense__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expense => {
                            if expense__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expense"));
                            }
                            expense__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetExpenseResponse {
                    expense: expense__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GetExpenseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGeneralLedgerTransactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transaction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GetGeneralLedgerTransactionResponse", len)?;
        if let Some(v) = self.transaction.as_ref() {
            struct_ser.serialize_field("transaction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetGeneralLedgerTransactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transaction,
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
                            "transaction" => Ok(GeneratedField::Transaction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetGeneralLedgerTransactionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GetGeneralLedgerTransactionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetGeneralLedgerTransactionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transaction => {
                            if transaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transaction"));
                            }
                            transaction__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetGeneralLedgerTransactionResponse {
                    transaction: transaction__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GetGeneralLedgerTransactionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVendorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vendor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.GetVendorResponse", len)?;
        if let Some(v) = self.vendor.as_ref() {
            struct_ser.serialize_field("vendor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVendorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vendor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vendor,
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
                            "vendor" => Ok(GeneratedField::Vendor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVendorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.GetVendorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVendorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vendor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vendor => {
                            if vendor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendor"));
                            }
                            vendor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetVendorResponse {
                    vendor: vendor__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.GetVendorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListChartOfAccountsRequest {
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
        if self.filter_by_account_type.is_some() {
            len += 1;
        }
        if self.filter_by_parent_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListChartOfAccountsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_account_type.as_ref() {
            struct_ser.serialize_field("filterByAccountType", v)?;
        }
        if let Some(v) = self.filter_by_parent_id.as_ref() {
            struct_ser.serialize_field("filterByParentId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListChartOfAccountsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_account_type",
            "filterByAccountType",
            "filter_by_parent_id",
            "filterByParentId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByAccountType,
            FilterByParentId,
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
                            "filterByAccountType" | "filter_by_account_type" => Ok(GeneratedField::FilterByAccountType),
                            "filterByParentId" | "filter_by_parent_id" => Ok(GeneratedField::FilterByParentId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListChartOfAccountsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListChartOfAccountsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListChartOfAccountsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_account_type__ = None;
                let mut filter_by_parent_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByAccountType => {
                            if filter_by_account_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByAccountType"));
                            }
                            filter_by_account_type__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByParentId => {
                            if filter_by_parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByParentId"));
                            }
                            filter_by_parent_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListChartOfAccountsRequest {
                    base_request: base_request__,
                    filter_by_account_type: filter_by_account_type__,
                    filter_by_parent_id: filter_by_parent_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListChartOfAccountsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListChartOfAccountsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.accounts.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListChartOfAccountsResponse", len)?;
        if !self.accounts.is_empty() {
            struct_ser.serialize_field("accounts", &self.accounts)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListChartOfAccountsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "accounts",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Accounts,
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
                            "accounts" => Ok(GeneratedField::Accounts),
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
            type Value = ListChartOfAccountsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListChartOfAccountsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListChartOfAccountsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut accounts__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Accounts => {
                            if accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accounts"));
                            }
                            accounts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListChartOfAccountsResponse {
                    accounts: accounts__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListChartOfAccountsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExpenseCategoriesRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListExpenseCategoriesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExpenseCategoriesRequest {
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
            type Value = ListExpenseCategoriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListExpenseCategoriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExpenseCategoriesRequest, V::Error>
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
                Ok(ListExpenseCategoriesRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListExpenseCategoriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExpenseCategoriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.categories.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListExpenseCategoriesResponse", len)?;
        if !self.categories.is_empty() {
            struct_ser.serialize_field("categories", &self.categories)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExpenseCategoriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "categories",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Categories,
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
                            "categories" => Ok(GeneratedField::Categories),
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
            type Value = ListExpenseCategoriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListExpenseCategoriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExpenseCategoriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut categories__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Categories => {
                            if categories__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categories"));
                            }
                            categories__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListExpenseCategoriesResponse {
                    categories: categories__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListExpenseCategoriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExpensesRequest {
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
        if self.filter_by_vendor_id.is_some() {
            len += 1;
        }
        if self.filter_by_expense_category_id.is_some() {
            len += 1;
        }
        if self.filter_by_expense_date_range.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListExpensesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_warehouse_id.as_ref() {
            struct_ser.serialize_field("filterByWarehouseId", v)?;
        }
        if let Some(v) = self.filter_by_vendor_id.as_ref() {
            struct_ser.serialize_field("filterByVendorId", v)?;
        }
        if let Some(v) = self.filter_by_expense_category_id.as_ref() {
            struct_ser.serialize_field("filterByExpenseCategoryId", v)?;
        }
        if let Some(v) = self.filter_by_expense_date_range.as_ref() {
            struct_ser.serialize_field("filterByExpenseDateRange", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExpensesRequest {
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
            "filter_by_vendor_id",
            "filterByVendorId",
            "filter_by_expense_category_id",
            "filterByExpenseCategoryId",
            "filter_by_expense_date_range",
            "filterByExpenseDateRange",
            "filter_by_status",
            "filterByStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByWarehouseId,
            FilterByVendorId,
            FilterByExpenseCategoryId,
            FilterByExpenseDateRange,
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
                            "filterByVendorId" | "filter_by_vendor_id" => Ok(GeneratedField::FilterByVendorId),
                            "filterByExpenseCategoryId" | "filter_by_expense_category_id" => Ok(GeneratedField::FilterByExpenseCategoryId),
                            "filterByExpenseDateRange" | "filter_by_expense_date_range" => Ok(GeneratedField::FilterByExpenseDateRange),
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
            type Value = ListExpensesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListExpensesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExpensesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_warehouse_id__ = None;
                let mut filter_by_vendor_id__ = None;
                let mut filter_by_expense_category_id__ = None;
                let mut filter_by_expense_date_range__ = None;
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
                        GeneratedField::FilterByVendorId => {
                            if filter_by_vendor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByVendorId"));
                            }
                            filter_by_vendor_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByExpenseCategoryId => {
                            if filter_by_expense_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByExpenseCategoryId"));
                            }
                            filter_by_expense_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByExpenseDateRange => {
                            if filter_by_expense_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByExpenseDateRange"));
                            }
                            filter_by_expense_date_range__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListExpensesRequest {
                    base_request: base_request__,
                    filter_by_warehouse_id: filter_by_warehouse_id__,
                    filter_by_vendor_id: filter_by_vendor_id__,
                    filter_by_expense_category_id: filter_by_expense_category_id__,
                    filter_by_expense_date_range: filter_by_expense_date_range__,
                    filter_by_status: filter_by_status__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListExpensesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExpensesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expenses.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListExpensesResponse", len)?;
        if !self.expenses.is_empty() {
            struct_ser.serialize_field("expenses", &self.expenses)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExpensesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expenses",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expenses,
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
                            "expenses" => Ok(GeneratedField::Expenses),
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
            type Value = ListExpensesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListExpensesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExpensesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expenses__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expenses => {
                            if expenses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenses"));
                            }
                            expenses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListExpensesResponse {
                    expenses: expenses__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListExpensesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListGeneralLedgerTransactionsRequest {
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
        if self.filter_by_transaction_date_range.is_some() {
            len += 1;
        }
        if self.filter_by_posting_date_range.is_some() {
            len += 1;
        }
        if self.filter_by_status.is_some() {
            len += 1;
        }
        if self.filter_by_account_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListGeneralLedgerTransactionsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_transaction_date_range.as_ref() {
            struct_ser.serialize_field("filterByTransactionDateRange", v)?;
        }
        if let Some(v) = self.filter_by_posting_date_range.as_ref() {
            struct_ser.serialize_field("filterByPostingDateRange", v)?;
        }
        if let Some(v) = self.filter_by_status.as_ref() {
            struct_ser.serialize_field("filterByStatus", v)?;
        }
        if let Some(v) = self.filter_by_account_id.as_ref() {
            struct_ser.serialize_field("filterByAccountId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListGeneralLedgerTransactionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_transaction_date_range",
            "filterByTransactionDateRange",
            "filter_by_posting_date_range",
            "filterByPostingDateRange",
            "filter_by_status",
            "filterByStatus",
            "filter_by_account_id",
            "filterByAccountId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByTransactionDateRange,
            FilterByPostingDateRange,
            FilterByStatus,
            FilterByAccountId,
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
                            "filterByTransactionDateRange" | "filter_by_transaction_date_range" => Ok(GeneratedField::FilterByTransactionDateRange),
                            "filterByPostingDateRange" | "filter_by_posting_date_range" => Ok(GeneratedField::FilterByPostingDateRange),
                            "filterByStatus" | "filter_by_status" => Ok(GeneratedField::FilterByStatus),
                            "filterByAccountId" | "filter_by_account_id" => Ok(GeneratedField::FilterByAccountId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListGeneralLedgerTransactionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListGeneralLedgerTransactionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListGeneralLedgerTransactionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_transaction_date_range__ = None;
                let mut filter_by_posting_date_range__ = None;
                let mut filter_by_status__ = None;
                let mut filter_by_account_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByTransactionDateRange => {
                            if filter_by_transaction_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByTransactionDateRange"));
                            }
                            filter_by_transaction_date_range__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByPostingDateRange => {
                            if filter_by_posting_date_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByPostingDateRange"));
                            }
                            filter_by_posting_date_range__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByStatus => {
                            if filter_by_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByStatus"));
                            }
                            filter_by_status__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByAccountId => {
                            if filter_by_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByAccountId"));
                            }
                            filter_by_account_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListGeneralLedgerTransactionsRequest {
                    base_request: base_request__,
                    filter_by_transaction_date_range: filter_by_transaction_date_range__,
                    filter_by_posting_date_range: filter_by_posting_date_range__,
                    filter_by_status: filter_by_status__,
                    filter_by_account_id: filter_by_account_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListGeneralLedgerTransactionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListGeneralLedgerTransactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transactions.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListGeneralLedgerTransactionsResponse", len)?;
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListGeneralLedgerTransactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transactions",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transactions,
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
                            "transactions" => Ok(GeneratedField::Transactions),
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
            type Value = ListGeneralLedgerTransactionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListGeneralLedgerTransactionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListGeneralLedgerTransactionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transactions__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListGeneralLedgerTransactionsResponse {
                    transactions: transactions__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListGeneralLedgerTransactionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListVendorsRequest {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListVendorsRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListVendorsRequest {
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
            type Value = ListVendorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListVendorsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListVendorsRequest, V::Error>
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
                Ok(ListVendorsRequest {
                    base_request: base_request__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListVendorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListVendorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vendors.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.ListVendorsResponse", len)?;
        if !self.vendors.is_empty() {
            struct_ser.serialize_field("vendors", &self.vendors)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListVendorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vendors",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vendors,
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
                            "vendors" => Ok(GeneratedField::Vendors),
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
            type Value = ListVendorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.ListVendorsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListVendorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vendors__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vendors => {
                            if vendors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendors"));
                            }
                            vendors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListVendorsResponse {
                    vendors: vendors__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.ListVendorsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PostGeneralLedgerTransactionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transaction_id.is_empty() {
            len += 1;
        }
        if self.posted_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.PostGeneralLedgerTransactionRequest", len)?;
        if !self.transaction_id.is_empty() {
            struct_ser.serialize_field("transactionId", &self.transaction_id)?;
        }
        if let Some(v) = self.posted_by_user_id.as_ref() {
            struct_ser.serialize_field("postedByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostGeneralLedgerTransactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_id",
            "transactionId",
            "posted_by_user_id",
            "postedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionId,
            PostedByUserId,
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
                            "transactionId" | "transaction_id" => Ok(GeneratedField::TransactionId),
                            "postedByUserId" | "posted_by_user_id" => Ok(GeneratedField::PostedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostGeneralLedgerTransactionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.PostGeneralLedgerTransactionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PostGeneralLedgerTransactionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_id__ = None;
                let mut posted_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransactionId => {
                            if transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionId"));
                            }
                            transaction_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PostedByUserId => {
                            if posted_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postedByUserId"));
                            }
                            posted_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PostGeneralLedgerTransactionRequest {
                    transaction_id: transaction_id__.unwrap_or_default(),
                    posted_by_user_id: posted_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.PostGeneralLedgerTransactionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecordExpensePaymentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expense_id.is_empty() {
            len += 1;
        }
        if self.payment_amount.is_some() {
            len += 1;
        }
        if !self.payment_method_id.is_empty() {
            len += 1;
        }
        if !self.payment_date.is_empty() {
            len += 1;
        }
        if self.reference_number.is_some() {
            len += 1;
        }
        if self.paid_by_user_id.is_some() {
            len += 1;
        }
        if self.post_to_gl_on_payment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.RecordExpensePaymentRequest", len)?;
        if !self.expense_id.is_empty() {
            struct_ser.serialize_field("expenseId", &self.expense_id)?;
        }
        if let Some(v) = self.payment_amount.as_ref() {
            struct_ser.serialize_field("paymentAmount", v)?;
        }
        if !self.payment_method_id.is_empty() {
            struct_ser.serialize_field("paymentMethodId", &self.payment_method_id)?;
        }
        if !self.payment_date.is_empty() {
            struct_ser.serialize_field("paymentDate", &self.payment_date)?;
        }
        if let Some(v) = self.reference_number.as_ref() {
            struct_ser.serialize_field("referenceNumber", v)?;
        }
        if let Some(v) = self.paid_by_user_id.as_ref() {
            struct_ser.serialize_field("paidByUserId", v)?;
        }
        if let Some(v) = self.post_to_gl_on_payment.as_ref() {
            struct_ser.serialize_field("postToGlOnPayment", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecordExpensePaymentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expense_id",
            "expenseId",
            "payment_amount",
            "paymentAmount",
            "payment_method_id",
            "paymentMethodId",
            "payment_date",
            "paymentDate",
            "reference_number",
            "referenceNumber",
            "paid_by_user_id",
            "paidByUserId",
            "post_to_gl_on_payment",
            "postToGlOnPayment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpenseId,
            PaymentAmount,
            PaymentMethodId,
            PaymentDate,
            ReferenceNumber,
            PaidByUserId,
            PostToGlOnPayment,
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
                            "expenseId" | "expense_id" => Ok(GeneratedField::ExpenseId),
                            "paymentAmount" | "payment_amount" => Ok(GeneratedField::PaymentAmount),
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "paymentDate" | "payment_date" => Ok(GeneratedField::PaymentDate),
                            "referenceNumber" | "reference_number" => Ok(GeneratedField::ReferenceNumber),
                            "paidByUserId" | "paid_by_user_id" => Ok(GeneratedField::PaidByUserId),
                            "postToGlOnPayment" | "post_to_gl_on_payment" => Ok(GeneratedField::PostToGlOnPayment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecordExpensePaymentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.RecordExpensePaymentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecordExpensePaymentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expense_id__ = None;
                let mut payment_amount__ = None;
                let mut payment_method_id__ = None;
                let mut payment_date__ = None;
                let mut reference_number__ = None;
                let mut paid_by_user_id__ = None;
                let mut post_to_gl_on_payment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExpenseId => {
                            if expense_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseId"));
                            }
                            expense_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentAmount => {
                            if payment_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentAmount"));
                            }
                            payment_amount__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethodId => {
                            if payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodId"));
                            }
                            payment_method_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentDate => {
                            if payment_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentDate"));
                            }
                            payment_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceNumber => {
                            if reference_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceNumber"));
                            }
                            reference_number__ = map_.next_value()?;
                        }
                        GeneratedField::PaidByUserId => {
                            if paid_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paidByUserId"));
                            }
                            paid_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::PostToGlOnPayment => {
                            if post_to_gl_on_payment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postToGlOnPayment"));
                            }
                            post_to_gl_on_payment__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RecordExpensePaymentRequest {
                    expense_id: expense_id__.unwrap_or_default(),
                    payment_amount: payment_amount__,
                    payment_method_id: payment_method_id__.unwrap_or_default(),
                    payment_date: payment_date__.unwrap_or_default(),
                    reference_number: reference_number__,
                    paid_by_user_id: paid_by_user_id__,
                    post_to_gl_on_payment: post_to_gl_on_payment__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.RecordExpensePaymentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateChartOfAccountRequest {
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
        if self.account_code.is_some() {
            len += 1;
        }
        if self.account_type.is_some() {
            len += 1;
        }
        if self.parent_account_id.is_some() {
            len += 1;
        }
        if self.normal_balance.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.UpdateChartOfAccountRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.account_code.as_ref() {
            struct_ser.serialize_field("accountCode", v)?;
        }
        if let Some(v) = self.account_type.as_ref() {
            struct_ser.serialize_field("accountType", v)?;
        }
        if let Some(v) = self.parent_account_id.as_ref() {
            struct_ser.serialize_field("parentAccountId", v)?;
        }
        if let Some(v) = self.normal_balance.as_ref() {
            struct_ser.serialize_field("normalBalance", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateChartOfAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "account_code",
            "accountCode",
            "account_type",
            "accountType",
            "parent_account_id",
            "parentAccountId",
            "normal_balance",
            "normalBalance",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AccountCode,
            AccountType,
            ParentAccountId,
            NormalBalance,
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
                            "accountCode" | "account_code" => Ok(GeneratedField::AccountCode),
                            "accountType" | "account_type" => Ok(GeneratedField::AccountType),
                            "parentAccountId" | "parent_account_id" => Ok(GeneratedField::ParentAccountId),
                            "normalBalance" | "normal_balance" => Ok(GeneratedField::NormalBalance),
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
            type Value = UpdateChartOfAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.UpdateChartOfAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateChartOfAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut account_code__ = None;
                let mut account_type__ = None;
                let mut parent_account_id__ = None;
                let mut normal_balance__ = None;
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
                        GeneratedField::AccountCode => {
                            if account_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountCode"));
                            }
                            account_code__ = map_.next_value()?;
                        }
                        GeneratedField::AccountType => {
                            if account_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountType"));
                            }
                            account_type__ = map_.next_value()?;
                        }
                        GeneratedField::ParentAccountId => {
                            if parent_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentAccountId"));
                            }
                            parent_account_id__ = map_.next_value()?;
                        }
                        GeneratedField::NormalBalance => {
                            if normal_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalBalance"));
                            }
                            normal_balance__ = map_.next_value()?;
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
                Ok(UpdateChartOfAccountRequest {
                    id: id__.unwrap_or_default(),
                    account_code: account_code__,
                    account_type: account_type__,
                    parent_account_id: parent_account_id__,
                    normal_balance: normal_balance__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.UpdateChartOfAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateExpenseCategoryRequest {
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
        if self.parent_category_id.is_some() {
            len += 1;
        }
        if self.default_expense_account_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.UpdateExpenseCategoryRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.parent_category_id.as_ref() {
            struct_ser.serialize_field("parentCategoryId", v)?;
        }
        if let Some(v) = self.default_expense_account_id.as_ref() {
            struct_ser.serialize_field("defaultExpenseAccountId", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateExpenseCategoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "parent_category_id",
            "parentCategoryId",
            "default_expense_account_id",
            "defaultExpenseAccountId",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            ParentCategoryId,
            DefaultExpenseAccountId,
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
                            "parentCategoryId" | "parent_category_id" => Ok(GeneratedField::ParentCategoryId),
                            "defaultExpenseAccountId" | "default_expense_account_id" => Ok(GeneratedField::DefaultExpenseAccountId),
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
            type Value = UpdateExpenseCategoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.UpdateExpenseCategoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateExpenseCategoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut parent_category_id__ = None;
                let mut default_expense_account_id__ = None;
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
                        GeneratedField::ParentCategoryId => {
                            if parent_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentCategoryId"));
                            }
                            parent_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultExpenseAccountId => {
                            if default_expense_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultExpenseAccountId"));
                            }
                            default_expense_account_id__ = map_.next_value()?;
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
                Ok(UpdateExpenseCategoryRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    parent_category_id: parent_category_id__,
                    default_expense_account_id: default_expense_account_id__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.UpdateExpenseCategoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateExpenseRequest {
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
        if self.expense_date.is_some() {
            len += 1;
        }
        if self.vendor_id.is_some() {
            len += 1;
        }
        if self.expense_category_id.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.tax_amount.is_some() {
            len += 1;
        }
        if self.payment_method_id.is_some() {
            len += 1;
        }
        if self.reference_number.is_some() {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if !self.attachment_urls.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.UpdateExpenseRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.warehouse_id.as_ref() {
            struct_ser.serialize_field("warehouseId", v)?;
        }
        if let Some(v) = self.expense_date.as_ref() {
            struct_ser.serialize_field("expenseDate", v)?;
        }
        if let Some(v) = self.vendor_id.as_ref() {
            struct_ser.serialize_field("vendorId", v)?;
        }
        if let Some(v) = self.expense_category_id.as_ref() {
            struct_ser.serialize_field("expenseCategoryId", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.tax_amount.as_ref() {
            struct_ser.serialize_field("taxAmount", v)?;
        }
        if let Some(v) = self.payment_method_id.as_ref() {
            struct_ser.serialize_field("paymentMethodId", v)?;
        }
        if let Some(v) = self.reference_number.as_ref() {
            struct_ser.serialize_field("referenceNumber", v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if !self.attachment_urls.is_empty() {
            struct_ser.serialize_field("attachmentUrls", &self.attachment_urls)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateExpenseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "warehouse_id",
            "warehouseId",
            "expense_date",
            "expenseDate",
            "vendor_id",
            "vendorId",
            "expense_category_id",
            "expenseCategoryId",
            "description",
            "amount",
            "tax_amount",
            "taxAmount",
            "payment_method_id",
            "paymentMethodId",
            "reference_number",
            "referenceNumber",
            "notes",
            "attachment_urls",
            "attachmentUrls",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            WarehouseId,
            ExpenseDate,
            VendorId,
            ExpenseCategoryId,
            Description,
            Amount,
            TaxAmount,
            PaymentMethodId,
            ReferenceNumber,
            Notes,
            AttachmentUrls,
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
                            "expenseDate" | "expense_date" => Ok(GeneratedField::ExpenseDate),
                            "vendorId" | "vendor_id" => Ok(GeneratedField::VendorId),
                            "expenseCategoryId" | "expense_category_id" => Ok(GeneratedField::ExpenseCategoryId),
                            "description" => Ok(GeneratedField::Description),
                            "amount" => Ok(GeneratedField::Amount),
                            "taxAmount" | "tax_amount" => Ok(GeneratedField::TaxAmount),
                            "paymentMethodId" | "payment_method_id" => Ok(GeneratedField::PaymentMethodId),
                            "referenceNumber" | "reference_number" => Ok(GeneratedField::ReferenceNumber),
                            "notes" => Ok(GeneratedField::Notes),
                            "attachmentUrls" | "attachment_urls" => Ok(GeneratedField::AttachmentUrls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateExpenseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.UpdateExpenseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateExpenseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut warehouse_id__ = None;
                let mut expense_date__ = None;
                let mut vendor_id__ = None;
                let mut expense_category_id__ = None;
                let mut description__ = None;
                let mut amount__ = None;
                let mut tax_amount__ = None;
                let mut payment_method_id__ = None;
                let mut reference_number__ = None;
                let mut notes__ = None;
                let mut attachment_urls__ = None;
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
                        GeneratedField::ExpenseDate => {
                            if expense_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseDate"));
                            }
                            expense_date__ = map_.next_value()?;
                        }
                        GeneratedField::VendorId => {
                            if vendor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendorId"));
                            }
                            vendor_id__ = map_.next_value()?;
                        }
                        GeneratedField::ExpenseCategoryId => {
                            if expense_category_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expenseCategoryId"));
                            }
                            expense_category_id__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::TaxAmount => {
                            if tax_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxAmount"));
                            }
                            tax_amount__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethodId => {
                            if payment_method_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethodId"));
                            }
                            payment_method_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReferenceNumber => {
                            if reference_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceNumber"));
                            }
                            reference_number__ = map_.next_value()?;
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::AttachmentUrls => {
                            if attachment_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentUrls"));
                            }
                            attachment_urls__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateExpenseRequest {
                    id: id__.unwrap_or_default(),
                    warehouse_id: warehouse_id__,
                    expense_date: expense_date__,
                    vendor_id: vendor_id__,
                    expense_category_id: expense_category_id__,
                    description: description__,
                    amount: amount__,
                    tax_amount: tax_amount__,
                    payment_method_id: payment_method_id__,
                    reference_number: reference_number__,
                    notes: notes__,
                    attachment_urls: attachment_urls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.UpdateExpenseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateVendorRequest {
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
        if self.contact_person.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.tax_id.is_some() {
            len += 1;
        }
        if self.bank_account_details.is_some() {
            len += 1;
        }
        if self.default_expense_account_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if !self.translations_to_update.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.UpdateVendorRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.contact_person.as_ref() {
            struct_ser.serialize_field("contactPerson", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.tax_id.as_ref() {
            struct_ser.serialize_field("taxId", v)?;
        }
        if let Some(v) = self.bank_account_details.as_ref() {
            struct_ser.serialize_field("bankAccountDetails", v)?;
        }
        if let Some(v) = self.default_expense_account_id.as_ref() {
            struct_ser.serialize_field("defaultExpenseAccountId", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateVendorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "contact_person",
            "contactPerson",
            "email",
            "phone",
            "address",
            "tax_id",
            "taxId",
            "bank_account_details",
            "bankAccountDetails",
            "default_expense_account_id",
            "defaultExpenseAccountId",
            "is_active",
            "isActive",
            "translations_to_update",
            "translationsToUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            ContactPerson,
            Email,
            Phone,
            Address,
            TaxId,
            BankAccountDetails,
            DefaultExpenseAccountId,
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
                            "contactPerson" | "contact_person" => Ok(GeneratedField::ContactPerson),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "address" => Ok(GeneratedField::Address),
                            "taxId" | "tax_id" => Ok(GeneratedField::TaxId),
                            "bankAccountDetails" | "bank_account_details" => Ok(GeneratedField::BankAccountDetails),
                            "defaultExpenseAccountId" | "default_expense_account_id" => Ok(GeneratedField::DefaultExpenseAccountId),
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
            type Value = UpdateVendorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.UpdateVendorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateVendorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut contact_person__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut address__ = None;
                let mut tax_id__ = None;
                let mut bank_account_details__ = None;
                let mut default_expense_account_id__ = None;
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
                        GeneratedField::ContactPerson => {
                            if contact_person__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactPerson"));
                            }
                            contact_person__ = map_.next_value()?;
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
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::TaxId => {
                            if tax_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxId"));
                            }
                            tax_id__ = map_.next_value()?;
                        }
                        GeneratedField::BankAccountDetails => {
                            if bank_account_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankAccountDetails"));
                            }
                            bank_account_details__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultExpenseAccountId => {
                            if default_expense_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultExpenseAccountId"));
                            }
                            default_expense_account_id__ = map_.next_value()?;
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
                Ok(UpdateVendorRequest {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    contact_person: contact_person__,
                    email: email__,
                    phone: phone__,
                    address: address__,
                    tax_id: tax_id__,
                    bank_account_details: bank_account_details__,
                    default_expense_account_id: default_expense_account_id__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.UpdateVendorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vendor {
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
        if self.contact_person.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.tax_id.is_some() {
            len += 1;
        }
        if self.bank_account_details.is_some() {
            len += 1;
        }
        if self.default_expense_account_id.is_some() {
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
        if !self.notes.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.Vendor", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.contact_person.as_ref() {
            struct_ser.serialize_field("contactPerson", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.tax_id.as_ref() {
            struct_ser.serialize_field("taxId", v)?;
        }
        if let Some(v) = self.bank_account_details.as_ref() {
            struct_ser.serialize_field("bankAccountDetails", v)?;
        }
        if let Some(v) = self.default_expense_account_id.as_ref() {
            struct_ser.serialize_field("defaultExpenseAccountId", v)?;
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
        if !self.notes.is_empty() {
            struct_ser.serialize_field("notes", &self.notes)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vendor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
            "contact_person",
            "contactPerson",
            "email",
            "phone",
            "address",
            "tax_id",
            "taxId",
            "bank_account_details",
            "bankAccountDetails",
            "default_expense_account_id",
            "defaultExpenseAccountId",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "notes",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Code,
            ContactPerson,
            Email,
            Phone,
            Address,
            TaxId,
            BankAccountDetails,
            DefaultExpenseAccountId,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Notes,
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
                            "contactPerson" | "contact_person" => Ok(GeneratedField::ContactPerson),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "address" => Ok(GeneratedField::Address),
                            "taxId" | "tax_id" => Ok(GeneratedField::TaxId),
                            "bankAccountDetails" | "bank_account_details" => Ok(GeneratedField::BankAccountDetails),
                            "defaultExpenseAccountId" | "default_expense_account_id" => Ok(GeneratedField::DefaultExpenseAccountId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "notes" => Ok(GeneratedField::Notes),
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
            type Value = Vendor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.Vendor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vendor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                let mut contact_person__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut address__ = None;
                let mut tax_id__ = None;
                let mut bank_account_details__ = None;
                let mut default_expense_account_id__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut notes__ = None;
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
                        GeneratedField::ContactPerson => {
                            if contact_person__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactPerson"));
                            }
                            contact_person__ = map_.next_value()?;
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
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::TaxId => {
                            if tax_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taxId"));
                            }
                            tax_id__ = map_.next_value()?;
                        }
                        GeneratedField::BankAccountDetails => {
                            if bank_account_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankAccountDetails"));
                            }
                            bank_account_details__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultExpenseAccountId => {
                            if default_expense_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultExpenseAccountId"));
                            }
                            default_expense_account_id__ = map_.next_value()?;
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
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Vendor {
                    id: id__.unwrap_or_default(),
                    code: code__,
                    contact_person: contact_person__,
                    email: email__,
                    phone: phone__,
                    address: address__,
                    tax_id: tax_id__,
                    bank_account_details: bank_account_details__,
                    default_expense_account_id: default_expense_account_id__,
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    notes: notes__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.Vendor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VoidGeneralLedgerTransactionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transaction_id.is_empty() {
            len += 1;
        }
        if !self.void_reason.is_empty() {
            len += 1;
        }
        if self.voided_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.accounting.v1.VoidGeneralLedgerTransactionRequest", len)?;
        if !self.transaction_id.is_empty() {
            struct_ser.serialize_field("transactionId", &self.transaction_id)?;
        }
        if !self.void_reason.is_empty() {
            struct_ser.serialize_field("voidReason", &self.void_reason)?;
        }
        if let Some(v) = self.voided_by_user_id.as_ref() {
            struct_ser.serialize_field("voidedByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VoidGeneralLedgerTransactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_id",
            "transactionId",
            "void_reason",
            "voidReason",
            "voided_by_user_id",
            "voidedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionId,
            VoidReason,
            VoidedByUserId,
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
                            "transactionId" | "transaction_id" => Ok(GeneratedField::TransactionId),
                            "voidReason" | "void_reason" => Ok(GeneratedField::VoidReason),
                            "voidedByUserId" | "voided_by_user_id" => Ok(GeneratedField::VoidedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VoidGeneralLedgerTransactionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.accounting.v1.VoidGeneralLedgerTransactionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VoidGeneralLedgerTransactionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_id__ = None;
                let mut void_reason__ = None;
                let mut voided_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransactionId => {
                            if transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionId"));
                            }
                            transaction_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VoidReason => {
                            if void_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voidReason"));
                            }
                            void_reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VoidedByUserId => {
                            if voided_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voidedByUserId"));
                            }
                            voided_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VoidGeneralLedgerTransactionRequest {
                    transaction_id: transaction_id__.unwrap_or_default(),
                    void_reason: void_reason__.unwrap_or_default(),
                    voided_by_user_id: voided_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.accounting.v1.VoidGeneralLedgerTransactionRequest", FIELDS, GeneratedVisitor)
    }
}
