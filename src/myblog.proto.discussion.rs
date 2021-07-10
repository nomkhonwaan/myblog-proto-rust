/// A written remark expressing an opinion or reaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// Identifier of the comment
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Status of the comment which could be...
    /// - Published
    /// - Deleted
    #[prost(enumeration = "CommentStatus", tag = "2")]
    pub status: i32,
    /// Content of the comment in plain text format
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// A commentator
    #[prost(message, optional, tag = "4")]
    pub author: ::core::option::Option<super::auth::User>,
    /// A parent of the comment, this could be null
    #[prost(message, optional, boxed, tag = "5")]
    pub parent: ::core::option::Option<::prost::alloc::boxed::Box<Comment>>,
    /// List of comments were replied to this comment
    #[prost(message, repeated, tag = "6")]
    pub children: ::prost::alloc::vec::Vec<Comment>,
    /// Date-time that the comment was created
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Date-time that the comment was updated
    #[prost(message, optional, tag = "8")]
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
    #[prost(message, optional, tag = "1")]
    pub comment: ::core::option::Option<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommentResponse {
    #[prost(message, optional, tag = "1")]
    pub comment: ::core::option::Option<Comment>,
}
#[doc = r" Generated server implementations."]
pub mod discussion_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DiscussionServiceServer."]
    #[async_trait]
    pub trait DiscussionService: Send + Sync + 'static {
        async fn create_comment(
            &self,
            request: tonic::Request<super::CreateCommentRequest>,
        ) -> Result<tonic::Response<super::CreateCommentResponse>, tonic::Status>;
    }
    #[doc = " The discussion service definition."]
    #[derive(Debug)]
    pub struct DiscussionServiceServer<T: DiscussionService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DiscussionService> DiscussionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for DiscussionServiceServer<T>
    where
        T: DiscussionService,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/myblog.proto.discussion.DiscussionService/CreateComment" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCommentSvc<T: DiscussionService>(pub Arc<T>);
                    impl<T: DiscussionService>
                        tonic::server::UnaryService<super::CreateCommentRequest>
                        for CreateCommentSvc<T>
                    {
                        type Response = super::CreateCommentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCommentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_comment(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: DiscussionService> tonic::transport::NamedService for DiscussionServiceServer<T> {
        const NAME: &'static str = "myblog.proto.discussion.DiscussionService";
    }
}
