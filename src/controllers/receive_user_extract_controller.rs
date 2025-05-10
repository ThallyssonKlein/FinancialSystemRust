use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use crate::adapter::receive_user_extract_adapter::ReceiveUserExtractAdapter;

pub struct ReceiveUserExtractController {
    adapter: ReceiveUserExtractAdapter,
}

impl ReceiveUserExtractController {
    pub fn new(adapter: ReceiveUserExtractAdapter) -> Self {
        Self { adapter }
    }
}

pub async fn process_files(
    controller: web::Data<ReceiveUserExtractController>,
    user_id: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut extract_file = None;
    let mut credit_card_file = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap_or("unknown");

        if filename.contains("extrato") {
            extract_file = Some(payload);
            break;
        } else if filename.contains("cartao") {
            credit_card_file = Some(payload);
            break;
        }
    }

    let extract_file = extract_file.ok_or_else(|| {
        actix_web::error::ErrorBadRequest("Arquivo de extrato não encontrado")
    })?;

    let credit_card_file = credit_card_file.ok_or_else(|| {
        actix_web::error::ErrorBadRequest("Arquivo de cartão de crédito não encontrado")
    })?;

    controller.adapter.process_files(user_id.into_inner(), extract_file, credit_card_file).await?;

    Ok(HttpResponse::Ok().json("Arquivos processados com sucesso"))
} 