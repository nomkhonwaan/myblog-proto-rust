/// A written remark expressing an opinion or reaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// Identifier of the comment 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Status of the comment which could be...
    /// - Published
    /// - Deleted
    #[prost(enumeration="CommentStatus", tag="2")]
    pub status: i32,
    /// Content of the comment in plain text format
    #[prost(string, tag="3")]
    pub text: ::prost::alloc::string::String,
    /// A commentator
    #[prost(message, optional, tag="4")]
    pub author: ::core::option::Option<super::auth::User>,
    /// Identifier of the parent comment,
    /// this could be null if the comment is not belonging to the other
    #[prost(string, tag="5")]
    pub parent_id: ::prost::alloc::string::String,
    /// List of comments were replied to this comment
    #[prost(message, repeated, tag="6")]
    pub children: ::prost::alloc::vec::Vec<Comment>,
    /// Date-time that the comment was created
    #[prost(message, optional, tag="7")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Date-time that the comment was updated
    #[prost(message, optional, tag="8")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommentStatus {
    Published = 0,
    Deleted = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommentRequest {
    #[prost(message, optional, tag="1")]
    pub comment: ::core::option::Option<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommentResponse {
    #[prost(message, optional, tag="1")]
    pub comment: ::core::option::Option<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublishedCommentsRequest {
    #[prost(uint32, tag="1")]
    pub offset: u32,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    /// Identifier of the parent comment or post that the comment belonging to
    #[prost(string, tag="3")]
    pub parent_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublishedCommentsResponse {
    #[prost(message, repeated, tag="1")]
    pub comments: ::prost::alloc::vec::Vec<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCommentRequest {
    #[prost(string, tag="1")]
    pub comment_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCommentResponse {
    #[prost(message, optional, tag="1")]
    pub comment: ::core::option::Option<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCommentRequest {
    #[prost(message, optional, tag="1")]
    pub comment: ::core::option::Option<Comment>,
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCommentResponse {
    #[prost(message, optional, tag="1")]
    pub comment: ::core::option::Option<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCommentRequest {
    #[prost(string, tag="1")]
    pub comment_id: ::prost::alloc::string::String,
}
/// Generated server implementations.
pub mod discussion_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DiscussionServiceServer.
    #[async_trait]
    pub trait DiscussionService: Send + Sync + 'static {
        async fn create_comment(
            &self,
            request: tonic::Request<super::CreateCommentRequest>,
        ) -> Result<tonic::Response<super::CreateCommentResponse>, tonic::Status>;
        async fn list_published_comments(
            &self,
            request: tonic::Request<super::ListPublishedCommentsRequest>,
        ) -> Result<
                tonic::Response<super::ListPublishedCommentsResponse>,
                tonic::Status,
            >;
        async fn get_comment(
            &self,
            request: tonic::Request<super::GetCommentRequest>,
        ) -> Result<tonic::Response<super::GetCommentResponse>, tonic::Status>;
        async fn update_comment(
            &self,
            request: tonic::Request<super::UpdateCommentRequest>,
        ) -> Result<tonic::Response<super::UpdateCommentResponse>, tonic::Status>;
        async fn delete_comment(
            &self,
            request: tonic::Request<super::DeleteCommentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    /// The discussion service definition.
    #[derive(Debug)]
    pub struct DiscussionServiceServer<T: DiscussionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DiscussionService> DiscussionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
        /// Enable decompressing requests with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        /// Compress responses with `gzip`, if the client supports it.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DiscussionServiceServer<T>
    where
        T: DiscussionService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/myblog.proto.discussion.DiscussionService/CreateComment" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCommentSvc<T: DiscussionService>(pub Arc<T>);
                    impl<
                        T: DiscussionService,
                    > tonic::server::UnaryService<super::CreateCommentRequest>
                    for CreateCommentSvc<T> {
                        type Response = super::CreateCommentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCommentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_comment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateCommentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myblog.proto.discussion.DiscussionService/ListPublishedComments" => {
                    #[allow(non_camel_case_types)]
                    struct ListPublishedCommentsSvc<T: DiscussionService>(pub Arc<T>);
                    impl<
                        T: DiscussionService,
                    > tonic::server::UnaryService<super::ListPublishedCommentsRequest>
                    for ListPublishedCommentsSvc<T> {
                        type Response = super::ListPublishedCommentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPublishedCommentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_published_comments(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPublishedCommentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myblog.proto.discussion.DiscussionService/GetComment" => {
                    #[allow(non_camel_case_types)]
                    struct GetCommentSvc<T: DiscussionService>(pub Arc<T>);
                    impl<
                        T: DiscussionService,
                    > tonic::server::UnaryService<super::GetCommentRequest>
                    for GetCommentSvc<T> {
                        type Response = super::GetCommentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCommentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_comment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCommentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myblog.proto.discussion.DiscussionService/UpdateComment" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCommentSvc<T: DiscussionService>(pub Arc<T>);
                    impl<
                        T: DiscussionService,
                    > tonic::server::UnaryService<super::UpdateCommentRequest>
                    for UpdateCommentSvc<T> {
                        type Response = super::UpdateCommentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCommentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_comment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateCommentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myblog.proto.discussion.DiscussionService/DeleteComment" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCommentSvc<T: DiscussionService>(pub Arc<T>);
                    impl<
                        T: DiscussionService,
                    > tonic::server::UnaryService<super::DeleteCommentRequest>
                    for DeleteCommentSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCommentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_comment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCommentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: DiscussionService> Clone for DiscussionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: DiscussionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DiscussionService> tonic::transport::NamedService
    for DiscussionServiceServer<T> {
        const NAME: &'static str = "myblog.proto.discussion.DiscussionService";
    }
}
