use log::{Level, LevelFilter, Log, Metadata, Record};
use owo_colors::OwoColorize;
use owo_colors::Stream::{Stderr, Stdout};
use owo_colors::Style;

pub struct CliLogger;

impl Log for CliLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        match record.level() {
            Level::Error => eprintln!(
                "{} {}",
                "error:".if_supports_color(Stderr, |t| t.style(Style::new().red().bold())),
                record.args()
            ),
            Level::Warn => eprintln!(
                "{} {}",
                "warning:".if_supports_color(Stderr, |t| t.style(Style::new().yellow().bold())),
                record.args()
            ),
            Level::Info => println!("{}", record.args()),
            Level::Debug | Level::Trace => println!(
                "{} {}",
                "debug:".if_supports_color(Stdout, |t| t.style(Style::new().dimmed())),
                record.args()
            ),
        }
    }

    fn flush(&self) {}
}

static LOGGER: CliLogger = CliLogger;

pub fn init(level: LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);
}
