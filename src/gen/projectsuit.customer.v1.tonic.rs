// @generated
/// Generated client implementations.
pub mod customer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CustomerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustomerServiceClient<tonic::transport::Channel> {
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
    impl<T> CustomerServiceClient<T>
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
        ) -> CustomerServiceClient<InterceptedService<T, F>>
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
            CustomerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomerRequest>,
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
                "/projectsuit.customer.v1.CustomerService/CreateCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "CreateCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomerRequest>,
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
                "/projectsuit.customer.v1.CustomerService/UpdateCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "UpdateCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_customer(
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
                "/projectsuit.customer.v1.CustomerService/DeleteCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "DeleteCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_customer_address(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomerAddressRequest>,
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
                "/projectsuit.customer.v1.CustomerService/AddCustomerAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "AddCustomerAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_customer_address(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomerAddressRequest>,
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
                "/projectsuit.customer.v1.CustomerService/UpdateCustomerAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "UpdateCustomerAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_customer_address(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomerAddressRequest>,
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
                "/projectsuit.customer.v1.CustomerService/DeleteCustomerAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "DeleteCustomerAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_customer_address(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultCustomerAddressRequest>,
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
                "/projectsuit.customer.v1.CustomerService/SetDefaultCustomerAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "SetDefaultCustomerAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_customer(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::common::v1::GetByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
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
                "/projectsuit.customer.v1.CustomerService/GetCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "GetCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_customers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomersResponse>,
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
                "/projectsuit.customer.v1.CustomerService/ListCustomers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "ListCustomers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_customer_by_email(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerByEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
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
                "/projectsuit.customer.v1.CustomerService/GetCustomerByEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "GetCustomerByEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_customer_by_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerByPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
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
                "/projectsuit.customer.v1.CustomerService/GetCustomerByPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "projectsuit.customer.v1.CustomerService",
                        "GetCustomerByPhone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod customer_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CustomerServiceServer.
    #[async_trait]
    pub trait CustomerService: Send + Sync + 'static {
        async fn create_customer(
            &self,
            request: tonic::Request<super::CreateCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_customer(
            &self,
            request: tonic::Request<super::UpdateCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_customer(
            &self,
            request: tonic::Request<super::super::super::common::v1::DeleteByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn add_customer_address(
            &self,
            request: tonic::Request<super::AddCustomerAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::IdResponse>,
            tonic::Status,
        >;
        async fn update_customer_address(
            &self,
            request: tonic::Request<super::UpdateCustomerAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn delete_customer_address(
            &self,
            request: tonic::Request<super::DeleteCustomerAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn set_default_customer_address(
            &self,
            request: tonic::Request<super::SetDefaultCustomerAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v1::MutateResponse>,
            tonic::Status,
        >;
        async fn get_customer(
            &self,
            request: tonic::Request<super::super::super::common::v1::GetByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
            tonic::Status,
        >;
        async fn list_customers(
            &self,
            request: tonic::Request<super::ListCustomersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomersResponse>,
            tonic::Status,
        >;
        async fn get_customer_by_email(
            &self,
            request: tonic::Request<super::GetCustomerByEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
            tonic::Status,
        >;
        async fn get_customer_by_phone(
            &self,
            request: tonic::Request<super::GetCustomerByPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CustomerServiceServer<T: CustomerService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: CustomerService> CustomerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CustomerServiceServer<T>
    where
        T: CustomerService,
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
                "/projectsuit.customer.v1.CustomerService/CreateCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCustomerSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::CreateCustomerRequest>
                    for CreateCustomerSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCustomerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::create_customer(&inner, request)
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
                        let method = CreateCustomerSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/UpdateCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCustomerSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::UpdateCustomerRequest>
                    for UpdateCustomerSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCustomerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::update_customer(&inner, request)
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
                        let method = UpdateCustomerSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/DeleteCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCustomerSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::DeleteByIdRequest,
                    > for DeleteCustomerSvc<T> {
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
                                <T as CustomerService>::delete_customer(&inner, request)
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
                        let method = DeleteCustomerSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/AddCustomerAddress" => {
                    #[allow(non_camel_case_types)]
                    struct AddCustomerAddressSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::AddCustomerAddressRequest>
                    for AddCustomerAddressSvc<T> {
                        type Response = super::super::super::common::v1::IdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCustomerAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::add_customer_address(
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
                        let method = AddCustomerAddressSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/UpdateCustomerAddress" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCustomerAddressSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::UpdateCustomerAddressRequest>
                    for UpdateCustomerAddressSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCustomerAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::update_customer_address(
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
                        let method = UpdateCustomerAddressSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/DeleteCustomerAddress" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCustomerAddressSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::DeleteCustomerAddressRequest>
                    for DeleteCustomerAddressSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCustomerAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::delete_customer_address(
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
                        let method = DeleteCustomerAddressSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/SetDefaultCustomerAddress" => {
                    #[allow(non_camel_case_types)]
                    struct SetDefaultCustomerAddressSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<
                        super::SetDefaultCustomerAddressRequest,
                    > for SetDefaultCustomerAddressSvc<T> {
                        type Response = super::super::super::common::v1::MutateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetDefaultCustomerAddressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::set_default_customer_address(
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
                        let method = SetDefaultCustomerAddressSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/GetCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct GetCustomerSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<
                        super::super::super::common::v1::GetByIdRequest,
                    > for GetCustomerSvc<T> {
                        type Response = super::GetCustomerResponse;
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
                                <T as CustomerService>::get_customer(&inner, request).await
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
                        let method = GetCustomerSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/ListCustomers" => {
                    #[allow(non_camel_case_types)]
                    struct ListCustomersSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::ListCustomersRequest>
                    for ListCustomersSvc<T> {
                        type Response = super::ListCustomersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCustomersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::list_customers(&inner, request)
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
                        let method = ListCustomersSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/GetCustomerByEmail" => {
                    #[allow(non_camel_case_types)]
                    struct GetCustomerByEmailSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::GetCustomerByEmailRequest>
                    for GetCustomerByEmailSvc<T> {
                        type Response = super::GetCustomerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCustomerByEmailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::get_customer_by_email(
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
                        let method = GetCustomerByEmailSvc(inner);
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
                "/projectsuit.customer.v1.CustomerService/GetCustomerByPhone" => {
                    #[allow(non_camel_case_types)]
                    struct GetCustomerByPhoneSvc<T: CustomerService>(pub Arc<T>);
                    impl<
                        T: CustomerService,
                    > tonic::server::UnaryService<super::GetCustomerByPhoneRequest>
                    for GetCustomerByPhoneSvc<T> {
                        type Response = super::GetCustomerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCustomerByPhoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustomerService>::get_customer_by_phone(
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
                        let method = GetCustomerByPhoneSvc(inner);
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
    impl<T: CustomerService> Clone for CustomerServiceServer<T> {
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
    impl<T: CustomerService> tonic::server::NamedService for CustomerServiceServer<T> {
        const NAME: &'static str = "projectsuit.customer.v1.CustomerService";
    }
}
