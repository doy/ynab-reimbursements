#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ynab_api::models::TransactionClearedStatus,
    pub approved: bool,
    pub flag_color: Option<ynab_api::models::TransactionFlagColor>,
    pub account_id: uuid::Uuid,
    pub payee_id: Option<uuid::Uuid>,
    pub category_id: Option<uuid::Uuid>,
    pub import_id: Option<String>,

    pub account: Option<String>,
    pub payee: Option<String>,
    pub total_amount: i64,
    pub reimbursed: bool,
    pub selected: bool,
}

impl Transaction {
    pub fn from_transaction(
        t: &ynab_api::models::TransactionSummary,
    ) -> Self {
        let reimbursed = if let Some(Some(color)) = &t.flag_color {
            color == &ynab_api::models::TransactionFlagColor::Green
        } else {
            false
        };
        Self {
            id: t.id.clone(),
            date: t.date.clone(),
            amount: t.amount,
            memo: t.memo.clone().flatten(),
            cleared: t.cleared,
            approved: t.approved,
            flag_color: t.flag_color.flatten(),
            account_id: t.account_id,
            payee_id: t.payee_id.flatten(),
            category_id: t.category_id.flatten(),
            import_id: t.import_id.clone().flatten(),

            account: None,
            payee: None,
            total_amount: t.amount,
            reimbursed,
            selected: false,
        }
    }

    pub fn from_sub_transaction(
        t: &ynab_api::models::TransactionSummary,
        st: &ynab_api::models::SubTransaction,
    ) -> Self {
        let reimbursed = if let Some(Some(color)) = &t.flag_color {
            color == &ynab_api::models::TransactionFlagColor::Green
        } else {
            false
        };
        Self {
            id: t.id.clone(),
            date: t.date.clone(),
            amount: st.amount,
            memo: t.memo.clone().flatten(),
            cleared: t.cleared,
            approved: t.approved,
            flag_color: t.flag_color.flatten(),
            account_id: t.account_id,
            payee_id: t.payee_id.flatten(),
            category_id: t.category_id.flatten(),
            import_id: t.import_id.clone().flatten(),
            account: None,
            payee: None,
            total_amount: t.amount,
            reimbursed,
            selected: false,
        }
    }

    pub fn to_save_transaction(
        &self,
    ) -> ynab_api::models::SaveTransactionWithIdOrImportId {
        let mut st = ynab_api::models::SaveTransactionWithIdOrImportId::new();
        st.id = Some(Some(self.id.clone()));
        st.account_id = Some(self.account_id);
        st.date = Some(self.date.clone());
        st.amount = Some(self.amount);
        st.payee_id = Some(self.payee_id);
        st.category_id = Some(self.category_id);
        st.memo = Some(self.memo.clone());
        st.cleared = Some(self.cleared);
        st.approved = Some(self.approved);
        st.flag_color = Some(self.flag_color);
        st.import_id = Some(self.import_id.clone());

        st
    }
}
