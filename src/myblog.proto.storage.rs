/// Object that keeps in the storage server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// Identifier of the file
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// An original file name
    #[prost(string, tag="2")]
    pub file_name: ::prost::alloc::string::String,
    /// Valid url string composes with file name and id
    #[prost(string, tag="3")]
    pub slug: ::prost::alloc::string::String,
    /// An uploaded file path
    #[prost(string, tag="4")]
    pub uploaded_file_path: ::prost::alloc::string::String,
    /// A media type
    #[prost(string, tag="5")]
    pub mime_type: ::prost::alloc::string::String,
    /// Name of the cloud object storage provider (e.g. s3, gcp, azure, etc.)
    #[prost(string, tag="6")]
    pub provider: ::prost::alloc::string::String,
    /// Region or location where the object has been uploaded
    #[prost(string, tag="7")]
    pub region: ::prost::alloc::string::String,
    /// Name of the bucket
    #[prost(string, tag="8")]
    pub bucket: ::prost::alloc::string::String,
    /// Date-time that the file was uploaded
    #[prost(message, optional, tag="9")]
    pub uploaded_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Date-time that the file was modified
    #[prost(message, optional, tag="10")]
    pub modified_at: ::core::option::Option<::prost_types::Timestamp>,
}
