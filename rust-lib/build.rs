use cmake;

fn main() {
    let dst = cmake::Config::new("../")
        .generator("Ninja Multi-Config")
        .build_target("libminimal.a")
        .profile("debug")
        .build();
    
    println!("cargo:rustc-link-search=native={}/build/Debug", dst.display());
    println!("cargo:rustc-link-lib=static=minimal");
}
