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

    let index_name = "movies";

    client
        .indices()
        .create(opensearch::indices::IndicesCreateParts::Index(index_name))
        .send()
        .await?;

    client
        .index(opensearch::IndexParts::IndexId(index_name, "1"))
        .body(serde_json::json!({
                "id": 1,
                "title": "Moneyball",
                "director": "Bennett Miller",
                "year": 2011
            }
        ))
        .send()
        .await?;

    thread::sleep(time::Duration::from_secs(1));

    let response = client
        .search(opensearch::SearchParts::Index(&[index_name]))
        .body(serde_json::json!({
                "query": {
                    "match": {
                        "director": "miller"
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
            index_name,
        ]))
        .send()
        .await?;

    Ok(())
}
