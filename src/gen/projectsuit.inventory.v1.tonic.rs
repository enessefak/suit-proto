// @generated
/// Generated client implementations.
pub mod inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InventoryServiceClient<tonic::transport::Channel> {
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
    impl<T> InventoryServiceClient<T>
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
        ) -> InventoryServiceClient<InterceptedService<T, F>>
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
            InventoryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_warehouse(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWarehouseRequest>,
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
                "/projectsuit.inventory.v1.InventoryService/CreateWarehouse",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "CreateWarehouse",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_warehouse(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWarehouseRequest>,
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
                "/projectsuit.inventory.v1.InventoryService/UpdateWarehouse",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "UpdateWarehouse",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_warehouse(
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
                "/projectsuit.inventory.v1.InventoryService/DeleteWarehouse",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "DeleteWarehouse",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn adjust_stock_level(
            &mut self,
            request: impl tonic::IntoRequest<super::AdjustStockLevelRequest>,
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
                "/projectsuit.inventory.v1.InventoryService/AdjustStockLevel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "AdjustStockLevel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn record_stock_movement(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordStockMovementRequest>,
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
                "/projectsuit.inventory.v1.InventoryService/RecordStockMovement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "RecordStockMovement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_warehouse(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::GetByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetWarehouseResponse>,
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
                "/projectsuit.inventory.v1.InventoryService/GetWarehouse",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "GetWarehouse",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_warehouses(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWarehousesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWarehousesResponse>,
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
                "/projectsuit.inventory.v1.InventoryService/ListWarehouses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "ListWarehouses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_stock_level(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStockLevelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStockLevelResponse>,
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
                "/projectsuit.inventory.v1.InventoryService/GetStockLevel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "GetStockLevel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_stock_levels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStockLevelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStockLevelsResponse>,
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
                "/projectsuit.inventory.v1.InventoryService/ListStockLevels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "ListStockLevels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_stock_movements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStockMovementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStockMovementsResponse>,
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
                "/projectsuit.inventory.v1.InventoryService/ListStockMovements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.inventory.v1.InventoryService",
                        "ListStockMovements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod inventory_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InventoryServiceServer.
    #[async_trait]
    pub trait InventoryService: Send + Sync + 'static {
        async fn create_warehouse(
            &self,
            request: tonic::Request<super::CreateWarehouseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_warehouse(
            &self,
            request: tonic::Request<super::UpdateWarehouseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_warehouse(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn adjust_stock_level(
            &self,
            request: tonic::Request<super::AdjustStockLevelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn record_stock_movement(
            &self,
            request: tonic::Request<super::RecordStockMovementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn get_warehouse(
            &self,
            request: tonic::Request<super::super::super::common::v1::GetByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWarehouseResponse>,
            tonic::Status,
        >;
        async fn list_warehouses(
            &self,
            request: tonic::Request<super::ListWarehousesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWarehousesResponse>,
            tonic::Status,
        >;
        async fn get_stock_level(
            &self,
            request: tonic::Request<super::GetStockLevelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStockLevelResponse>,
            tonic::Status,
        >;
        async fn list_stock_levels(
            &self,
            request: tonic::Request<super::ListStockLevelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStockLevelsResponse>,
            tonic::Status,
        >;
        async fn list_stock_movements(
            &self,
            request: tonic::Request<super::ListStockMovementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStockMovementsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct InventoryServiceServer<T: InventoryService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: InventoryService> InventoryServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InventoryServiceServer<T>
    where
        T: InventoryService,
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
                "/projectsuit.inventory.v1.InventoryService/CreateWarehouse" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWarehouseSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::CreateWarehouseRequest>
                    for CreateWarehouseSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWarehouseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::create_warehouse(&inner, request)
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
                        let method = CreateWarehouseSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/UpdateWarehouse" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWarehouseSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::UpdateWarehouseRequest>
                    for UpdateWarehouseSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateWarehouseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::update_warehouse(&inner, request)
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
                        let method = UpdateWarehouseSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/DeleteWarehouse" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWarehouseSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteWarehouseSvc<T> {
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
                                <T as InventoryService>::delete_warehouse(&inner, request)
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
                        let method = DeleteWarehouseSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/AdjustStockLevel" => {
                    #[allow(non_camel_case_types)]
                    struct AdjustStockLevelSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::AdjustStockLevelRequest>
                    for AdjustStockLevelSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdjustStockLevelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::adjust_stock_level(&inner, request)
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
                        let method = AdjustStockLevelSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/RecordStockMovement" => {
                    #[allow(non_camel_case_types)]
                    struct RecordStockMovementSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::RecordStockMovementRequest>
                    for RecordStockMovementSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordStockMovementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::record_stock_movement(
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
                        let method = RecordStockMovementSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/GetWarehouse" => {
                    #[allow(non_camel_case_types)]
                    struct GetWarehouseSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::GetByIdRequest,
                    > for GetWarehouseSvc<T> {
                        type Response = super::GetWarehouseResponse;
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
                                <T as InventoryService>::get_warehouse(&inner, request)
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
                        let method = GetWarehouseSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/ListWarehouses" => {
                    #[allow(non_camel_case_types)]
                    struct ListWarehousesSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::ListWarehousesRequest>
                    for ListWarehousesSvc<T> {
                        type Response = super::ListWarehousesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWarehousesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::list_warehouses(&inner, request)
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
                        let method = ListWarehousesSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/GetStockLevel" => {
                    #[allow(non_camel_case_types)]
                    struct GetStockLevelSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::GetStockLevelRequest>
                    for GetStockLevelSvc<T> {
                        type Response = super::GetStockLevelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStockLevelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::get_stock_level(&inner, request)
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
                        let method = GetStockLevelSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/ListStockLevels" => {
                    #[allow(non_camel_case_types)]
                    struct ListStockLevelsSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::ListStockLevelsRequest>
                    for ListStockLevelsSvc<T> {
                        type Response = super::ListStockLevelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListStockLevelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::list_stock_levels(&inner, request)
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
                        let method = ListStockLevelsSvc(inner);
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
                "/projectsuit.inventory.v1.InventoryService/ListStockMovements" => {
                    #[allow(non_camel_case_types)]
                    struct ListStockMovementsSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::ListStockMovementsRequest>
                    for ListStockMovementsSvc<T> {
                        type Response = super::ListStockMovementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListStockMovementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InventoryService>::list_stock_movements(
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
                        let method = ListStockMovementsSvc(inner);
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
    impl<T: InventoryService> Clone for InventoryServiceServer<T> {
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
    impl<T: InventoryService> tonic::server::NamedService for InventoryServiceServer<T> {
        const NAME: &'static str = "projectsuit.inventory.v1.InventoryService";
    }
}
