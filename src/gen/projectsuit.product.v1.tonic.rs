// @generated
/// Generated client implementations.
pub mod product_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ProductServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProductServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProductServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProductServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ProductServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_product(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/CreateProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "CreateProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_product(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/UpdateProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "UpdateProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_product(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::DeleteByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/DeleteProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "DeleteProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_product_to_category(
            &mut self,
            request: impl tonic::IntoRequest<super::ProductCategoryLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/AddProductToCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "AddProductToCategory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_product_from_category(
            &mut self,
            request: impl tonic::IntoRequest<super::ProductCategoryLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/RemoveProductFromCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "RemoveProductFromCategory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_product_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductUnitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/CreateProductUnit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "CreateProductUnit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_product_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductUnitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/UpdateProductUnit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "UpdateProductUnit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_product_unit(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::DeleteByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/DeleteProductUnit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "DeleteProductUnit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_tag_to_product(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTagToProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/AddTagToProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "AddTagToProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_tag_from_product(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveTagFromProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/RemoveTagFromProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "RemoveTagFromProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_product_option(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductOptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/CreateProductOption",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "CreateProductOption",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_product_option(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductOptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/UpdateProductOption",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "UpdateProductOption",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_product_option(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::DeleteByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/DeleteProductOption",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "DeleteProductOption",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_product_option_value(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductOptionValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/CreateProductOptionValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "CreateProductOptionValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_product_option_value(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductOptionValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/UpdateProductOptionValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "UpdateProductOptionValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_product_option_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::DeleteByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/DeleteProductOptionValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "DeleteProductOptionValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_product_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/CreateProductVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "CreateProductVariant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_product_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/UpdateProductVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "UpdateProductVariant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_product_variant(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::DeleteByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/DeleteProductVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "DeleteProductVariant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn assign_option_value_to_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::AssignOptionValueToVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/AssignOptionValueToVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "AssignOptionValueToVariant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_option_value_from_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOptionValueFromVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/RemoveOptionValueFromVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "RemoveOptionValueFromVariant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProductResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/GetProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "GetProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/ListProducts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "ListProducts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_product_variant(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::GetByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetProductVariantResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/GetProductVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "GetProductVariant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_product_variants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductVariantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductVariantsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/projectsuit.product.v1.ProductService/ListProductVariants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.product.v1.ProductService",
                        "ListProductVariants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod product_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProductServiceServer.
    #[async_trait]
    pub trait ProductService: Send + Sync + 'static {
        async fn create_product(
            &self,
            request: tonic::Request<super::CreateProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_product(
            &self,
            request: tonic::Request<super::UpdateProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_product(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn add_product_to_category(
            &self,
            request: tonic::Request<super::ProductCategoryLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn remove_product_from_category(
            &self,
            request: tonic::Request<super::ProductCategoryLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn create_product_unit(
            &self,
            request: tonic::Request<super::CreateProductUnitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_product_unit(
            &self,
            request: tonic::Request<super::UpdateProductUnitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_product_unit(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn add_tag_to_product(
            &self,
            request: tonic::Request<super::AddTagToProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn remove_tag_from_product(
            &self,
            request: tonic::Request<super::RemoveTagFromProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn create_product_option(
            &self,
            request: tonic::Request<super::CreateProductOptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_product_option(
            &self,
            request: tonic::Request<super::UpdateProductOptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_product_option(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn create_product_option_value(
            &self,
            request: tonic::Request<super::CreateProductOptionValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_product_option_value(
            &self,
            request: tonic::Request<super::UpdateProductOptionValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_product_option_value(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn create_product_variant(
            &self,
            request: tonic::Request<super::CreateProductVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_product_variant(
            &self,
            request: tonic::Request<super::UpdateProductVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_product_variant(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn assign_option_value_to_variant(
            &self,
            request: tonic::Request<super::AssignOptionValueToVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn remove_option_value_from_variant(
            &self,
            request: tonic::Request<super::RemoveOptionValueFromVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn get_product(
            &self,
            request: tonic::Request<super::GetProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProductResponse>,
            tonic::Status,
        >;
        async fn list_products(
            &self,
            request: tonic::Request<super::ListProductsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductsResponse>,
            tonic::Status,
        >;
        async fn get_product_variant(
            &self,
            request: tonic::Request<super::super::super::common::v1::GetByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProductVariantResponse>,
            tonic::Status,
        >;
        async fn list_product_variants(
            &self,
            request: tonic::Request<super::ListProductVariantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductVariantsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ProductServiceServer<T: ProductService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: ProductService> ProductServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProductServiceServer<T>
    where
        T: ProductService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/projectsuit.product.v1.ProductService/CreateProduct" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::CreateProductRequest>
                    for CreateProductSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::create_product(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/UpdateProduct" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::UpdateProductRequest>
                    for UpdateProductSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::update_product(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/DeleteProduct" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteProductSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::common::v1::DeleteByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::delete_product(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/AddProductToCategory" => {
                    #[allow(non_camel_case_types)]
                    struct AddProductToCategorySvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::ProductCategoryLinkRequest>
                    for AddProductToCategorySvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProductCategoryLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::add_product_to_category(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddProductToCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/RemoveProductFromCategory" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveProductFromCategorySvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::ProductCategoryLinkRequest>
                    for RemoveProductFromCategorySvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProductCategoryLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::remove_product_from_category(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveProductFromCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/CreateProductUnit" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductUnitSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::CreateProductUnitRequest>
                    for CreateProductUnitSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductUnitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::create_product_unit(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateProductUnitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/UpdateProductUnit" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductUnitSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::UpdateProductUnitRequest>
                    for UpdateProductUnitSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductUnitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::update_product_unit(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateProductUnitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/DeleteProductUnit" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductUnitSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteProductUnitSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::common::v1::DeleteByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::delete_product_unit(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteProductUnitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/AddTagToProduct" => {
                    #[allow(non_camel_case_types)]
                    struct AddTagToProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::AddTagToProductRequest>
                    for AddTagToProductSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddTagToProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::add_tag_to_product(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddTagToProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/RemoveTagFromProduct" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTagFromProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::RemoveTagFromProductRequest>
                    for RemoveTagFromProductSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveTagFromProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::remove_tag_from_product(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveTagFromProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/CreateProductOption" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductOptionSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::CreateProductOptionRequest>
                    for CreateProductOptionSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductOptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::create_product_option(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateProductOptionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/UpdateProductOption" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductOptionSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::UpdateProductOptionRequest>
                    for UpdateProductOptionSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductOptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::update_product_option(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateProductOptionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/DeleteProductOption" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductOptionSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteProductOptionSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::common::v1::DeleteByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::delete_product_option(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteProductOptionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/CreateProductOptionValue" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductOptionValueSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::CreateProductOptionValueRequest>
                    for CreateProductOptionValueSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateProductOptionValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::create_product_option_value(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateProductOptionValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/UpdateProductOptionValue" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductOptionValueSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::UpdateProductOptionValueRequest>
                    for UpdateProductOptionValueSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateProductOptionValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::update_product_option_value(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateProductOptionValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/DeleteProductOptionValue" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductOptionValueSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteProductOptionValueSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::common::v1::DeleteByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::delete_product_option_value(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteProductOptionValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/CreateProductVariant" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductVariantSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::CreateProductVariantRequest>
                    for CreateProductVariantSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductVariantRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::create_product_variant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateProductVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/UpdateProductVariant" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductVariantSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::UpdateProductVariantRequest>
                    for UpdateProductVariantSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductVariantRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::update_product_variant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateProductVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/DeleteProductVariant" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductVariantSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteProductVariantSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::common::v1::DeleteByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::delete_product_variant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteProductVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/AssignOptionValueToVariant" => {
                    #[allow(non_camel_case_types)]
                    struct AssignOptionValueToVariantSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::AssignOptionValueToVariantRequest,
                    > for AssignOptionValueToVariantSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AssignOptionValueToVariantRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::assign_option_value_to_variant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AssignOptionValueToVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/RemoveOptionValueFromVariant" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveOptionValueFromVariantSvc<T: ProductService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::RemoveOptionValueFromVariantRequest,
                    > for RemoveOptionValueFromVariantSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveOptionValueFromVariantRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::remove_option_value_from_variant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveOptionValueFromVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/GetProduct" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::GetProductRequest>
                    for GetProductSvc<T> {
                        type Response = super::GetProductResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::get_product(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/ListProducts" => {
                    #[allow(non_camel_case_types)]
                    struct ListProductsSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::ListProductsRequest>
                    for ListProductsSvc<T> {
                        type Response = super::ListProductsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::list_products(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListProductsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/GetProductVariant" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductVariantSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::GetByIdRequest,
                    > for GetProductVariantSvc<T> {
                        type Response = super::GetProductVariantResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::common::v1::GetByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::get_product_variant(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetProductVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/projectsuit.product.v1.ProductService/ListProductVariants" => {
                    #[allow(non_camel_case_types)]
                    struct ListProductVariantsSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::ListProductVariantsRequest>
                    for ListProductVariantsSvc<T> {
                        type Response = super::ListProductVariantsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductVariantsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProductService>::list_product_variants(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListProductVariantsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ProductService> Clone for ProductServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ProductService> tonic::server::NamedService for ProductServiceServer<T> {
        const NAME: &'static str = "projectsuit.product.v1.ProductService";
    }
}
