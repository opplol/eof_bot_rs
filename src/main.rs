extern crate daemonize;
use std::fs::File;

use daemonize::Daemonize;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::env;
extern crate getopts;
use getopts::Options;

mod communicator;
mod controller;

#[actix_web::main]
async fn app_server(port: String) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(controller::hello)
            .service(controller::echo)
            .service(controller::eol)
            .route("hey", web::get().to(controller::manual_hello))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
fn main() {
    let stdout = File::create("./log/daemon.out").unwrap();
    let stderr = File::create("./log/daemon.err").unwrap();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "", "set server port", "PORT");
    opts.optflag("d", "", "Run server with daemonize");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&program, opts);
            panic!("{}", f.to_string())
        }
    };

    let port = match matches.opt_str("p") {
        Some(port) => port,
        None => "80".to_string(),
    };
    if matches.opt_present("d") {
        let daemonize = Daemonize::new()
            .pid_file("./pid/server.pid")
            .working_directory("./pid")
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(|| println!("Executed before master process exits"))
            .privileged_action(|| "Executed before drop privileges");

        match daemonize.start() {
            Ok(v) => {
                println!("{:?}", v);
                println!("Success, daemonized");
                app_server(port).unwrap();
            }
            Err(e) => eprintln!("Error, {}", e),
        }
        return;
    }
    app_server(port).unwrap()
}
