
fn main() {
    let jvm_loc = "C:/Program Files/Java/jdk1.8.0_241/";

    println!("cargo:rustc-link-lib=jvm");
    println!("cargo:rustc-link-search={}lib/", jvm_loc);
    println!("cargo:rustc-link-search={}jre/bin/server/", jvm_loc);
}
