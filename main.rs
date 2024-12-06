use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyMemoryResponse {
    responseData: ResponseData,
    #[serde(rename = "responseStatus")]
    response_status: u16,
}

#[derive(Deserialize)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 번역할 텍스트와 언어 설정
    let source_text = "도안";
    let source_lang = "ko"; // 원본 언어
    let target_lang = "en"; // 대상 언어

    // MyMemory API URL 생성
    let api_url = "https://api.mymemory.translated.net/get";
    let client = Client::new();

    // HTTP 요청
    let response = client
        .get(api_url)
        .query(&[
            ("q", source_text),
            ("langpair", &format!("{}|{}", source_lang, target_lang)),
        ])
        .send()
        .await?;

    // 응답 처리
    if response.status().is_success() {
        let mymemory_response: MyMemoryResponse = response.json().await?;
        println!("Original Text: {}", source_text);
        println!("Translated Text: {}", mymemory_response.responseData.translated_text);
    } else {
        eprintln!("Failed to translate: {}", response.text().await?);
    }

    Ok(())
}
