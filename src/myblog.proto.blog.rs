/// The social network engagement information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Engagement {
    #[prost(uint32, tag = "1")]
    pub share_count: u32,
}
/// Label attached to the post for the purpose of identification or classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Taxonomy {
    /// Identifier of the taxonomy
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Name of the taxonomy
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Valid url string composes with name and id
    #[prost(string, tag = "3")]
    pub slug: ::prost::alloc::string::String,
    /// Type of the term e.g. category or tag
    #[prost(enumeration = "TaxonomyType", tag = "4")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaxonomyType {
    Category = 0,
    Tag = 1,
}
/// Piece of content in the blog platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
    /// Identifier of the post
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Title of the post
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Valid url string composes with title and id
    #[prost(string, tag = "3")]
    pub slug: ::prost::alloc::string::String,
    /// Status of the post which could be...
    /// - Draft
    /// - Published
    #[prost(enumeration = "PostStatus", tag = "4")]
    pub status: i32,
    /// Original content of the post in markdown syntax
    #[prost(string, tag = "5")]
    pub markdown: ::prost::alloc::string::String,
    /// Content of the post in HTML format which will be translated from markdown
    #[prost(string, tag = "6")]
    pub html: ::prost::alloc::string::String,
    /// Date-time that the post was published
    #[prost(message, optional, tag = "7")]
    pub published_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Creator of the post
    #[prost(message, optional, tag = "8")]
    pub author: ::core::option::Option<super::auth::User>,
    /// List of categories that the post belonging to
    #[prost(message, repeated, tag = "9")]
    pub categories: ::prost::alloc::vec::Vec<Taxonomy>,
    /// List of tags that the post belonging to
    #[prost(message, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<Taxonomy>,
    /// A featured image to be shown at the archive page as a cover image
    #[prost(message, optional, tag = "11")]
    pub featured_image: ::core::option::Option<super::storage::File>,
    /// Date-time that the post was created
    #[prost(message, optional, tag = "12")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Date-time that the post was updated
    #[prost(message, optional, tag = "13")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostStatus {
    Draft = 0,
    Published = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublishedPostsRequest {
    #[prost(uint32, tag = "1")]
    pub offset: u32,
    #[prost(uint32, tag = "2")]
    pub limit: u32,
    #[prost(message, optional, tag = "3")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublishedPostsResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<Post>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub categories: ::prost::alloc::vec::Vec<Taxonomy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomyPublishedPostsRequest {
    #[prost(message, optional, tag = "1")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
    #[prost(uint32, tag = "2")]
    pub offset: u32,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(message, optional, tag = "4")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomyPublishedPostsResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<Post>,
}
#[doc = r" Generated server implementations."]
pub mod blog_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BlogServiceServer."]
    #[async_trait]
    pub trait BlogService: Send + Sync + 'static {
        async fn list_categories(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::ListCategoriesResponse>, tonic::Status>;
        async fn list_published_posts(
            &self,
            request: tonic::Request<super::ListPublishedPostsRequest>,
        ) -> Result<tonic::Response<super::ListPublishedPostsResponse>, tonic::Status>;
        async fn list_taxonomy_published_posts(
            &self,
            request: tonic::Request<super::ListTaxonomyPublishedPostsRequest>,
        ) -> Result<tonic::Response<super::ListTaxonomyPublishedPostsResponse>, tonic::Status>;
    }
    #[doc = " The blog service definition."]
    #[derive(Debug)]
    pub struct BlogServiceServer<T: BlogService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BlogService> BlogServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for BlogServiceServer<T>
    where
        T: BlogService,
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
                "/myblog.proto.blog.BlogService/ListCategories" => {
                    #[allow(non_camel_case_types)]
                    struct ListCategoriesSvc<T: BlogService>(pub Arc<T>);
                    impl<T: BlogService> tonic::server::UnaryService<()> for ListCategoriesSvc<T> {
                        type Response = super::ListCategoriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_categories(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListCategoriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myblog.proto.blog.BlogService/ListPublishedPosts" => {
                    #[allow(non_camel_case_types)]
                    struct ListPublishedPostsSvc<T: BlogService>(pub Arc<T>);
                    impl<T: BlogService>
                        tonic::server::UnaryService<super::ListPublishedPostsRequest>
                        for ListPublishedPostsSvc<T>
                    {
                        type Response = super::ListPublishedPostsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPublishedPostsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_published_posts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPublishedPostsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myblog.proto.blog.BlogService/ListTaxonomyPublishedPosts" => {
                    #[allow(non_camel_case_types)]
                    struct ListTaxonomyPublishedPostsSvc<T: BlogService>(pub Arc<T>);
                    impl<T: BlogService>
                        tonic::server::UnaryService<super::ListTaxonomyPublishedPostsRequest>
                        for ListTaxonomyPublishedPostsSvc<T>
                    {
                        type Response = super::ListTaxonomyPublishedPostsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTaxonomyPublishedPostsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_taxonomy_published_posts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTaxonomyPublishedPostsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
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
    impl<T: BlogService> Clone for BlogServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BlogService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BlogService> tonic::transport::NamedService for BlogServiceServer<T> {
        const NAME: &'static str = "myblog.proto.blog.BlogService";
    }
}
