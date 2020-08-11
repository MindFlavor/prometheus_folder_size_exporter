#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
use clap;
use clap::Arg;
use futures::future::{done, ok, Future};
use http::StatusCode;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use log::{debug, error, info, trace, warn};
use std::env;
mod options;
use options::Options;
mod exporter_error;
use exporter_error::ExporterError;
mod render_to_prometheus;
use render_to_prometheus::RenderToPrometheus;
mod folder_scanner;
use folder_scanner::FolderToScan;

fn handle_request(
    req: Request<Body>,
    options: &Options,
) -> impl Future<Item = Response<Body>, Error = failure::Error> {
    trace!("{:?}", req);

    perform_request(req, options).then(|res| match res {
        Ok(body) => ok(body),
        Err(inner_error) => match inner_error {
            ExporterError::UnsupportedPathError { path: ref _path } => {
                warn!("{:?}", inner_error);
                let r = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(hyper::Body::empty())
                    .unwrap();
                ok(r)
            }
            ExporterError::UnsupportedMethodError { verb: ref _verb } => {
                warn!("{:?}", inner_error);
                let r = Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(hyper::Body::empty())
                    .unwrap();
                ok(r)
            }
            _ => {
                error!("{:?}", inner_error);
                let r = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(hyper::Body::empty())
                    .unwrap();
                ok(r)
            }
        },
    })
}

fn perform_request(
    _req: Request<Body>,
    options: &Options,
) -> impl Future<Item = Response<Body>, Error = ExporterError> {
    trace!("perform_request");
    done(options.folders_to_scan.scan())
        .from_err()
        .and_then(|v_sizes| {
            let mut s = String::with_capacity(1024);
            for result in v_sizes {
                s.push_str(&format!(
                    "folder_size{{path=\"{}\",recursive=\"{}\"}, user=\"{}\"}} {}\n",
                    result.folder.path,  result.folder.recursive, result.folder.user, result.size
                ));
            }

            ok(Response::new(Body::from(s)))
        })
}

fn main() {
    let matches = clap::App::new("prometheus_folder_size_exporter")
        .version("0.1.1")
        .author("Francesco Cogno <francesco.cogno@outlook.com>  & Guido Scatena <guido.scatena@unipi.it>")
        .arg(
            Arg::with_name("port")
                .short("p")
                .help("exporter port (default 9974)")
                .default_value("9974")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("folders_file")
                .short("i")
                .help("folders to scan json file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("verbose logging")
                .takes_value(false),
        )
        .get_matches();

    let options = Options::from_claps(&matches);

    if options.verbose {
        env::set_var("RUST_LOG", "prometheus_folder_size_exporter=trace");
    } else {
        env::set_var("RUST_LOG", "prometheus_folder_size_exporter=info");
    }
    env_logger::init();

    info!("using options: {:?}", options);

    let bind = matches.value_of("port").unwrap();
    let bind = u16::from_str_radix(&bind, 10).expect("port must be a valid number");
    let addr = ([0, 0, 0, 0], bind).into();

    info!("starting exporter on {}", addr);

    let new_svc = move || {
        let options = options.clone();
        service_fn(move |req| handle_request(req, &options))
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));
    hyper::rt::run(server);
}
