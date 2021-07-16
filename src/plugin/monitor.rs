use std::collections::HashMap;

use appbase::*;
use jsonrpc_core::{Params, Value};
use serde_json;
use serde_json::Map;

use crate::plugin::jsonrpc::JsonRpcPlugin;
use crate::validation::subscribe;

pub struct MonitorPlugin {
    base: PluginBase,
    channels: Option<HashMap<String, ChannelHandle>>,
}

appbase_plugin_requires!(MonitorPlugin; JsonRpcPlugin);

impl Plugin for MonitorPlugin {
    appbase_plugin_default!(MonitorPlugin);

    fn new() -> Self {
        MonitorPlugin {
            base: PluginBase::new(),
            channels: None,
        }
    }

    fn initialize(&mut self) {
        if !self.plugin_initialize() {
            return;
        }

        unsafe {
            let mut channels: HashMap<String, ChannelHandle> = HashMap::new();
            channels.insert("tendermint".to_string(), APP.get_channel("tendermint".to_string()));
            self.channels = Some(channels.to_owned());
        }
        let channels = self.channels.as_ref().unwrap().clone();

        let mut _p1: PluginHandle;
        unsafe {
            _p1 = APP.get_plugin::<JsonRpcPlugin>();
        }
        let mut plugin = _p1.lock().unwrap();
        let jsonrpc = plugin.downcast_mut::<JsonRpcPlugin>().unwrap();
        jsonrpc.add_sync_method("subscribe".to_string(), move |params: Params| {
            let params: Map<String, Value> = params.parse().unwrap();
            let verified = subscribe::verify(&params);
            if verified.is_err() {
                return Ok(Value::String(verified.unwrap_err()));
            }

            let chain = params.get("chain").unwrap().as_str().unwrap();
            let result = match channels.get(chain) {
                None => {
                    Err(String::from("not registered chain"))
                }
                Some(channel) => {
                    channel
                        .lock()
                        .unwrap()
                        .send(Value::Object(params))
                        .unwrap();
                    Ok(String::from("subscribe started"))
                }
            };

            let message = match result {
                Ok(message) => message,
                Err(err_message) => err_message,
            };
            Ok(Value::String(message))
        });
    }

    fn startup(&mut self) {
        if !self.plugin_startup() {
            return;
        }
    }

    fn shutdown(&mut self) {
        if !self.plugin_shutdown() {
            return;
        }
    }
}
