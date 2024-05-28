use ureq::Request;
use yaml_rust2::Yaml;

pub fn init_requests(yaml: Vec<Yaml>) -> Result<(), Box<dyn std::error::Error>> {
    let doc = &yaml[0];

    match doc["https"].is_badvalue() {
        false => run_http_requests(doc["https"].clone()),
        true => panic!(
            "No valid requests to be sent from the yaml, check your input file for syntax error"
        ),
    }

    Ok(())
}

fn run_http_requests(http_yaml: Yaml) {
    let reqs_ctx = http_yaml.into_iter();

    for ctx in reqs_ctx {
        // dbg!(&ctx);
        match &ctx["type"].as_str() {
            Some("put") | Some("get") | Some("patch") | Some("delete") | Some("post") => {
                perform_req(ctx)
            }
            _ => panic!("Not a valid request type"),
        }
    }
}

fn perform_req(ctx: Yaml) {
    let fqdn = build_domain("https://", ctx["domain"].as_str().unwrap());
    println!(
        "Commence api test for {} {} requests",
        &fqdn,
        ctx["type"]
            .as_str()
            .expect("Check your yaml, 'api' misconfigured")
            .to_uppercase()
    );

    let api_ctx = ctx["api"].clone().into_iter();

    for api in api_ctx {
        let mut api_call = fqdn.clone();
        api_call.push_str(api.as_str().unwrap());

        let req_type = ctx["type"].as_str().unwrap();

        let headers = ctx["headers"].clone();
        let query_strings = ctx["query_strings"].clone();
        let request = choose_request_type(req_type, &api_call, headers, query_strings);

        println!("{}: {}", req_type.to_uppercase(), request.url());

        match request.call() {
            Ok(response) => println!("{}", response.status()),
            Err(ureq::Error::Status(code, _response)) => {
                panic!("{}, double check if the response is expected.", code)
            }
            Err(_) => panic!("Unexpected error occurred"),
        }
    }
}

fn choose_request_type(
    req_type: &str,
    api_call: &str,
    headers: Yaml,
    query_strings: Yaml,
) -> Request {
    let mut request = match req_type.to_lowercase().as_str() {
        "get" => ureq::get(api_call),
        "post" => ureq::post(api_call),
        "delete" => ureq::delete(api_call),
        "patch" => ureq::patch(api_call),
        "put" => ureq::put(api_call),
        _ => panic!("Not a valid request type"),
    };
    request = set_headers(request, headers);
    request = set_querystrings(request, query_strings);
    request
}

fn build_domain(fqdn: &str, domain: &str) -> String {
    format!("{fqdn}{domain}")
}

fn set_headers(mut req: Request, header_ctx: Yaml) -> Request {
    let header_iter = header_ctx.into_iter();

    req = req.set("reqev", "true");

    for header in header_iter {
        req = req.set(
            header["key"]
                .as_str()
                .expect("Check your yaml, 'key' of 'headers' misconfigured"),
            header["value"]
                .as_str()
                .expect("Check your yaml, 'value' of 'headers' misconfigured"),
        );
    }

    req
}

fn set_querystrings(mut req: Request, qs_ctx: Yaml) -> Request {
    let qs_iter = qs_ctx.into_iter();

    req = req.set("reqev", "true");

    for qs in qs_iter {
        req = req.query(
            qs["key"]
                .as_str()
                .expect("Check your yaml, 'key' of 'query_strings' misconfigured"),
            qs["value"]
                .as_str()
                .expect("Check your yaml, 'value' of 'query_strings' misconfigured"),
        );
    }

    req
}

#[cfg(test)]
mod requester_tests {
    use super::*;
    use yaml_rust2::{Yaml, YamlLoader};

    #[test]
    fn test_build_domain_simple() {
        let fqdn = build_domain("https://test.io", "/api/v1/1234");
        assert_eq!(fqdn, "https://test.io/api/v1/1234")
    }

    #[test]
    fn test_set_headers() {
        let headers = "
        headers:
        - key: accept
          value: text/plain
        ";

        let docs = YamlLoader::load_from_str(headers).unwrap();
        let h_yaml = &docs[0];

        let mut req = ureq::get("http://example.com");

        req = set_headers(req, h_yaml["headers"].clone());

        assert_eq!("true", req.header("reqev").unwrap());
        assert_eq!("text/plain", req.header("accept").unwrap());
    }

    #[test]
    fn test_set_querystrings() {
        let headers = "
        query_strings:
          - key: limit
            value: \"10\"
          - key: t
            value: year
        ";

        let docs = YamlLoader::load_from_str(headers).unwrap();
        let q_yaml = &docs[0];

        let mut req = ureq::get("http://example.com");

        req = set_headers(req, q_yaml["query_strings"].clone());

        assert_eq!("10", req.header("limit").unwrap());
        assert_eq!("year", req.header("t").unwrap());
    }

    #[test]
    #[should_panic]
    fn test_choose_invalid_request_type() {
        let api_call = String::new();
        let h_yaml = Yaml::from_str("1234");
        let q_yaml = Yaml::from_str("abcd");
        let _req = choose_request_type("bad test", &api_call, h_yaml, q_yaml);
    }
}
