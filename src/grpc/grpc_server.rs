// 基于生成grpc代码 生成grpc服务
use crate::grpc::signature_service_server::SignatureService;
struct MyHelloAxumRpc {}

#[tonic::async_trait]
impl SignatureService for MyHelloAxumRpc {}
