use druid::{AppLauncher, Data};
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::{fmt, prelude::*, registry::Registry, EnvFilter};

pub fn setup_global_subscriber() -> impl Drop {
    let fmt_layer = fmt::Layer::default();
    /*let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("errorxfg"))
        .unwrap();*/

    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();

    Registry::default()
        //.with(filter_layer)
        .with(fmt_layer)
        .with(chrome_layer).init();

    //druid::trace::set_default_subscriber(subscriber);
    _guard
}
