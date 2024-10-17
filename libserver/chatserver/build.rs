

fn main() {
    println!("cargo:rerun-if-changed=src/network/protos");
    protobuf_codegen::Codegen::new()
        .out_dir("src/network/stubs")
        .include("src/network/protos")
        .inputs([
            "src/network/protos/chatmsg.proto",
            "src/network/protos/user.proto"
        ])
        .run_from_script();

}