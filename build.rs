
fn main(){
    let includes = ["protos"];
    let inputs = ["protos/foo.proto", "protos/annotations.proto"];

    protox::compile(inputs, includes).unwrap();

    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .pure()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(includes)
        // Inputs must reside in some of include paths.
        .inputs(inputs)
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();


}