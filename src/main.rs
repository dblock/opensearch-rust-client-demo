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
    use aws_config::meta::region::RegionProviderChain;
    use std::{convert::TryInto, env, thread, time};

    use serde_json::Value;

    use opensearch::{
        http::transport::{SingleNodeConnectionPool, TransportBuilder},
        OpenSearch,
    };

    use url::Url;

    tracing_subscriber::fmt::init();

    let url = Url::parse(&env::var("ENDPOINT").expect("Missing ENDPOINT"));
    let service_name = &env::var("SERVICE").unwrap_or("es".to_string());
    let conn_pool = SingleNodeConnectionPool::new(url?);
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::from_env().region(region_provider).load().await.clone();
    let transport = TransportBuilder::new(conn_pool)
        .auth(aws_config.clone().try_into()?)
        .service_name(service_name)
        .build()?;
    let client = OpenSearch::new(transport);

    // TODO: remove when OpenSearch Serverless adds support for GET /
    if service_name == "es" {
        let info: Value = client.info().send().await?.json().await?;
        println!(
            "{}: {}",
            info["version"]["distribution"].as_str().unwrap(),
            info["version"]["number"].as_str().unwrap()
        );
    }

    let index_name = "movies";

    client
        .indices()
        .create(opensearch::indices::IndicesCreateParts::Index(index_name))
        .send()
        .await?;

    client
        .index(opensearch::IndexParts::Index(index_name))
        .body(serde_json::json!({
                "title": "Moneyball",
                "director": "Bennett Miller",
                "year": 2011
            }
        ))
        .send()
        .await?;

    thread::sleep(time::Duration::from_secs(5));

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
    println!("{}", serde_json::to_string_pretty(&response_body).unwrap());

    client
        .indices()
        .delete(opensearch::indices::IndicesDeleteParts::Index(&[
            index_name,
        ]))
        .send()
        .await?;

    Ok(())
}
