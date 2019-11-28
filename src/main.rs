fn main() {
    println!(
        "{:?}",
        spirv_cross::ErrorCode::CompilationError("Hello, World!".to_string())
    );
}
