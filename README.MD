# What is this?
A simple tool where you specify a yaml file to run a set of requests. This tool simplifies the need to write many scripts to test a web app or API, and instead you can maintain 1 yaml file with some specific keywords.

# How to Use?
Download the latest release on the right side of the page.

# Usage
```shell
    reqev $file_format (only yaml for now) $yaml_file
    
    reqev yaml tests/test.yaml
```

# Result
```shell
Commence api test for https://api.apis.guru GET requests
GET: https://api.apis.guru/v2/list.json
200
Commence api test for https://www.reddit.com GET requests
GET: https://www.reddit.com/r/Wallstreetbets/top.json?limit=10&t=year
200
Commence api test for https://httpbin.org DELETE requests
DELETE: https://httpbin.org/status/200
200
Commence api test for https://httpbin.org GET requests
GET: https://httpbin.org/status/200
200
Commence api test for https://httpbin.org PATCH requests
PATCH: https://httpbin.org/status/200
200
Commence api test for https://httpbin.org POST requests
POST: https://httpbin.org/status/200
200
Commence api test for https://httpbin.org PUT requests
PUT: https://httpbin.org/status/200
200

```

# Example Yaml file
Start with `https` is required. Later this may support other web protocols.
```yaml
https:
  - domain: api.apis.guru
    type: get
    headers:
      - key: accept
        value: text/plain
    expect:
      - status: 200
    api:
      - /v2/list.json
      - /v2/list2.json
    query_strings:
      - key: limit
        value: "10"
      - key: t
        value: year
```

# Roadmap
Updated 2024/06/02
- [ ] Add functionality to specify expected response for get, post, etc.
- [ ] Add support for RPC?
- [ ] Add support for json file format
- [ ] Add support for Auth tokens
- [ ] Clean/Simplify logic for serializing yaml

# Contributing
Message me, I still have to work it out.

# License
TODO, if I don't update here soon check the License in the repo.