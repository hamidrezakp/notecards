use std::time::Duration;

use anyhow::anyhow;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys::Promise};

pub async fn sleep(dur: Duration) {
    let window = web_sys::window()
        .ok_or(anyhow!("window not found"))
        .unwrap();

    let closure = Box::new(move |f, _| {
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &f,
                dur.as_millis().clamp(0, i32::MAX as _) as i32,
            )
            .unwrap();
    });
    let mut closure = Box::leak(closure);

    let promise = Promise::new(&mut closure);

    JsFuture::from(promise).await.unwrap();
}

pub fn log(s: String) {
    console::log_1(&s.into());
}

#[macro_export]
macro_rules! jprintln {
    ()              => ($crate::utils::log("".to_owned()));
    ($($arg:tt)*)   => ($crate::utils::log(format!($($arg)*)));
}
