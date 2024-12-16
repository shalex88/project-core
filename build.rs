fn main() {
    prost_build::compile_protos(&["proto/device_irs/video.proto"], &["proto/device_irs"]).unwrap();
}