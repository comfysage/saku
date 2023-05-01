use yuki::cli::start::setup;

fn main() {
    let result = setup();

    match result {
        Ok(_) => {},
        Err(e) => eprintln!("  an error occured:\n{}", e),
    };
}
