fn main() {
    tonic_build::compile_protos("proto/health_check.proto").unwrap();
}
