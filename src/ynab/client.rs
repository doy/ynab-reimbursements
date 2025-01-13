#[derive(Debug, snafu::Snafu)]
pub enum Error {
    // ynab-api error types don't implement Error, so can't use the
    // auto-source behavior
    #[snafu(display("failed to update transactions: {}", source_msg))]
    UpdateTransactions { source_msg: String },

    #[snafu(display("failed to get budgets: {}", source_msg))]
    GetBudgets { source_msg: String },

    #[snafu(display("failed to get budget {}: {}", id, source_msg))]
    GetBudgetById { id: String, source_msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
    configuration: ynab_api::apis::configuration::Configuration,
    rt: tokio::runtime::Runtime,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let mut configuration =
            ynab_api::apis::configuration::Configuration::new();
        configuration.bearer_access_token = Some(key.to_string());
        Self {
            configuration,
            rt: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    pub fn default_budget(&self) -> Result<ynab_api::models::BudgetDetail> {
        let budget_id = self
            .rt
            .block_on(ynab_api::apis::budgets_api::get_budgets(
                &self.configuration,
                None,
            ))
            .map_err(|e| Error::GetBudgets {
                source_msg: format!("{:?}", e),
            })?
            .data
            .budgets
            .first()
            .ok_or_else(|| Error::GetBudgets {
                source_msg: "no budgets found".to_string(),
            })?
            .id
            .to_string();
        Ok(*self
            .rt
            .block_on(ynab_api::apis::budgets_api::get_budget_by_id(
                &self.configuration,
                &budget_id,
                None,
            ))
            .map_err(|e| Error::GetBudgetById {
                id: budget_id,
                source_msg: format!("{:?}", e),
            })?
            .data
            .budget)
    }

    pub fn update_transactions(
        &self,
        budget_id: &str,
        transactions: ynab_api::models::PatchTransactionsWrapper,
    ) -> Result<()> {
        self.rt
            .block_on(ynab_api::apis::transactions_api::update_transactions(
                &self.configuration,
                budget_id,
                transactions,
            ))
            .map(|_| ())
            .map_err(|e| Error::UpdateTransactions {
                source_msg: format!("{:?}", e),
            })
    }
}
