use oram_smst::grpc::Server as SMTServer;
use oram_smst::smt::smt_backend_server::SmtBackendServer;
use std::error;
use tonic::transport::Server;

mod smt_proto {
    include!("smt.rs");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("smt");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = SMTServer::<3>::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(smt_proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!(message = "Starting server.", %addr);
    Server::builder()
        .trace_fn(|_| tracing::info_span!("SMTBackend server"))
        .add_service(SmtBackendServer::new(server))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
