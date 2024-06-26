use std::{fs, io, path::Path};
use tracing::{
    level_filters::LevelFilter,
    subscriber::{set_default, DefaultGuard},
};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt};

/// A builder for initializing a multi-threaded logger.
///
/// Allows configuring the logger with optional settings for
/// test name, standard I/O output, and log level filter.
pub struct LoggerBuilder {
    output_file: Option<String>,
    with_stdio: bool,
    filter: LevelFilter,
}

impl Default for LoggerBuilder {
    /// std I/O Only
    /// - wtiH_stdio: true
    /// - Level: TRACE
    /// - output file: None
    fn default() -> Self {
        Self {
            output_file: None,
            with_stdio: true,
            filter: LevelFilter::TRACE,
        }
    }
}

impl LoggerBuilder {
    /// Creates a new `LoggerBuilder`(No output at this point.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder =  quick_tracing::builder::LoggerBuilder::new();
    /// ```
    pub const fn new() -> Self {
        LoggerBuilder {
            output_file: None,
            with_stdio: false,
            filter: LevelFilter::TRACE,
        }
    }

    /// Sets the test name for the logger.
    ///
    /// If a test name is provided, log output will be written to a file
    /// named `../logs/{test_name}.log`.
    ///
    /// # Examples
    /// ```
    /// use quick_tracing::builder::LoggerBuilder;
    /// let builder = LoggerBuilder::new().test_name("my_test");
    /// ```
    ///
    /// # Errors
    /// Failed to create a directory: `"../logs"`
    pub fn test_name(mut self, test_name: &str) -> io::Result<Self> {
        fs::create_dir_all("../logs")?;
        self.output_file = Some(format!("../logs/{test_name}.log"));
        Ok(self)
    }

    /// Write log to arbitrary file.
    ///
    /// # Examples
    /// ```
    /// std::fs::create_dir_all("../logs").expect("Failed to create directory");
    /// let builder =  quick_tracing::builder::LoggerBuilder::new().file("../logs/test.log");
    /// ```
    pub fn file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output_file = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Enables or disables standard I/O output for the logger.
    ///
    /// When enabled, log output will also be written to stdout with ANSI colors.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = quick_tracing::builder::LoggerBuilder::new().stdio();
    /// ```
    pub fn stdio(mut self) -> Self {
        self.with_stdio = true;
        self
    }

    /// Sets the log level filter for the logger.
    ///
    /// The log level filter determines the maximum log level that will be output.
    ///
    /// # Arguments
    ///
    /// * `filter` - A `LevelFilter` representing the maximum log level.
    ///
    /// # Examples
    ///
    /// ```
    /// use tracing::metadata::LevelFilter;
    /// use quick_tracing::builder::LoggerBuilder;
    /// let builder = LoggerBuilder::new().filter(LevelFilter::DEBUG);
    /// ```
    pub fn filter(mut self, filter: LevelFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Builds and initializes the logger with the specified settings.
    ///
    /// Returns a tuple containing an optional `WorkerGuard` and a `DefaultGuard`.
    /// Dropping these guards will stop the logger.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a failure in creating the log directory or file.
    ///
    /// # Examples
    ///
    /// ```
    /// use tracing::metadata::LevelFilter;
    /// use quick_tracing::builder::LoggerBuilder;
    ///
    /// let (worker_guard, default_guard) = LoggerBuilder::new()
    ///     .test_name("my_test")
    ///     .expect("Failed to create log directory")
    ///     .stdio()
    ///     .filter(LevelFilter::DEBUG)
    ///     .build().expect("Failed to build builder");
    /// ```
    pub fn build(self) -> io::Result<(Option<WorkerGuard>, DefaultGuard)> {
        let (worker_guard, default_guard) = match self.output_file {
            Some(output_file) => {
                let (file_writer, guard) =
                    tracing_appender::non_blocking(fs::File::create(output_file)?);

                let thread_guard = match self.with_stdio {
                    true => set_default(
                        fmt::Subscriber::builder()
                            .compact()
                            .pretty()
                            .with_file(true)
                            .with_line_number(true)
                            .with_max_level(self.filter)
                            .with_target(false)
                            .finish()
                            .with(
                                fmt::Layer::default()
                                    .compact()
                                    .with_ansi(false)
                                    .with_file(true)
                                    .with_line_number(true)
                                    .with_target(false)
                                    .with_writer(file_writer),
                            ),
                    ),
                    false => set_default(
                        fmt::Subscriber::builder()
                            .compact()
                            .with_ansi(false)
                            .with_file(true)
                            .with_line_number(true)
                            .with_target(false)
                            .with_writer(file_writer)
                            .with_max_level(self.filter)
                            .finish(),
                    ),
                };

                (Some(guard), thread_guard)
            }
            None => (
                None,
                set_default(
                    fmt::Subscriber::builder()
                        .compact()
                        .pretty()
                        .with_file(true)
                        .with_line_number(true)
                        .with_max_level(self.filter)
                        .with_target(false)
                        .finish(),
                ),
            ),
        };
        Ok((worker_guard, default_guard))
    }
}
