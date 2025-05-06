use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};

pub struct ReceiveUserExtractController {
    // Aqui será injetado o serviço quando for implementado
}

impl ReceiveUserExtractController {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn process_files(
    controller: web::Data<ReceiveUserExtractController>,
    user_id: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut extract_file = None;
    let mut credit_card_file = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or("");

        match field_name {
            "extract" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    data.extend_from_slice(&chunk?);
                }
                extract_file = Some(data);
            }
            "creditCard" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    data.extend_from_slice(&chunk?);
                }
                credit_card_file = Some(data);
            }
            _ => {}
        }
    }

    if extract_file.is_none() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Arquivo de extrato é obrigatório"
        })));
    }

    if credit_card_file.is_none() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Arquivo de fatura é obrigatório"
        })));
    }

    // Aqui será implementado o processamento dos arquivos quando o serviço for criado
    // Por enquanto, apenas retornamos um sucesso
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Arquivos recebidos com sucesso"
    })))
} 