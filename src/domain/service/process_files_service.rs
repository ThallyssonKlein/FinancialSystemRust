use crate::adapter::receive_user_extract_adapter::Transaction;

pub struct ProcessFilesService {
    // Aqui serão injetadas as dependências necessárias posteriormente
}

impl ProcessFilesService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_files(&self, transactions: Vec<Transaction>, user_id: &str, file_type: &str) {
        // Aqui será implementada a lógica de processamento
        println!("Processando {} transações do tipo {} para o usuário {}", 
            transactions.len(), 
            file_type, 
            user_id
        );
    }
} 