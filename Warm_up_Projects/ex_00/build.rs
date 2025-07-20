fn main() {
    slint_build::compile("slint_ui/main.slint").unwrap();
}

/*
* generates Rust bindings for  MainWindow and other components.
* Then in main.rs, this is loaded with: 'slint::include_modules!();'
* That macro loads what slint_build compiled.
*
* cargo run:
*  1.Runs build.rs
*      Calls slint_build::compile(...)
*      Compiles .slint into Rust code (in OUT_DIR)
*
*  2.Compiles the actual Rust app (main.rs)
*      Which uses slint::include_modules!() to include the generated code
*
* ===
*      slint: runtime GUI for the app → needed in main.rs
       slint-build: compiles .slint files into Rust → needed in build.rs
*
*/
