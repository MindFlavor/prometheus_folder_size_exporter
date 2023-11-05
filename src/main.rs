#[macro_use]
extern crate serde_derive;
use clap::Arg;
use hyper::Body;
use std::env;
use std::sync::Arc;
mod options;
mod state;
use clap::{crate_authors, crate_name, crate_version};
use options::Options;
pub use state::State;
mod exporter_error;
mod folder_scanner;
mod render_to_prometheus;
use prometheus_exporter_base::prelude::*;
use std::error::Error;
use std::time::Duration;

async fn perform_request(
    _req: http::request::Request<Body>,
    state: Arc<State>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let results = if state.options.background_poll_seconds.is_some() {
        // loop until we have some data.
        // This is needed because the first scan can take a lot of time
        // and we must block until we have something. It will happen only
        // at startup though.
        let mut results = state.shared_vec.read().unwrap().clone();
        while results.is_empty() {
            std::thread::sleep(Duration::from_millis(100));
            results = state.shared_vec.read().unwrap().clone();
        }
        results
    } else {
        state.options.folders_to_scan.scan()?
    };

    let mut metric = PrometheusMetric::build()
        .with_name("folder_size_bytes")
        .with_metric_type(MetricType::Gauge)
        .with_help(
            "Size of the folder, including the subfolders if \"explode_depth\" is zero and \"sum_remaining_subfolders\" is true" 
        )
        .build();

    for result in results {
        if let Some(user) = &result.folder.user {
            metric.render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("path", result.folder.path.as_str())
                    .with_label(
                        "explode_depth",
                        format!("{:?}", result.folder.explode_depth).as_str(),
                    )
                    .with_label(
                        "sum_remaining_subfolders",
                        format!("{}", result.folder.sum_remaining_subfolders).as_str(),
                    )
                    .with_label("user", user.as_str())
                    .with_value(result.size),
            );
        } else {
            metric.render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("path", result.folder.path.as_str())
                    .with_label(
                        "explode_depth",
                        format!("{:?}", result.folder.explode_depth).as_str(),
                    )
                    .with_label(
                        "sum_remaining_subfolders",
                        format!("{}", result.folder.sum_remaining_subfolders).as_str(),
                    )
                    .with_value(result.size),
            );
        }
    }

    Ok(metric.render())
}

#[tokio::main]
async fn main() {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
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
        .arg(
            Arg::with_name("background_poll_seconds")
                .short("b")
                .help("enables background scanning every <sec> seconds")
                .required(false)
                .takes_value(true),
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

    let state = State::new(options);

    // start the background thread only
    // if requested
    if let Some(background_poll_seconds) = state.options.background_poll_seconds {
        let state = state.clone();
        std::thread::spawn(move || loop {
            log::info!("starting background folder structure update");
            let results = state.options.folders_to_scan.scan().unwrap();
            *state.shared_vec.write().unwrap() = results;
            log::info!("background folder structure update completed");

            std::thread::sleep(background_poll_seconds);
        });
    }

    let bind = matches.value_of("port").unwrap();
    let bind = bind.parse::<u16>().expect("port must be a valid number");
    let addr = ([0, 0, 0, 0], bind).into();
    log::info!("starting exporter on {}", addr);

    let server_options = ServerOptions {
        addr,
        authorization: Authorization::None,
    };

    prometheus_exporter_base::render_prometheus(server_options, state, |request, state| {
        Box::pin(perform_request(request, state))
    })
    .await
}
