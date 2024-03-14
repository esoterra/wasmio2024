use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};

use clap::Parser;

use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut component_file = std::fs::File::open(args.input).unwrap();
    let mut component_bytes = Vec::new();
    component_file.read_to_end(&mut component_bytes).unwrap();
    
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();

    let component = Component::new(&engine, &component_bytes).unwrap();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    bindgen!("plugin" in "wit");

    impl PluginImports for () {
        fn foo(&mut self, a: String) -> wasmtime::Result<String> {
            match a.as_str() {
                "slow-input" => {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                },
                _ => {}
            }
            wasmtime::Result::Ok(a)
        }
    }

    use wasi::logging::logging;
    impl logging::Host for () {
        fn log(
            &mut self,
            level: logging::Level,
            context: String,
            message: String,
        ) -> wasmtime::Result<()> {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let level = match level {
                logging::Level::Trace => "TRACE",
                logging::Level::Debug => "DEBUG",
                logging::Level::Info => " INFO",
                logging::Level::Warn => " WARN",
                logging::Level::Error => "ERROR",
                logging::Level::Critical => " CRIT",
            };
            println!("{:?} {} [{}]: {}", timestamp, level, context, message);
            wasmtime::Result::Ok(())
        }
    }

    use wasi::clocks::monotonic_clock;
    impl monotonic_clock::Host for () {
        fn now(&mut self) -> wasmtime::Result<monotonic_clock::Instant> {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            wasmtime::Result::Ok(monotonic_clock::Instant::from(timestamp as u64))
        }
    }

    Plugin::add_to_linker(&mut linker, |s| s).unwrap();

    let (plugin, _) = Plugin::instantiate(&mut store, &component, &linker).unwrap();

    plugin.call_run(&mut store).unwrap();
}
