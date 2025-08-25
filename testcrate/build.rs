fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "lua51")]
    let version = lua_src::Lua51;
    #[cfg(feature = "lua51-wasi")]
    let version = lua_src::Lua51Wasi;
    #[cfg(feature = "lua52")]
    let version = lua_src::Lua52;
    #[cfg(feature = "lua53")]
    let version = lua_src::Lua53;
    #[cfg(feature = "lua54")]
    let version = lua_src::Lua54;

    let artifacts = lua_src::Build::new().build(version);
    artifacts.print_cargo_metadata();

    // Example of WASI post-processing (commented out since this is a library build)
    // For applications that produce a final WASM binary, you would uncomment and modify:
    /*
    #[cfg(all(feature = "lua51-wasi", target_arch = "wasm32"))]
    {
        use std::env;
        let target = env::var("CARGO_CFG_TARGET_TRIPLE").unwrap_or_default();
        if target.ends_with("wasip1") {
            // Assume your final binary is in OUT_DIR or target directory
            let out_dir = env::var("OUT_DIR").unwrap();
            let input_wasm = format!("{}/your-app.wasm", out_dir);
            let output_wasm = format!("{}/your-app.exnref.wasm", out_dir);

            if std::path::Path::new(&input_wasm).exists() {
                if let Err(e) = lua_src::wasm_opt_exnref(&input_wasm, &output_wasm) {
                    println!("cargo:warning=Failed to run wasm-opt: {}", e);
                }
            }
        }
    }
    */
}
