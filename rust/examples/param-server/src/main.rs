//! Example of a parameter server using the Foxglove SDK.
//!
//! This example uses the 'unstable' feature to expose capabilities.
//!
//! Usage:
//! ```text
//! cargo run -p example-param-server
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use clap::Parser;
use foxglove::websocket::{
    Capability, Client, Parameter, ParameterType, ParameterValue, ServerListener,
};
use foxglove::{WebSocketServer, WebSocketServerHandle};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 8765)]
    port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

struct ParamListener {
    param_store: Mutex<HashMap<String, Parameter>>,
}

impl ParamListener {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            param_store: Mutex::new(HashMap::new()),
        })
    }
}

impl ServerListener for ParamListener {
    fn on_get_parameters(
        &self,
        _client: Client,
        param_names: Vec<String>,
        request_id: Option<&str>,
    ) -> Vec<Parameter> {
        println!(
            "on_get_parameters called with parameter names: {:?}; request_id: {}",
            param_names,
            request_id.unwrap_or("None")
        );

        let params = self.param_store.lock().unwrap();
        if param_names.is_empty() {
            params.values().cloned().collect()
        } else {
            param_names
                .iter()
                .filter_map(|name| params.get(name).cloned())
                .collect()
        }
    }

    fn on_set_parameters(
        &self,
        _client: Client,
        mut parameters: Vec<Parameter>,
        request_id: Option<&str>,
    ) -> Vec<Parameter> {
        let param_names: Vec<String> = parameters.iter().map(|param| param.name.clone()).collect();
        println!(
            "on_set_parameters called with parameter names: {:?}; request_id: {}",
            param_names,
            request_id.unwrap_or("None")
        );

        let mut params = self.param_store.lock().unwrap();
        for param in parameters.iter_mut() {
            match params.get_mut(&param.name) {
                Some(old_param) => {
                    if param.name.starts_with("read_only_") {
                        // Return the old value
                        param.value = old_param.value.clone();
                        param.r#type = old_param.r#type.clone();
                    } else {
                        // Update the value
                        old_param.value = param.value.clone();
                        old_param.r#type = param.r#type.clone();
                    }
                }
                _ => {
                    params.insert(param.name.clone(), param.clone());
                }
            }
        }
        parameters
    }

    fn on_parameters_subscribe(&self, param_names: Vec<String>) {
        println!(
            "on_parameters_subscribe called with parameter names: {:?}",
            param_names
        );
    }

    fn on_parameters_unsubscribe(&self, param_names: Vec<String>) {
        println!(
            "on_parameters_unsubscribe called with parameter names: {:?}",
            param_names
        );
    }
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let args = Cli::parse();
    let listener = ParamListener::new();

    // Initialize the parameter store with some example parameters
    {
        let mut param_store = listener.param_store.lock().unwrap();
        param_store.insert(
            "read_only_str_param".to_string(),
            Parameter {
                name: "read_only_str_param".to_string(),
                value: Some(ParameterValue::String(
                    "can't change me".as_bytes().to_vec(),
                )),
                r#type: None,
            },
        );
        param_store.insert(
            "elapsed".to_string(),
            Parameter {
                name: "elapsed".to_string(),
                value: Some(ParameterValue::Number(0.0)),
                r#type: None,
            },
        );
        param_store.insert(
            "float_array_param".to_string(),
            Parameter {
                name: "float_array_param".to_string(),
                value: Some(ParameterValue::Array(vec![
                    ParameterValue::Number(1.0),
                    ParameterValue::Number(2.0),
                    ParameterValue::Number(3.0),
                ])),
                r#type: Some(ParameterType::Float64Array),
            },
        );
    }

    let server = WebSocketServer::new()
        .name("param server")
        .capabilities([Capability::Parameters])
        .listener(listener.clone())
        .bind(args.host, args.port)
        .start()
        .await
        .expect("Failed to start server");

    let shutdown = watch_ctrl_c();
    tokio::select! {
        () = shutdown.cancelled() => (),
        () = update_parameters(&server, listener) => (),
    };

    server.stop().await;
}

async fn update_parameters(server: &WebSocketServerHandle, _listener: Arc<ParamListener>) {
    let start = Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let parameter = Parameter {
            name: "elapsed".to_string(),
            value: Some(ParameterValue::Number(start.elapsed().as_secs_f64())),
            r#type: Some(ParameterType::Float64),
        };
        server.publish_parameter_values(vec![parameter]);
    }
}

fn watch_ctrl_c() -> CancellationToken {
    let token = CancellationToken::new();
    tokio::spawn({
        let token = token.clone();
        async move {
            tokio::signal::ctrl_c().await.ok();
            token.cancel();
        }
    });
    token
}
