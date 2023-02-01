# OpenSearch Ruby Client Demo

Makes requests to [Amazon OpenSearch Service](https://aws.amazon.com/opensearch-service/) using the [OpenSearch Rust Client](https://github.com/opensearch-project/opensearch-rs). Supports [OpenSearch Serverless](https://aws.amazon.com/opensearch-service/features/serverless/) since version 2.1.0.

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
export AWS_REGION=us-west-2

export ENDPOINT=https://....us-west-2.es.amazonaws.com
export SERVICE=es # use "aoss" for OpenSearch Serverless 
cargo run
```

This will output the version of OpenSearch and a search result.

```
opensearch: 2.3.0

{
  "_shards": {
    "failed": 0,
    "skipped": 0,
    "successful": 5,
    "total": 5
  },
  "hits": {
    "hits": [
      {
        "_id": "8qhYDoYBpJvbGlZLDVk_",
        "_index": "movies",
        "_score": 0.2876821,
        "_source": {
          "director": "Bennett Miller",
          "title": "Moneyball",
          "year": 2011
        }
      }
    ],
    "max_score": 0.2876821,
    "total": {
      "relation": "eq",
      "value": 1
    }
  },
  "timed_out": false,
  "took": 4
}
```

The [code](src/main.rs) creates an index, adds a document, then cleans up.

## License 

This project is licensed under the [Apache v2.0 License](LICENSE.txt).

## Copyright

Copyright OpenSearch Contributors. See [NOTICE](NOTICE.txt) for details.
