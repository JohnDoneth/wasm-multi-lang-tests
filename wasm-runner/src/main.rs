extern crate walkdir;
extern crate wasmi;

use wasmi::{ImportsBuilder, Module, ModuleInstance, ModuleRef, NopExternals, RuntimeValue};

use walkdir::WalkDir;

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;

mod resolvers;
use resolvers::EnvModuleResolver;

fn load_from_file<P: AsRef<Path>>(filename: P) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Module::from_buffer(buf).unwrap()
}

// Sometimes the module functions are exported with a leading underscore so this method tries to get the correct one
fn get_function_name(name: &'static str, module: &ModuleRef) -> Option<String> {
    if module.export_by_name(name).is_some() {
        return Some(name.to_string());
    } else {
        let underscore = "_".to_string() + name;

        if module.export_by_name(&underscore).is_some() {
            return Some(underscore);
        }
    }

    return None;
}

fn factorial_runner(module: &ModuleRef) {
    // The argument should be parsable as a valid integer
    let argument: i32 = 7;

    let function_name = get_function_name("factorial", module).unwrap();

    let result = module.invoke_export(
        &function_name,
        &[RuntimeValue::I32(argument)],
        &mut NopExternals,
    );

    println!("\tResult: {:?}", result);
}

fn add_runner(module: &ModuleRef) {
    // The argument should be parsable as a valid integer
    let argument: i32 = 2;

    let function_name = get_function_name("add", module).unwrap();

    let result = module.invoke_export(
        &function_name,
        &[RuntimeValue::I32(argument), RuntimeValue::I32(argument)],
        &mut NopExternals,
    );

    println!("\tResult: {:?}", result);
}

fn main() {
    let mut runners: HashMap<&'static str, fn(&ModuleRef)> = HashMap::new();

    runners.insert("add", add_runner);
    runners.insert("factorial", factorial_runner);

    for entry in WalkDir::new("../generated") {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("wasm")) {
                
                let file_name = entry.path().file_stem().unwrap().to_str().unwrap();

                if let Some(runner) = runners.get(file_name) {

                    println!("Running: {:?}: ", entry.path());

                    let module = load_from_file(entry.path());

                    let imports = ImportsBuilder::new().with_resolver("env", &EnvModuleResolver);

                    match ModuleInstance::new(&module, &imports) {
                        Ok(module) => {
                            let module = module
                                .run_start(&mut NopExternals)
                                .expect("Failed to run start function in module");

                            runner(&module);
                        }
                        Err(e) => {
                            println!("\tError while instantiating module: {:?}", e);
                        }
                    }
                } else {
                    println!("\tNo runner found for {:?}", file_name);
                }

                println!();

            }
        }
    }
}
