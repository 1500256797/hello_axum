// curl https://api.openai.com/v1/chat/completions \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer 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" \
//   -d '{
//      "model": "gpt-3.5-turbo",
//      "messages": [{"role": "user", "content": "麻烦帮我写个300百字的检讨"}],
//      "temperature": 0.7
//    }'
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};

#[tokio::test]
async fn main() {
    let api_key = "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";
    let client = Client::new(api_key.to_string());

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                name: None,
            },
            ChatMessage {
                role: Role::User,
                content: "帮我写一个1000字的检讨".to_string(),
                name: None,
            },
        ],
        temperature: None,
        top_p: None,
        n: None,
        stop: None,
        max_tokens: Some(2002),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        // or use ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    // 获取result 中的 chioces
    let response = result.choices[0].message.content.clone();
    // meet \n

    println!("{:?}", response);
}
