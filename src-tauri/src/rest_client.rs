use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:8080/broker";

#[derive(Default)]
pub struct RestClient {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOfferPayload {
    pub room_id: String,
    pub user_id: String,
    pub offer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswerPayload {
    pub room_id: String,
    pub user_id: String,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptPayload {
    pub room_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferResponse {
    pub ok: bool,
    pub offer: CreateOfferPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerResponse {
    pub ok: bool,
    pub answer: CreateAnswerPayload,
}

impl RestClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder();
        Ok(Self {
            client: client.build()?,
        })
    }

    pub async fn create_offer(&self, options: CreateOfferPayload) -> Result<OfferResponse> {
        let url = format!("{BASE_URL}/create-offer");
        let response = self.client.post(url).json(&options).send().await?;
        Ok(response.json().await?)
    }

    pub async fn accept_offer(&self, options: AcceptPayload) -> Result<OfferResponse> {
        let url = format!("{BASE_URL}/accept-offer");
        let response = self.client.post(url).json(&options).send().await?;
        Ok(response.json().await?)
    }

    pub async fn create_answer(&self, options: CreateAnswerPayload) -> Result<AnswerResponse> {
        let url = format!("{BASE_URL}/create-answer");
        let response = self.client.post(url).json(&options).send().await?;
        Ok(response.json().await?)
    }

    pub async fn accept_answer(&self, options: AcceptPayload) -> Result<AnswerResponse> {
        let url = format!("{BASE_URL}/accept-answer");
        let response = self.client.post(url).json(&options).send().await?;
        Ok(response.json().await?)
    }
}
