use colored::Colorize;
use ctrlc;
use pyo3::prelude::*;

mod bridge;
mod components;
mod core;
mod logging;
mod macros;

use crate::core as zenx_core;

#[pyfunction]
#[pyo3(signature = (handler, *, host = None, port = None))]
fn go(
    _py: Python<'_>,
    handler: Py<PyAny>,
    host: Option<String>,
    port: Option<String>,
) -> PyResult<()> {
    ctrlc::set_handler(|| {
        println!("^C");
        std::process::exit(2);
    })
    .ok();

    let f = async move {
        println!("\n{} v0\n", "zenx".bold());
        println!(
            "   {} Local: \x1b]8;;http://localhost:{}\x1b\\{}\x1b]8;;\x1b\\\n",
            "●".green(),
            port.to_owned().unwrap_or(s!("8787")),
            format!("http://localhost:{}", port.to_owned().unwrap_or(s!("8787"))).cyan()
        );

        zenx_core::create_app(
            host.unwrap_or(s!("127.0.0.1")),
            port.unwrap_or(s!("8787")),
            handler,
        )
        .await;
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f);

    Ok(())
}

#[pymodule]
fn zenx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(go, m)?)?;
    Ok(())
}
