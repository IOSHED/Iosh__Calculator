extern crate lalrpop;
extern crate winres;

fn main() {
    lalrpop::process_root().unwrap();

    let mut res = winres::WindowsResource::new();
    res.set_icon("../resources/logo.ico");
    res.compile().unwrap();
}
