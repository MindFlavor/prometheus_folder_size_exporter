#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
use clap::Arg;
use hyper::Body;
use std::env;
use std::sync::Arc;
mod options;
use options::Options;
mod exporter_error;
mod folder_scanner;
mod render_to_prometheus;
use prometheus_exporter_base::prelude::*;

async fn perform_request(
    _req: http::request::Request<Body>,
    options: Arc<Options>,
) -> Result<String, failure::Error> {
    let results = options.folders_to_scan.scan()?;

    let mut metric = PrometheusMetric::build()
        .with_name("folder_size")
        .with_metric_type(MetricType::Counter)
        .with_help(
            "Size of the folder, including the subfolders is the label recurse_type is \"Sum\"",
        )
        .build();

    for result in results {
        if let Some(user) = result.folder.user {
            metric.render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("path", result.folder.path.as_str())
                    .with_label(
                        "recurse_type",
                        format!("{:?}", result.folder.recurse_type).as_str(),
                    )
                    .with_label("user", user.as_str())
                    .with_value(result.size),
            );
        } else {
            metric.render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("path", result.folder.path.as_str())
                    .with_label(
                        "recurse_type",
                        format!("{:?}", result.folder.recurse_type).as_str(),
                    )
                    .with_value(result.size),
            );
        }
    }

    Ok(metric.render())
}

#[tokio::main]
async fn main() {
    let matches = clap::App::new("prometheus_folder_size_exporter")
        .version("0.2.0")
        .author("Francesco Cogno <francesco.cogno@outlook.com> & Guido Scatena <guido.scatena@unipi.it>")
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

    log::info!("using options: {:?}", options);

    let bind = matches.value_of("port").unwrap();
    let bind = u16::from_str_radix(&bind, 10).expect("port must be a valid number");
    let addr = ([0, 0, 0, 0], bind).into();

    log::info!("starting exporter on {}", addr);

    prometheus_exporter_base::render_prometheus(addr, options, |request, options| {
        Box::pin(perform_request(request, options))
    })
    .await
}
