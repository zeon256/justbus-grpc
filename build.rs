fn main() {
    tonic_build::compile_protos("proto/buses.proto").unwrap()
}
