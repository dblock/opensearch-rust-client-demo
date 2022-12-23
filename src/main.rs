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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{convert::TryInto, env, thread, time};

    use serde_json::Value;

    use opensearch::{
        http::transport::{SingleNodeConnectionPool, TransportBuilder},
        OpenSearch,
    };

    use url::Url;

    let url = Url::parse(&env::var("OPENSEARCH_ENDPOINT").expect("Missing OPENSEARCH_ENDPOINT"));
    let conn_pool = SingleNodeConnectionPool::new(url?);
    let aws_config = aws_config::load_from_env().await.clone();
    let transport = TransportBuilder::new(conn_pool)
        .auth(aws_config.clone().try_into()?)
        .build()?;
    let client = OpenSearch::new(transport);

    let info: Value = client.info().send().await?.json().await?;
    println!(
        "{}: {}",
        info["version"]["distribution"].as_str().unwrap(),
        info["version"]["number"].as_str().unwrap()
    );

    client
        .indices()
        .create(opensearch::indices::IndicesCreateParts::Index("test-index"));

    client
        .index(opensearch::IndexParts::IndexId("test-index", "1"))
        .body(serde_json::json!({
                "id": 1,
                "first_name": "Bruce"
            }
        ))
        .send()
        .await?;

    thread::sleep(time::Duration::from_secs(3));

    let response = client
        .search(opensearch::SearchParts::Index(&["test-index"]))
        .body(serde_json::json!({
                "query": {
                    "match": {
                        "first_name": "bruce"
                    }
                }
            }
        ))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        println!("{:?}", hit["_source"]);
    }

    client
        .indices()
        .delete(opensearch::indices::IndicesDeleteParts::Index(&[
            "test-index",
        ]));

    Ok(())
}
