use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

/// 日志初始化配置
pub fn initialise_logging() -> () {
    let logging_file_path = "log/output.log";
    let logging_time_format = "{d(%d-%m-%Y %H:%M:%S)}::{l}: {m}{n}";
    let logging_level = LevelFilter::Info;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(logging_time_format)))
        .build();

    let output = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(logging_time_format)))
        .build(logging_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(logging_level)))
                .build("stdout", Box::new(stdout)),
        )
        .appender(Appender::builder().build("output", Box::new(output)))
        .build(
            Root::builder()
                .appender("output")
                .appender("stdout")
                .build(logging_level),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).expect("log4rs has been unable to initialise.");
}