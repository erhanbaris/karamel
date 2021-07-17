use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
extern crate karamellib;
use clap::{Arg, App};


use karamellib::{constants::{KARAMEL_CONTACT_EMAIL, KARAMEL_HELP_ABOUT, KARAMEL_TITLE, KARAMEL_VERSION}, vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {
    let matches = App::new(KARAMEL_TITLE)
                          .version(KARAMEL_VERSION)
                          .author(KARAMEL_CONTACT_EMAIL)
                          .about(KARAMEL_HELP_ABOUT)
                          .arg(Arg::with_name("file")
                               .short("d")
                               .long("dosya")
                               .value_name("FILE")
                               .help("Çalıştırılacak karamel dosyası")
                               .takes_value(true))
                          .get_matches();

    let parameters = match matches.value_of("file") {
        Some(file) => ExecutionParameters {
            source: ExecutionSource::File(file.to_string()),
            return_opcode: true,
            return_output: true
        },
        None => ExecutionParameters {
            source: ExecutionSource::Code(r#"
gç::satıryaz(123)
        
"#.to_string()),
            return_opcode: true,
            return_output: true
        }
    };

    
    let result = karamellib::vm::executer::code_executer(parameters);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

