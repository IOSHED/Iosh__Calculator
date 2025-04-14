use std::panic::PanicHookInfo;

pub fn debug_panic_hook(info: &PanicHookInfo) {
    if let Some(location) = info.location() {
        eprintln!("Panic occurred in {}:{}", location.file(), location.line());
    } else {
        eprintln!("Panic occurred but could not determine the location.");
    }

    if let Some(message) = info.payload().downcast_ref::<&str>() {
        eprintln!("\nPanic message: {}.", message);
    } else if let Some(message) = info.payload().downcast_ref::<String>() {
        eprintln!("\nPanic message: {}.", message);
    } else {
        eprintln!("\nPanic occurred without a message.");
    }

    eprintln!("\nError details: {:#?}", info);
    eprintln!("\nPress Enter to exit...");

    let _ = std::io::stdin().read_line(&mut String::new());
}
