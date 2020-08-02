mod app;
use colored::Colorize;
use structopt::StructOpt;

fn main() {
    let opt = app::Opt::from_args();
    let app = app::App::new(opt);
    app.close_closewait_sockets()
        .unwrap_or_else(|e| panic!("failed to close sockets: {}", e));
}
