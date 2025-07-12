// https://countries.trevorblades.com に対して GraphQL クエリを送信するテスト

#[derive(serde::Deserialize, Debug)]
struct GraphQlRespWrapper<T> {
    data: T,
}

#[derive(serde::Deserialize, Debug)]
struct Country {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct QueryResponse {
    countries: Vec<Country>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_graphql_query_countries_name() {
        let client = reqwest::Client::new();

        let query = r#"
        query {
            countries {
                name
            }
        }
        "#;

        let json_body = serde_json::json!({
            "query": query
        });

        let resp = client
            .post("https://countries.trevorblades.com")
            .json(&json_body)
            .send()
            .await
            .expect("リクエストの送信に失敗しました");

        let resp_json: GraphQlRespWrapper<QueryResponse> = resp
            .json()
            .await
            .expect("レスポンスの JSON 解析に失敗しました");

        assert_eq!(
            resp_json.data.countries[0].name, "Andorra",
            "最初の国の名前が期待と異なります"
        );
        eprintln!("最初の国の名前: {}", resp_json.data.countries[0].name);
    }
}
