extern crate lalrpop;
extern crate winres;

const PATH_TO_ICON: &'static str = "../../resources/logo.ico";

fn main() {
    lalrpop::process_root().unwrap();

    let mut res = winres::WindowsResource::new();
    res.set_icon(PATH_TO_ICON);
    res.compile().unwrap();
}
