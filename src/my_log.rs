use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;

const WINDOW_SIZE: u32 = 3;
const SIZE_LIMIT: u64 = 1024 * 1024 * 3;

pub fn init_log() -> Result<(), String> {
    let path_dir = crate::path::get_path_directory_config()?;

    if !path_dir.exists() {
        std::fs::create_dir_all(&path_dir).map_err(|e| {
            format!(
                "failed to create config directory: path={}, err={}",
                path_dir.display(),
                e
            )
        })?;
    }
    let file_pattern = crate::path::get_str_file_pattern_error_log()?;
    let file_log = crate::path::get_str_file_error_log()?;

    let fixed_window_roller = FixedWindowRoller::builder()
        .build(&file_pattern, WINDOW_SIZE)
        .map_err(|e| format!("failed to define FixedWindowRoller: {e}"))?;

    let size_trigger = SizeTrigger::new(SIZE_LIMIT);

    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y/%m/%d %H:%M:%S%.f%:z)}][{l}] {m}\n",
        )))
        .build(&file_log, Box::new(compound_policy))
        .map_err(|e| format!("failed to build RollingFileAppender: {e}"))?;

    let config = Config::builder()
        .appender(Appender::builder().build(&file_log, Box::new(appender)))
        .build(
            Root::builder()
                .appender(&file_log)
                .build(LevelFilter::Debug),
        )
        .map_err(|e| format!("failed to build Config: {e}"))?;

    init_config(config).map_err(|e| format!("failed to init log config: {e}"))?;

    Ok(())
}
