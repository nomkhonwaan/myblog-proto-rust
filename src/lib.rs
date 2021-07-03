pub mod myblog {
    pub mod proto {
        pub mod auth {
            include!("myblog.proto.auth.rs");
        }

        pub mod blog {
            include!("myblog.proto.blog.rs");
        }

        pub mod discussion {
            include!("myblog.proto.discussion.rs");
        }

        pub mod storage {
            include!("myblog.proto.storage.rs");
        }
    }
}
