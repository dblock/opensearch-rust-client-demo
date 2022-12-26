# OpenSearch Ruby Client Demo

Makes requests to Amazon OpenSearch using the [OpenSearch Rust Client](https://github.com/opensearch-project/opensearch-rs).

## Prerequisites

### Rust

Install [Rust](https://www.rust-lang.org/tools/install). YMMV.

```
$ rustc --version
rustc 1.65.0
```

## Running

Create an OpenSearch domain in (AWS) which support IAM based AuthN/AuthZ.

```
export AWS_ACCESS_KEY_ID=
export AWS_SECRET_ACCESS_KEY=
export AWS_SESSION_TOKEN=
export AWS_REGION=us-west2

export OPENSEARCH_ENDPOINT=https://....us-west-2.es.amazonaws.com

cargo run
```

This will output the version of OpenSearch and a search result.

```
opensearch: 2.3.0
Object {"director": String("Bennett Miller"), "id": Number(1), "title": String("Moneyball"), "year": Number(2011)}
```

The [code](src/main.rs) will create an index, add a document, then cleanup.

## License 

This project is licensed under the [Apache v2.0 License](LICENSE.txt).

## Copyright

Copyright OpenSearch Contributors. See [NOTICE](NOTICE.txt) for details.
