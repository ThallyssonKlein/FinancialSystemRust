use crate::adapter::inbound::receive_user_extract_adapter::Transaction;
use crate::adapter::outbound::key_value_database_adapter::OutboundKeyValueDatabaseAdapter;
use std::error::Error;

pub struct ProcessFilesService {
    credit_card_adapter: OutboundKeyValueDatabaseAdapter,
    extract_adapter: OutboundKeyValueDatabaseAdapter,
}

impl ProcessFilesService {
    pub fn new(credit_card_adapter: OutboundKeyValueDatabaseAdapter, extract_adapter: OutboundKeyValueDatabaseAdapter) -> Self {
        Self {
            credit_card_adapter,
            extract_adapter,
        }
    }

    fn order_items_by_date(&self, items: &mut [Transaction]) {
        items.sort_by(|a, b| a.date.cmp(&b.date));
    }

    async fn get_last_item_key_value_database(&self, user_id: &str, credit_or_extract: &str) -> Result<Option<Transaction>, Box<dyn Error>> {
        match credit_or_extract {
            "credit" => self.credit_card_adapter.get_last_item(user_id, "credit").await,
            "extract" => self.extract_adapter.get_last_item(user_id, "extract").await,
            _ => Err("Tipo de arquivo inválido".into()),
        }
    }

    fn look_for_last_item_in_array(&self, items: &[Transaction], last_item: &Transaction) -> Option<usize> {
        items.iter().position(|item| 
            item.date == last_item.date && 
            item.description == last_item.description && 
            item.value == last_item.value
        )
    }

    fn print_new_items(&self, items: &[Transaction]) {
        for item in items {
            println!("{:?}", item);
        }
    }

    pub async fn process_files(
        &self,
        mut items: Vec<Transaction>,
        user_id: &str,
        credit_or_extract: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.order_items_by_date(&mut items);

        let last_item = self.get_last_item_key_value_database(user_id, credit_or_extract).await?;

        match last_item {
            Some(last_item) => {
                if let Some(found_index) = self.look_for_last_item_in_array(&items, &last_item) {
                    self.print_new_items(&items[found_index + 1..]);
                } else {
                    self.print_new_items(&items);
                }
            }
            None => {
                self.print_new_items(&items);
            }
        }

        if let Some(last_item) = items.last() {
            match credit_or_extract {
                "credit" => self.credit_card_adapter.save_last_item(user_id, last_item, "credit").await?,
                "extract" => self.extract_adapter.save_last_item(user_id, last_item, "extract").await?,
                _ => return Err("Tipo de arquivo inválido".into()),
            }
        }

        Ok(())
    }
} 