use std::panic::PanicHookInfo;

pub fn debug_panic_hook(info: &PanicHookInfo) {
    if let Some(location) = info.location() {
        eprintln!("Panic occurred in {}:{}", location.file(), location.line());
    } else {
        eprintln!("Panic occurred but could not determine the location.");
    }

    if let Some(message) = info.payload().downcast_ref::<&str>() {
        eprintln!("Panic message: {}", message);
    } else if let Some(message) = info.payload().downcast_ref::<String>() {
        eprintln!("Panic message: {}", message);
    } else {
        eprintln!("Panic occurred without a message.");
    }

    eprintln!("Error details: {:#?}", info);
    eprintln!("Press Enter to exit...");

    let _ = std::io::stdin().read_line(&mut String::new());
}
