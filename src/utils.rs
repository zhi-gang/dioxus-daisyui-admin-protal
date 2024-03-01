#[cfg(feature = "web")]
use web_sys::wasm_bindgen::{JsCast, JsValue};
#[cfg(feature = "web")]
use web_sys::{Storage, Window};
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::prelude::*;
use log::error;

#[cfg(feature = "web")]
pub fn alert(msg: &str) {
    if let Some(window) = window() {
        if let Some(window) = window.dyn_into::<Window>().ok() {
            if let Err(err) = window.alert_with_message(msg) {
                error!("Failed to invoke alert: {:?}", err);
            }
        } else {
            error!("Failed to cast window to Window object");
        }
    } else {
        error!("Failed to cast window to Window object")
    }
}
#[cfg(feature = "web")]
fn window() -> Option<Window> {
    web_sys::window()
}
// Function to get the session storage
#[cfg(feature = "web")]
fn session_storage() -> Option<Storage> {
    window()?.session_storage().ok()?
}

// Function to set a key-value pair in session storage
#[cfg(feature = "web")]
pub fn set_item(key: &str, value: &str) -> Result<(), JsValue> {
    if let Some(storage) = session_storage() {
        storage.set_item(key, value)?;
        Ok(())
    } else {
        Err(JsValue::from_str("Session storage not supported"))
    }
}

// Function to get a value from session storage by key
#[cfg(feature = "web")]
pub fn get_item(key: &str) -> Option<String> {
    session_storage().and_then(|storage| match storage.get_item(key) {
        Ok(item) => item,
        Err(_) => None,
    })
}

// Function to remove an item from session storage by key
#[cfg(feature = "web")]
pub fn remove_item(key: &str) -> Result<(), JsValue> {
    if let Some(storage) = session_storage() {
        storage.remove_item(key)?;
        Ok(())
    } else{ 
        Err(JsValue::from_str("Session storage not supported"))
    }
}
#[cfg(feature = "web")]
pub fn get_base_url() -> Option<String> {
    // Get the window object
    let window = window()?;

    // Get the location object from the window
    let location = window.location();

    // Get the protocol, hostname, and port from the location
    let protocol = location.protocol().unwrap();
    let hostname = location.hostname().unwrap();
    let port = location.port().unwrap();

    // Construct the base URL
    let base_url = format!("{}//{}:{}", protocol, hostname, port);

    Some(base_url)
}
