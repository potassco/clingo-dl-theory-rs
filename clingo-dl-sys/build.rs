use std::env;
fn main() {
    // checkout clingo-dl
    // git clone https://github.com/potassco/clingo-dl
    // cd clingo-dl
    // git checkout v1.4.0
    // copy clingo.h

    // // Configure and generate bindings.
    // let bindings = bindgen::Builder::default()
    //     .header("clingo-dl/libclingo-dl/clingo-dl.h")
    //     .allowlist_type("clingodl_theory_t")
    //     .allowlist_function("clingodl_create")
    //     .allowlist_function("clingodl_destroy")
    //     .allowlist_function("clingodl_register")
    //     .allowlist_function("clingodl_rewrite_ast")
    //     .allowlist_function("clingodl_prepare")
    //     .allowlist_function("clingodl_register_options")
    //     .allowlist_function("clingodl_validate_options")
    //     .allowlist_function("clingodl_on_model")
    //     .allowlist_function("clingodl_on_statistics")
    //     .allowlist_function("clingodl_lookup_symbol")
    //     .allowlist_function("clingodl_get_symbol")
    //     .allowlist_function("clingodl_assignment_begin")
    //     .allowlist_function("clingodl_assignment_next")
    //     .allowlist_function("clingodl_assignment_has_value")
    //     .allowlist_function("clingodl_assignment_get_value")
    //     .allowlist_function("clingodl_configure")
    //     .size_t_is_usize(true)
    //     .generate()
    //     .unwrap();

    // // Write the generated bindings to an output file.
    // bindings.write_to_file("bindings.rs").unwrap();

    let path =
        env::var("CLINGO_DL_LIBRARY_PATH").expect("$CLINGO_DL_LIBRARY_PATH should be defined");
    println!("cargo:rustc-link-search=native={}", path);

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=import_clingo-dl");
    } else {
        println!("cargo:rustc-link-lib=dylib=clingo-dl");
    }
}
