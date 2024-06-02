//! Initializing a multithreaded logger in Rust.(Derive)
//!
//! This library provides functionality to initialize a multithreaded logger with flexible configuration options.
//! It supports both file I/O logging and logging to stdout with ANSI color support.
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A builder for initializing a multi-threaded logger.
///
/// Allows configuring the logger with optional settings for
/// test name, standard I/O output, and log level filter.
#[derive(deluxe::ParseMetaItem)]
struct LoggerBuilder {
    file: Option<String>,
    test: Option<String>,
    #[deluxe(default = true)]
    stdio: bool,
    #[deluxe(default = "TRACE".to_string())]
    level: String,
}

pub(crate) fn parse_level(level: &str) -> TokenStream {
    let level = level.to_ascii_lowercase();
    let level = match level.as_str() {
        "trace" => quote! { tracing::metadata::LevelFilter::TRACE },
        "debug" => quote! { tracing::metadata::LevelFilter::DEBUG },
        "info" => quote! {  tracing::metadata::LevelFilter::INFO },
        "warn" => quote! {  tracing::metadata::LevelFilter::WARN },
        "error" => quote! { tracing::metadata::LevelFilter::ERROR },
        unknown => {
            panic!("The log level must be one of TRACE|DEBUG|INFO|WARN|ERROR. But got {unknown}")
        }
    };
    quote! { .filter(#level) }
}

/// An attribute to initialize a logger with various options.
///
/// # Attributes
///
/// - `test`: Sets the test name for the logger. Log output will be written to a file named `../logs/{test_name}.log`.
/// - `file`: Sets the file path for the log output.
/// - `stdio`: Enables standard I/O output for the logger.(default: true)
/// - `level`: Sets the log level filter (e.g., `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`. default: `TRACE`).
///
/// # Examples
///
/// - Trace mode + stdio
///
/// ```ignore
/// #[quick_tracing::init]
/// fn main() {
///     tracing::debug!("Hello, world!");
/// }
/// ```
///
/// - Debug mode + Output file(If there is no parent directory, it will automatically create one.)
///
/// ```ignore
/// #[quick_tracing::init(level= "DEBUG", file = "./log/test.log", stdio = false)]
/// fn main() {
///     tracing::debug!("Hello, world!");
/// }
/// ```
#[proc_macro_attribute]
pub fn init(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let LoggerBuilder {
        file,
        test,
        stdio,
        level,
    } = match deluxe::parse::<LoggerBuilder>(attr) {
        Ok(desc) => desc,
        Err(e) => return e.into_compile_error().into(),
    };

    if file.is_some() && test.is_some() {
        panic!("Use either one of `test` or `file`.")
    };

    let test = test.map_or(quote! {}, |name| {
        quote! {
            .test_name(#name).expect("Failed to create log directory.")
        }
    });
    let file = file.map_or(quote! {}, |path| {
        quote! {
            .file({
                let _ = std::path::Path::new(#path).parent().map(|dir| std::fs::create_dir_all(dir));
                #path
            })
        }
    });
    let stdio = match stdio {
        true => quote! { .stdio() },
        false => quote!(),
    };

    let level_filter = parse_level(&level);

    let logger_init = quote! {
        let (worker_guard, default_guard) = quick_tracing::builder::LoggerBuilder::new()
            #stdio
            #test
            #file
            #level_filter
            .build()
            .expect("Failed to initialize logger");
    };

    let mut item_fn = parse_macro_input!(item as ItemFn);
    item_fn
        .block
        .stmts
        .insert(0, syn::parse2(logger_init).unwrap());

    quote! { #item_fn }.into()
}

/// An attribute to initialize a logger with various options.
///
/// # Attributes
///
/// - `test`: Sets the test name for the logger. Log output will be written to a file named `../logs/{test_name}.log`.
/// - `file`: Sets the file path for the log output.
/// - `stdio`: Enables standard I/O output for the logger.(default: true)
/// - `level`: Sets the log level filter (e.g., `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`. default: `TRACE`).
///
/// # Examples
///
/// - Trace mode + stdio
///
/// ```ignore
/// #[quick_tracing::try_init]
/// fn main() -> std::io::Result<()> {
///     tracing::debug!("Hello, world!");
///     Ok(())
/// }
/// ```
///
/// - Debug mode + Output file(If there is no parent directory, it will automatically create one.)
///
/// ```ignore
/// #[quick_tracing::try_init(level= "DEBUG", file = "./log/test.log", stdio = false)]
/// fn main() -> std::io::Result<()> {
///     tracing::debug!("Hello, world!");
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn try_init(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let LoggerBuilder {
        file,
        test,
        stdio,
        level,
    } = match deluxe::parse::<LoggerBuilder>(attr) {
        Ok(desc) => desc,
        Err(e) => return e.into_compile_error().into(),
    };

    if file.is_some() && test.is_some() {
        panic!("Use either one of `test` or `file`.")
    };

    let test = test.map_or(quote! {}, |name| {
        quote! {
            .test_name(#name)?
        }
    });
    let file = file.map_or(quote! {}, |path| {
        quote! {
            .file({
                if let Some(dir) = std::path::Path::new(#path).parent() {
                    std::fs::create_dir_all(dir)?;
                };
                #path
            })
        }
    });
    let stdio = match stdio {
        true => quote! { .stdio() },
        false => quote!(),
    };

    let level_filter = parse_level(&level);

    let logger_init = quote! {
        let (worker_guard, default_guard) = quick_tracing::builder::LoggerBuilder::new()
            #stdio
            #test
            #file
            #level_filter
            .build()?;
    };

    let mut item_fn = parse_macro_input!(item as ItemFn);
    item_fn
        .block
        .stmts
        .insert(0, syn::parse2(logger_init).unwrap());

    quote! { #item_fn }.into()
}
