use wowser::net;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let verb = args.get(1).expect("Verb not passed in");
    let domain = args.get(2).expect("domain not passed in");

    let mut request = net::HttpRequest::new(net::Url::new(
        net::UrlProtocol::HTTP,
        net::UrlHost::DomainName(domain.to_string()),
        80,
        "",
        "",
        "",
    ));
    println!("Created request");
    let result = match verb.as_str() {
        "get" => futures::executor::block_on(request.get()),
        "head" => futures::executor::block_on(request.head()),
        _ => panic!("Unsupported HTTP verb {}", domain),
    };
    let response = result.expect("HttpRequest failed");
    println!(
        "HTTP Response {:?}\nBody\n{}",
        response,
        String::from_utf8_lossy(&response.body)
    );
}
