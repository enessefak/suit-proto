// @generated
impl serde::Serialize for Category {
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
        if self.parent_id.is_some() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        if self.image_url.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
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
        if !self.slug.is_empty() {
            len += 1;
        }
        if !self.all_translations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.category.v1.Category", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.parent_id.as_ref() {
            struct_ser.serialize_field("parentId", v)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.image_url.as_ref() {
            struct_ser.serialize_field("imageUrl", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
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
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if !self.all_translations.is_empty() {
            struct_ser.serialize_field("allTranslations", &self.all_translations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Category {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "parent_id",
            "parentId",
            "code",
            "image_url",
            "imageUrl",
            "icon_url",
            "iconUrl",
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
            "slug",
            "all_translations",
            "allTranslations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ParentId,
            Code,
            ImageUrl,
            IconUrl,
            SortOrder,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            Slug,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "code" => Ok(GeneratedField::Code),
                            "imageUrl" | "image_url" => Ok(GeneratedField::ImageUrl),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "sortOrder" | "sort_order" => Ok(GeneratedField::SortOrder),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "slug" => Ok(GeneratedField::Slug),
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
            type Value = Category;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.category.v1.Category")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Category, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut parent_id__ = None;
                let mut code__ = None;
                let mut image_url__ = None;
                let mut icon_url__ = None;
                let mut sort_order__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut slug__ = None;
                let mut all_translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = map_.next_value()?;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::ImageUrl => {
                            if image_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrl"));
                            }
                            image_url__ = map_.next_value()?;
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
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
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllTranslations => {
                            if all_translations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allTranslations"));
                            }
                            all_translations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Category {
                    id: id__.unwrap_or_default(),
                    parent_id: parent_id__,
                    code: code__,
                    image_url: image_url__,
                    icon_url: icon_url__,
                    sort_order: sort_order__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    all_translations: all_translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.category.v1.Category", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCategoryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parent_id.is_some() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        if self.image_url.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.category.v1.CreateCategoryRequest", len)?;
        if let Some(v) = self.parent_id.as_ref() {
            struct_ser.serialize_field("parentId", v)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.image_url.as_ref() {
            struct_ser.serialize_field("imageUrl", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
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
impl<'de> serde::Deserialize<'de> for CreateCategoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_id",
            "parentId",
            "code",
            "image_url",
            "imageUrl",
            "icon_url",
            "iconUrl",
            "sort_order",
            "sortOrder",
            "is_active",
            "isActive",
            "translations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
            Code,
            ImageUrl,
            IconUrl,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "code" => Ok(GeneratedField::Code),
                            "imageUrl" | "image_url" => Ok(GeneratedField::ImageUrl),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
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
            type Value = CreateCategoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.category.v1.CreateCategoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCategoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                let mut code__ = None;
                let mut image_url__ = None;
                let mut icon_url__ = None;
                let mut sort_order__ = None;
                let mut is_active__ = None;
                let mut translations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = map_.next_value()?;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::ImageUrl => {
                            if image_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrl"));
                            }
                            image_url__ = map_.next_value()?;
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
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
                Ok(CreateCategoryRequest {
                    parent_id: parent_id__,
                    code: code__,
                    image_url: image_url__,
                    icon_url: icon_url__,
                    sort_order: sort_order__,
                    is_active: is_active__,
                    translations: translations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.category.v1.CreateCategoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCategoryResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.category.v1.GetCategoryResponse", len)?;
        if let Some(v) = self.category.as_ref() {
            struct_ser.serialize_field("category", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCategoryResponse {
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
            type Value = GetCategoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.category.v1.GetCategoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCategoryResponse, V::Error>
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
                Ok(GetCategoryResponse {
                    category: category__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.category.v1.GetCategoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCategoriesRequest {
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
        if self.filter_by_parent_id.is_some() {
            len += 1;
        }
        if self.filter_by_is_root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("projectsuit.category.v1.ListCategoriesRequest", len)?;
        if let Some(v) = self.base_request.as_ref() {
            struct_ser.serialize_field("baseRequest", v)?;
        }
        if let Some(v) = self.filter_by_parent_id.as_ref() {
            struct_ser.serialize_field("filterByParentId", v)?;
        }
        if let Some(v) = self.filter_by_is_root.as_ref() {
            struct_ser.serialize_field("filterByIsRoot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCategoriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_request",
            "baseRequest",
            "filter_by_parent_id",
            "filterByParentId",
            "filter_by_is_root",
            "filterByIsRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseRequest,
            FilterByParentId,
            FilterByIsRoot,
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
                            "filterByParentId" | "filter_by_parent_id" => Ok(GeneratedField::FilterByParentId),
                            "filterByIsRoot" | "filter_by_is_root" => Ok(GeneratedField::FilterByIsRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListCategoriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.category.v1.ListCategoriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCategoriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_request__ = None;
                let mut filter_by_parent_id__ = None;
                let mut filter_by_is_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseRequest => {
                            if base_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseRequest"));
                            }
                            base_request__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByParentId => {
                            if filter_by_parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByParentId"));
                            }
                            filter_by_parent_id__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByIsRoot => {
                            if filter_by_is_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByIsRoot"));
                            }
                            filter_by_is_root__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListCategoriesRequest {
                    base_request: base_request__,
                    filter_by_parent_id: filter_by_parent_id__,
                    filter_by_is_root: filter_by_is_root__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.category.v1.ListCategoriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCategoriesResponse {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.category.v1.ListCategoriesResponse", len)?;
        if !self.categories.is_empty() {
            struct_ser.serialize_field("categories", &self.categories)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCategoriesResponse {
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
            type Value = ListCategoriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.category.v1.ListCategoriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCategoriesResponse, V::Error>
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
                Ok(ListCategoriesResponse {
                    categories: categories__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.category.v1.ListCategoriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCategoryRequest {
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
        if self.parent_id.is_some() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        if self.image_url.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("projectsuit.category.v1.UpdateCategoryRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.parent_id.as_ref() {
            struct_ser.serialize_field("parentId", v)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.image_url.as_ref() {
            struct_ser.serialize_field("imageUrl", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
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
impl<'de> serde::Deserialize<'de> for UpdateCategoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "parent_id",
            "parentId",
            "code",
            "image_url",
            "imageUrl",
            "icon_url",
            "iconUrl",
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
            ParentId,
            Code,
            ImageUrl,
            IconUrl,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "code" => Ok(GeneratedField::Code),
                            "imageUrl" | "image_url" => Ok(GeneratedField::ImageUrl),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
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
            type Value = UpdateCategoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct projectsuit.category.v1.UpdateCategoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCategoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut parent_id__ = None;
                let mut code__ = None;
                let mut image_url__ = None;
                let mut icon_url__ = None;
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
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = map_.next_value()?;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::ImageUrl => {
                            if image_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrl"));
                            }
                            image_url__ = map_.next_value()?;
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
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
                Ok(UpdateCategoryRequest {
                    id: id__.unwrap_or_default(),
                    parent_id: parent_id__,
                    code: code__,
                    image_url: image_url__,
                    icon_url: icon_url__,
                    sort_order: sort_order__,
                    is_active: is_active__,
                    translations_to_update: translations_to_update__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("projectsuit.category.v1.UpdateCategoryRequest", FIELDS, GeneratedVisitor)
    }
}
