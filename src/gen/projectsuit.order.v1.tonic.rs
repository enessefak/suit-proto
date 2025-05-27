// @generated
/// Generated client implementations.
pub mod order_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServiceClient<tonic::transport::Channel> {
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
    impl<T> OrderServiceClient<T>
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
        ) -> OrderServiceClient<InterceptedService<T, F>>
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
            OrderServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrderRequest>,
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
                "/projectsuit.order.v1.OrderService/CreateOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("projectsuit.order.v1.OrderService", "CreateOrder"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_order_status(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrderStatusRequest>,
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
                "/projectsuit.order.v1.OrderService/UpdateOrderStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "UpdateOrderStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_item_to_order(
            &mut self,
            request: impl tonic::IntoRequest<super::AddItemToOrderRequest>,
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
                "/projectsuit.order.v1.OrderService/AddItemToOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "AddItemToOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_order_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrderItemRequest>,
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
                "/projectsuit.order.v1.OrderService/UpdateOrderItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "UpdateOrderItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_order_item(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOrderItemRequest>,
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
                "/projectsuit.order.v1.OrderService/RemoveOrderItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "RemoveOrderItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn apply_promotion_to_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyPromotionToOrderRequest>,
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
                "/projectsuit.order.v1.OrderService/ApplyPromotionToOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "ApplyPromotionToOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn apply_coupon_to_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyCouponToOrderRequest>,
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
                "/projectsuit.order.v1.OrderService/ApplyCouponToOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "ApplyCouponToOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn record_payment_for_order(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordPaymentForOrderRequest>,
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
                "/projectsuit.order.v1.OrderService/RecordPaymentForOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.order.v1.OrderService",
                        "RecordPaymentForOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::GetByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
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
                "/projectsuit.order.v1.OrderService/GetOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("projectsuit.order.v1.OrderService", "GetOrder"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrdersResponse>,
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
                "/projectsuit.order.v1.OrderService/ListOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("projectsuit.order.v1.OrderService", "ListOrders"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod order_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrderServiceServer.
    #[async_trait]
    pub trait OrderService: Send + Sync + 'static {
        async fn create_order(
            &self,
            request: tonic::Request<super::CreateOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_order_status(
            &self,
            request: tonic::Request<super::UpdateOrderStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn add_item_to_order(
            &self,
            request: tonic::Request<super::AddItemToOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_order_item(
            &self,
            request: tonic::Request<super::UpdateOrderItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn remove_order_item(
            &self,
            request: tonic::Request<super::RemoveOrderItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn apply_promotion_to_order(
            &self,
            request: tonic::Request<super::ApplyPromotionToOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn apply_coupon_to_order(
            &self,
            request: tonic::Request<super::ApplyCouponToOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn record_payment_for_order(
            &self,
            request: tonic::Request<super::RecordPaymentForOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn get_order(
            &self,
            request: tonic::Request<super::super::super::common::v1::GetByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
            tonic::Status,
        >;
        async fn list_orders(
            &self,
            request: tonic::Request<super::ListOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrdersResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OrderServiceServer<T: OrderService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: OrderService> OrderServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderServiceServer<T>
    where
        T: OrderService,
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
                "/projectsuit.order.v1.OrderService/CreateOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::CreateOrderRequest>
                    for CreateOrderSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::create_order(&inner, request).await
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
                        let method = CreateOrderSvc(inner);
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
                "/projectsuit.order.v1.OrderService/UpdateOrderStatus" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrderStatusSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::UpdateOrderStatusRequest>
                    for UpdateOrderStatusSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrderStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::update_order_status(&inner, request)
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
                        let method = UpdateOrderStatusSvc(inner);
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
                "/projectsuit.order.v1.OrderService/AddItemToOrder" => {
                    #[allow(non_camel_case_types)]
                    struct AddItemToOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::AddItemToOrderRequest>
                    for AddItemToOrderSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddItemToOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::add_item_to_order(&inner, request)
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
                        let method = AddItemToOrderSvc(inner);
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
                "/projectsuit.order.v1.OrderService/UpdateOrderItem" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrderItemSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::UpdateOrderItemRequest>
                    for UpdateOrderItemSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrderItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::update_order_item(&inner, request)
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
                        let method = UpdateOrderItemSvc(inner);
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
                "/projectsuit.order.v1.OrderService/RemoveOrderItem" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveOrderItemSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::RemoveOrderItemRequest>
                    for RemoveOrderItemSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveOrderItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::remove_order_item(&inner, request)
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
                        let method = RemoveOrderItemSvc(inner);
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
                "/projectsuit.order.v1.OrderService/ApplyPromotionToOrder" => {
                    #[allow(non_camel_case_types)]
                    struct ApplyPromotionToOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::ApplyPromotionToOrderRequest>
                    for ApplyPromotionToOrderSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApplyPromotionToOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::apply_promotion_to_order(
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
                        let method = ApplyPromotionToOrderSvc(inner);
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
                "/projectsuit.order.v1.OrderService/ApplyCouponToOrder" => {
                    #[allow(non_camel_case_types)]
                    struct ApplyCouponToOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::ApplyCouponToOrderRequest>
                    for ApplyCouponToOrderSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApplyCouponToOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::apply_coupon_to_order(&inner, request)
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
                        let method = ApplyCouponToOrderSvc(inner);
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
                "/projectsuit.order.v1.OrderService/RecordPaymentForOrder" => {
                    #[allow(non_camel_case_types)]
                    struct RecordPaymentForOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::RecordPaymentForOrderRequest>
                    for RecordPaymentForOrderSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordPaymentForOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::record_payment_for_order(
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
                        let method = RecordPaymentForOrderSvc(inner);
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
                "/projectsuit.order.v1.OrderService/GetOrder" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::GetByIdRequest,
                    > for GetOrderSvc<T> {
                        type Response = super::GetOrderResponse;
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
                                <T as OrderService>::get_order(&inner, request).await
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
                        let method = GetOrderSvc(inner);
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
                "/projectsuit.order.v1.OrderService/ListOrders" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrdersSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::ListOrdersRequest>
                    for ListOrdersSvc<T> {
                        type Response = super::ListOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrderService>::list_orders(&inner, request).await
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
                        let method = ListOrdersSvc(inner);
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
    impl<T: OrderService> Clone for OrderServiceServer<T> {
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
    impl<T: OrderService> tonic::server::NamedService for OrderServiceServer<T> {
        const NAME: &'static str = "projectsuit.order.v1.OrderService";
    }
}
