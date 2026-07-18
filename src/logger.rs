use chrono::Local;
use std::io::Write;

pub fn init_logger() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let ts = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            let style = buf.default_level_style(record.level());
            let lvl = record.level();
            let target = record.target();
            write!(buf, "[{ts} ")?;
            write!(buf, "{style}{lvl:<5}{style:#}")?;
            write!(buf, " {target}] {}", record.args())?;
            writeln!(buf)
        })
        .try_init()
        .ok();
}

#[cfg(test)]
mod tests {
    use log::{info, LevelFilter};

    #[test]
    fn test_logger_init() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Off)
            .is_test(true)
            .try_init();
        info!("test log message");
    }
}
