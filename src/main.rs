/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 *
 * Modifications Copyright OpenSearch Contributors. See
 * GitHub history for details.
 */

use aws_config::meta::region::RegionProviderChain;
use std::{convert::TryInto, env, thread, time};

use serde_json::{json, Value};

use opensearch::http::headers::HeaderMap;
use opensearch::http::request::JsonBody;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::{Method, Url};
use opensearch::OpenSearch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let url = Url::parse(&env::var("ENDPOINT").expect("Missing ENDPOINT"));
    let service_name = &env::var("SERVICE").unwrap_or("es".to_string());
    let conn_pool = SingleNodeConnectionPool::new(url?);
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await
        .clone();
    let transport = TransportBuilder::new(conn_pool)
        .auth(aws_config.clone().try_into()?)
        .service_name(service_name)
        .build()?;
    let client = OpenSearch::new(transport);

    // TODO: remove when OpenSearch Serverless adds support for GET /
    if service_name == "es" {
        let info: Value = client
            .send::<(), ()>(Method::Get, "/", HeaderMap::new(), None, None, None)
            .await?
            .json()
            .await?;
        println!(
            "{}: {}",
            info["version"]["distribution"].as_str().unwrap(),
            info["version"]["number"].as_str().unwrap()
        );
    }

    let index_name = "movies";
    let document_id = "1";

    let index_body: JsonBody<_> = json!({
        "settings": {
            "index": {
                "number_of_shards" : 4
            }
        }
    })
    .into();

    let create_index_response = client
        .send(
            Method::Put,
            &format!("/{index_name}"),
            HeaderMap::new(),
            Option::<&()>::None,
            Some(index_body),
            None,
        )
        .await?;

    assert_eq!(create_index_response.status_code(), 200);

    let document: JsonBody<_> = json!({
        "title": "Moneyball",
        "director": "Bennett Miller",
        "year": "2011"
    })
    .into();
    let create_document_response = client
        .send(
            Method::Put,
            &format!("/{index_name}/_doc/{document_id}"),
            HeaderMap::new(),
            Some(&[("refresh", "true")]),
            Some(document),
            None,
        )
        .await?;

    assert_eq!(create_document_response.status_code(), 201);

    thread::sleep(time::Duration::from_secs(5));

    let q = "miller";
    let query: JsonBody<_> = json!({
        "size": 5,
        "query": {
            "multi_match": {
                "query": q,
                "fields": ["title^2", "director"]
            }
        }
    })
    .into();

    let search_response = client
        .send(
            Method::Post,
            &format!("/{index_name}/_search"),
            HeaderMap::new(),
            Option::<&()>::None,
            Some(query),
            None,
        )
        .await?;

    assert_eq!(search_response.status_code(), 200);
    let search_result = search_response.json::<Value>().await?;
    println!(
        "Hits: {:#?}",
        search_result["hits"]["hits"].as_array().unwrap()
    );

    let delete_document_response = client
        .send::<(), ()>(
            Method::Delete,
            &format!("/{index_name}/_doc/{document_id}"),
            HeaderMap::new(),
            None,
            None,
            None,
        )
        .await?;

    assert_eq!(delete_document_response.status_code(), 200);

    let delete_response = client
        .send::<(), ()>(
            Method::Delete,
            &format!("/{index_name}"),
            HeaderMap::new(),
            None,
            None,
            None,
        )
        .await?;

    assert_eq!(delete_response.status_code(), 200);

    Ok(())
}
