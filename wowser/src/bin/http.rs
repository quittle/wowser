use futures::Future;
use wowser::log;
use wowser::net::async_net::NETWORK_EXECUTOR;
use wowser::net::{self, HttpResult};
use wowser::util::{Executor, TaskToken};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let verb = args.get(1).expect("Verb not passed in");
    let url = args.get(2).expect("Url not passed in");

    let parsed_url = net::Url::parse(url).unwrap();

    let request = net::HttpRequest::new(parsed_url);
    let request_token = match verb.as_str() {
        "get" => run_on_network_executor(request.get()),
        "head" => run_on_network_executor(request.head()),
        _ => panic!("Unsupported HTTP verb {}", verb),
    };

    loop {
        let executor = NETWORK_EXECUTOR.lock().unwrap();
        if let Some(result) = executor.get_result(request_token) {
            let response = result.unwrap().unwrap();
            log!(INFO:
                "HTTP Response", response,
                "\nBody\n",
                String::from_utf8_lossy(&response.body)
            );
            break;
        }
    }
}

fn run_on_network_executor(future: impl Future<Output = HttpResult> + 'static + Send) -> TaskToken {
    NETWORK_EXECUTOR.lock().unwrap().run(future).unwrap()
}
