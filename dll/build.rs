fn main() {
    cc::Build::new().file("c_src/c_entry.c").compile("c_code");
}
