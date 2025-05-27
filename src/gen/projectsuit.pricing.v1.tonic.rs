// @generated
/// Generated client implementations.
pub mod pricing_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PricingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PricingServiceClient<tonic::transport::Channel> {
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
    impl<T> PricingServiceClient<T>
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
        ) -> PricingServiceClient<InterceptedService<T, F>>
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
            PricingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_price_list(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePriceListRequest>,
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
                "/projectsuit.pricing.v1.PricingService/CreatePriceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "CreatePriceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_price_list(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePriceListRequest>,
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
                "/projectsuit.pricing.v1.PricingService/UpdatePriceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "UpdatePriceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_price_list(
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
                "/projectsuit.pricing.v1.PricingService/DeletePriceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "DeletePriceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_item_to_price_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AddItemToPriceListRequest>,
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
                "/projectsuit.pricing.v1.PricingService/AddItemToPriceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "AddItemToPriceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_price_list_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePriceListItemRequest>,
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
                "/projectsuit.pricing.v1.PricingService/UpdatePriceListItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "UpdatePriceListItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_price_list_item(
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
                "/projectsuit.pricing.v1.PricingService/RemovePriceListItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "RemovePriceListItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn assign_price_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AssignPriceListRequest>,
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
                "/projectsuit.pricing.v1.PricingService/AssignPriceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "AssignPriceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_price_list_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePriceListAssignmentRequest>,
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
                "/projectsuit.pricing.v1.PricingService/UpdatePriceListAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "UpdatePriceListAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_price_list_assignment(
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
                "/projectsuit.pricing.v1.PricingService/RemovePriceListAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "RemovePriceListAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_promotion(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePromotionRequest>,
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
                "/projectsuit.pricing.v1.PricingService/CreatePromotion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "CreatePromotion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_promotion(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePromotionRequest>,
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
                "/projectsuit.pricing.v1.PricingService/UpdatePromotion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "UpdatePromotion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_promotion(
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
                "/projectsuit.pricing.v1.PricingService/DeletePromotion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "DeletePromotion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_coupon(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCouponRequest>,
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
                "/projectsuit.pricing.v1.PricingService/CreateCoupon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "CreateCoupon",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_coupon(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCouponRequest>,
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
                "/projectsuit.pricing.v1.PricingService/UpdateCoupon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "UpdateCoupon",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_coupon(
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
                "/projectsuit.pricing.v1.PricingService/DeleteCoupon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "DeleteCoupon",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_price_list(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::GetByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetPriceListResponse>,
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
                "/projectsuit.pricing.v1.PricingService/GetPriceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "GetPriceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_price_lists(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPriceListsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPriceListsResponse>,
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
                "/projectsuit.pricing.v1.PricingService/ListPriceLists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "ListPriceLists",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_promotion(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::GetByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetPromotionResponse>,
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
                "/projectsuit.pricing.v1.PricingService/GetPromotion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "GetPromotion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_promotions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPromotionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPromotionsResponse>,
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
                "/projectsuit.pricing.v1.PricingService/ListPromotions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "ListPromotions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_coupon_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCouponByCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCouponResponse>,
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
                "/projectsuit.pricing.v1.PricingService/GetCouponByCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "GetCouponByCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_coupons(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCouponsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCouponsResponse>,
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
                "/projectsuit.pricing.v1.PricingService/ListCoupons",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.pricing.v1.PricingService",
                        "ListCoupons",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod pricing_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PricingServiceServer.
    #[async_trait]
    pub trait PricingService: Send + Sync + 'static {
        async fn create_price_list(
            &self,
            request: tonic::Request<super::CreatePriceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_price_list(
            &self,
            request: tonic::Request<super::UpdatePriceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_price_list(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn add_item_to_price_list(
            &self,
            request: tonic::Request<super::AddItemToPriceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_price_list_item(
            &self,
            request: tonic::Request<super::UpdatePriceListItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn remove_price_list_item(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn assign_price_list(
            &self,
            request: tonic::Request<super::AssignPriceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_price_list_assignment(
            &self,
            request: tonic::Request<super::UpdatePriceListAssignmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn remove_price_list_assignment(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn create_promotion(
            &self,
            request: tonic::Request<super::CreatePromotionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_promotion(
            &self,
            request: tonic::Request<super::UpdatePromotionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_promotion(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn create_coupon(
            &self,
            request: tonic::Request<super::CreateCouponRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_coupon(
            &self,
            request: tonic::Request<super::UpdateCouponRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_coupon(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn get_price_list(
            &self,
            request: tonic::Request<super::super::super::common::v1::GetByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPriceListResponse>,
            tonic::Status,
        >;
        async fn list_price_lists(
            &self,
            request: tonic::Request<super::ListPriceListsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPriceListsResponse>,
            tonic::Status,
        >;
        async fn get_promotion(
            &self,
            request: tonic::Request<super::super::super::common::v1::GetByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPromotionResponse>,
            tonic::Status,
        >;
        async fn list_promotions(
            &self,
            request: tonic::Request<super::ListPromotionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPromotionsResponse>,
            tonic::Status,
        >;
        async fn get_coupon_by_code(
            &self,
            request: tonic::Request<super::GetCouponByCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCouponResponse>,
            tonic::Status,
        >;
        async fn list_coupons(
            &self,
            request: tonic::Request<super::ListCouponsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCouponsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PricingServiceServer<T: PricingService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: PricingService> PricingServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PricingServiceServer<T>
    where
        T: PricingService,
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
                "/projectsuit.pricing.v1.PricingService/CreatePriceList" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePriceListSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::CreatePriceListRequest>
                    for CreatePriceListSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePriceListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::create_price_list(&inner, request)
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
                        let method = CreatePriceListSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/UpdatePriceList" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePriceListSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::UpdatePriceListRequest>
                    for UpdatePriceListSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePriceListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::update_price_list(&inner, request)
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
                        let method = UpdatePriceListSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/DeletePriceList" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePriceListSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeletePriceListSvc<T> {
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
                                <T as PricingService>::delete_price_list(&inner, request)
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
                        let method = DeletePriceListSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/AddItemToPriceList" => {
                    #[allow(non_camel_case_types)]
                    struct AddItemToPriceListSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::AddItemToPriceListRequest>
                    for AddItemToPriceListSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddItemToPriceListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::add_item_to_price_list(
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
                        let method = AddItemToPriceListSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/UpdatePriceListItem" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePriceListItemSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::UpdatePriceListItemRequest>
                    for UpdatePriceListItemSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePriceListItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::update_price_list_item(
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
                        let method = UpdatePriceListItemSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/RemovePriceListItem" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePriceListItemSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for RemovePriceListItemSvc<T> {
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
                                <T as PricingService>::remove_price_list_item(
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
                        let method = RemovePriceListItemSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/AssignPriceList" => {
                    #[allow(non_camel_case_types)]
                    struct AssignPriceListSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::AssignPriceListRequest>
                    for AssignPriceListSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssignPriceListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::assign_price_list(&inner, request)
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
                        let method = AssignPriceListSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/UpdatePriceListAssignment" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePriceListAssignmentSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::UpdatePriceListAssignmentRequest,
                    > for UpdatePriceListAssignmentSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdatePriceListAssignmentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::update_price_list_assignment(
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
                        let method = UpdatePriceListAssignmentSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/RemovePriceListAssignment" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePriceListAssignmentSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for RemovePriceListAssignmentSvc<T> {
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
                                <T as PricingService>::remove_price_list_assignment(
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
                        let method = RemovePriceListAssignmentSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/CreatePromotion" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePromotionSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::CreatePromotionRequest>
                    for CreatePromotionSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePromotionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::create_promotion(&inner, request)
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
                        let method = CreatePromotionSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/UpdatePromotion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePromotionSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::UpdatePromotionRequest>
                    for UpdatePromotionSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePromotionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::update_promotion(&inner, request)
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
                        let method = UpdatePromotionSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/DeletePromotion" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePromotionSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeletePromotionSvc<T> {
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
                                <T as PricingService>::delete_promotion(&inner, request)
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
                        let method = DeletePromotionSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/CreateCoupon" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCouponSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::CreateCouponRequest>
                    for CreateCouponSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCouponRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::create_coupon(&inner, request).await
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
                        let method = CreateCouponSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/UpdateCoupon" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCouponSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::UpdateCouponRequest>
                    for UpdateCouponSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCouponRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::update_coupon(&inner, request).await
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
                        let method = UpdateCouponSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/DeleteCoupon" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCouponSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteCouponSvc<T> {
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
                                <T as PricingService>::delete_coupon(&inner, request).await
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
                        let method = DeleteCouponSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/GetPriceList" => {
                    #[allow(non_camel_case_types)]
                    struct GetPriceListSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::GetByIdRequest,
                    > for GetPriceListSvc<T> {
                        type Response = super::GetPriceListResponse;
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
                                <T as PricingService>::get_price_list(&inner, request).await
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
                        let method = GetPriceListSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/ListPriceLists" => {
                    #[allow(non_camel_case_types)]
                    struct ListPriceListsSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::ListPriceListsRequest>
                    for ListPriceListsSvc<T> {
                        type Response = super::ListPriceListsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPriceListsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::list_price_lists(&inner, request)
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
                        let method = ListPriceListsSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/GetPromotion" => {
                    #[allow(non_camel_case_types)]
                    struct GetPromotionSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::GetByIdRequest,
                    > for GetPromotionSvc<T> {
                        type Response = super::GetPromotionResponse;
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
                                <T as PricingService>::get_promotion(&inner, request).await
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
                        let method = GetPromotionSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/ListPromotions" => {
                    #[allow(non_camel_case_types)]
                    struct ListPromotionsSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::ListPromotionsRequest>
                    for ListPromotionsSvc<T> {
                        type Response = super::ListPromotionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPromotionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::list_promotions(&inner, request)
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
                        let method = ListPromotionsSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/GetCouponByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetCouponByCodeSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::GetCouponByCodeRequest>
                    for GetCouponByCodeSvc<T> {
                        type Response = super::GetCouponResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCouponByCodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::get_coupon_by_code(&inner, request)
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
                        let method = GetCouponByCodeSvc(inner);
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
                "/projectsuit.pricing.v1.PricingService/ListCoupons" => {
                    #[allow(non_camel_case_types)]
                    struct ListCouponsSvc<T: PricingService>(pub Arc<T>);
                    impl<
                        T: PricingService,
                    > tonic::server::UnaryService<super::ListCouponsRequest>
                    for ListCouponsSvc<T> {
                        type Response = super::ListCouponsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCouponsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PricingService>::list_coupons(&inner, request).await
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
                        let method = ListCouponsSvc(inner);
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
    impl<T: PricingService> Clone for PricingServiceServer<T> {
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
    impl<T: PricingService> tonic::server::NamedService for PricingServiceServer<T> {
        const NAME: &'static str = "projectsuit.pricing.v1.PricingService";
    }
}
