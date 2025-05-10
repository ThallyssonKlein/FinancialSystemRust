use actix_multipart::Multipart;
use actix_web::{web, Error};
use csv::{Reader, ReaderBuilder};
use futures::{StreamExt, TryStreamExt};
use std::io::Cursor;
use crate::domain::service::process_files_service::ProcessFilesService;

#[derive(Debug)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub value: String,
}

pub struct ReceiveUserExtractAdapter {
    process_files_service: ProcessFilesService,
}

impl ReceiveUserExtractAdapter {
    pub fn new(process_files_service: ProcessFilesService) -> Self {
        Self {
            process_files_service,
        }
    }

    pub async fn process_files(
        &self,
        user_id: String,
        extract_file: Multipart,
        credit_card_file: Multipart,
    ) -> Result<(), Error> {
        let extract_items = self.process_extract_file(extract_file).await?;
        let credit_card_items = self.process_credit_card_file(credit_card_file).await?;

        self.process_files_service.process_files(extract_items, &user_id, "extract");
        self.process_files_service.process_files(credit_card_items, &user_id, "credit");

        Ok(())
    }

    async fn process_extract_file(&self, mut payload: Multipart) -> Result<Vec<Transaction>, Error> {
        let mut items = Vec::new();
        let mut header_found = false;

        while let Some(mut field) = payload.try_next().await? {
            let content_type = field.content_disposition();
            let filename = content_type.get_filename().unwrap_or("unknown");

            let mut content = Vec::new();
            while let Some(chunk) = field.next().await {
                content.extend_from_slice(&chunk?);
            }

            let cursor = Cursor::new(content);
            let mut reader = ReaderBuilder::new()
                .delimiter(b';')
                .flexible(true)
                .from_reader(cursor);

            for result in reader.records() {
                let record = result.map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Erro ao ler CSV: {}", e))
                })?;

                if !header_found {
                    if record.get(0).map_or(false, |v| v == "Data Lançamento") {
                        header_found = true;
                    }
                    continue;
                }

                if record.is_empty() || (record.len() == 1 && record.get(0).unwrap_or("").is_empty()) {
                    continue;
                }

                items.push(Transaction {
                    date: record.get(0).unwrap_or("").to_string(),
                    description: record.get(1).unwrap_or("").to_string(),
                    value: record.get(2).unwrap_or("").to_string(),
                });
            }
        }

        if items.is_empty() {
            return Err(actix_web::error::ErrorBadRequest("Nenhum item encontrado no arquivo"));
        }

        Ok(items)
    }

    async fn process_credit_card_file(
        &self,
        mut payload: Multipart,
    ) -> Result<Vec<Transaction>, Error> {
        let mut items = Vec::new();
        let mut header_found = false;

        while let Some(mut field) = payload.try_next().await? {
            let content_type = field.content_disposition();
            let filename = content_type.get_filename().unwrap_or("unknown");

            let mut content = Vec::new();
            while let Some(chunk) = field.next().await {
                content.extend_from_slice(&chunk?);
            }

            let cursor = Cursor::new(content);
            let mut reader = ReaderBuilder::new()
                .delimiter(b',')
                .flexible(true)
                .from_reader(cursor);

            for result in reader.records() {
                let record = result.map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Erro ao ler CSV: {}", e))
                })?;

                if !header_found {
                    if record.get(0).map_or(false, |v| v.contains("Descricao")) {
                        header_found = true;
                    }
                    continue;
                }

                if record.is_empty() || (record.len() == 1 && record.get(0).unwrap_or("").is_empty()) {
                    continue;
                }

                let first_field = record.get(0).unwrap_or("");
                if first_field == "Cartão Principal" || first_field == "Vencimento" || first_field == "Total" {
                    continue;
                }

                let valor = record.get(6).unwrap_or("").trim_matches('"');
                let valor = if valor.starts_with("-R$ ") {
                    format!("-{}", valor.trim_start_matches("-R$ "))
                } else if valor.starts_with("R$ ") {
                    valor.trim_start_matches("R$ ").to_string()
                } else {
                    valor.to_string()
                };

                if record.get(1).unwrap_or("").is_empty()
                    || record.get(3).unwrap_or("").is_empty()
                    || record.get(6).unwrap_or("").is_empty()
                {
                    continue;
                }

                items.push(Transaction {
                    date: record.get(1).unwrap_or("").trim_matches('"').to_string(),
                    description: record.get(3).unwrap_or("").trim_matches('"').to_string(),
                    value: valor,
                });
            }
        }

        if items.is_empty() {
            return Err(actix_web::error::ErrorBadRequest("Nenhum item encontrado no arquivo"));
        }

        Ok(items)
    }
} 