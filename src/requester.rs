use serde_yaml::Value;
use ureq::Request;

pub fn init_requests(yaml: Value) -> Result<(), Box<dyn std::error::Error>> {
    match yaml["https"].is_null() {
        false => run_http_requests(yaml.clone()),
        true => panic!(
            "No valid requests to be sent from the yaml, check your input file for syntax error"
        ),
    }

    Ok(())
}

fn run_http_requests(yaml: Value) {
    let yaml_length = yaml["https"].as_sequence().unwrap().len();
    let mut count = 0;
    while count < yaml_length {
        match yaml["https"][count]["type"].as_str() {
            Some("put") | Some("get") | Some("patch") | Some("delete") | Some("post") => {
                perform_req(yaml["https"][count].clone())
            }
            _ => panic!("Not a valid request type"),
        }
        count += 1;
    }
}

fn perform_req(ctx: Value) {
    let fqdn = build_domain("https://", ctx["domain"].as_str().unwrap());
    println!(
        "Commence api test for {} {} requests",
        &fqdn,
        ctx["type"]
            .as_str()
            .expect("Check your yaml, 'api' misconfigured")
            .to_uppercase()
    );

    let api_ctx = ctx["api"].clone();
    let api_cnt = api_ctx.as_sequence().unwrap().len();
    let mut cur_idx = 0;

    while cur_idx < api_cnt {
        let mut api_call = fqdn.clone();
        api_call.push_str(api_ctx[cur_idx].as_str().unwrap());

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

        cur_idx += 1;
    }
}

fn choose_request_type(
    req_type: &str,
    api_call: &str,
    headers: Value,
    query_strings: Value,
) -> Request {
    let mut request = match req_type.to_lowercase().as_str() {
        "get" => ureq::get(api_call),
        "post" => ureq::post(api_call),
        "delete" => ureq::delete(api_call),
        "patch" => ureq::patch(api_call),
        "put" => ureq::put(api_call),
        _ => panic!("Not a valid request type"),
    };

    if !headers.is_null() {
        request = set_headers(request, headers);
    }

    if !query_strings.is_null() {
        request = set_querystrings(request, query_strings);
    }

    request
}

fn build_domain(fqdn: &str, domain: &str) -> String {
    format!("{fqdn}{domain}")
}

fn set_headers(mut req: Request, header_ctx: Value) -> Request {
    let header_cnt = header_ctx.as_sequence().unwrap().len();
    let mut cur_idx = 0;

    req = req.set("reqev", "true");

    while cur_idx < header_cnt {
        req = req.set(
            header_ctx[cur_idx]["key"]
                .as_str()
                .expect("Check your yaml, 'key' of 'headers' misconfigured"),
            header_ctx[cur_idx]["value"]
                .as_str()
                .expect("Check your yaml, 'value' of 'headers' misconfigured"),
        );
        cur_idx += 1;
    }

    req
}

fn set_querystrings(mut req: Request, qs_ctx: Value) -> Request {
    let qs_cnt = qs_ctx.as_sequence().unwrap().len();
    let mut cur_idx = 0;

    while cur_idx < qs_cnt {
        req = req.query(
            qs_ctx[cur_idx]["key"]
                .as_str()
                .expect("Check your yaml, 'key' of 'query_strings' misconfigured"),
            qs_ctx[cur_idx]["value"]
                .as_str()
                .expect("Check your yaml, 'value' of 'query_strings' misconfigured"),
        );
        cur_idx += 1;
    }

    req
}

#[cfg(test)]
mod requester_tests {
    use super::*;
    use serde_yaml::from_str;

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

        let yaml_contents: Value = from_str(headers).unwrap();

        // let docs = YamlLoader::load_from_str(headers).unwrap();
        // let h_yaml = &docs[0];

        let mut req = ureq::get("http://example.com");

        req = set_headers(req, yaml_contents["headers"].clone());

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

        let yaml_contents: Value = from_str(headers).unwrap();

        // let docs = YamlLoader::load_from_str(headers).unwrap();
        // let q_yaml = &docs[0];

        let mut req = ureq::get("http://example.com");

        req = set_headers(req, yaml_contents["query_strings"].clone());

        assert_eq!("10", req.header("limit").unwrap());
        assert_eq!("year", req.header("t").unwrap());
    }

    #[test]
    #[should_panic]
    fn test_choose_invalid_request_type() {
        let api_call = String::new();
        let h_yaml: Value = from_str("1234").unwrap();
        let q_yaml: Value = from_str("abcd").unwrap();
        let _req = choose_request_type("bad test", &api_call, h_yaml, q_yaml);
    }
}
