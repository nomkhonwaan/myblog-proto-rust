fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
        .build_client(false)
        .compile(
            &[
                "myblog-proto/proto/auth/service.proto",
                "myblog-proto/proto/blog/service.proto",
                "myblog-proto/proto/discussion/service.proto"
            ],
            &["myblog-proto"],
        )?;
    Ok(())
}
