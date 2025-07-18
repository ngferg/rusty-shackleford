#[derive(serde::Serialize, serde::Deserialize)]
struct Payload {
    model: String,
    prompt: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Resp {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:11434/api/generate".to_string();
    let payload = Payload {
        model: "llama3.2".to_string(),
        prompt: "Why is the sky blue?".to_string(),
    };

    let json_data = serde_json::to_string(&payload).unwrap();

    let res = client
        .post(url)
        .body(json_data)
        .send()
        .await?
        .text()
        .await?;

    for line in res.lines() {
        let resp: Resp = serde_json::from_str(line).unwrap();
        print!("{}", resp.response);
    }

    // println!("{:?}", res);

    Ok(())
}
