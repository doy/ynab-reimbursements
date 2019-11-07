#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ynab_api::models::transaction_summary::Cleared,
    pub approved: bool,
    pub flag_color: Option<ynab_api::models::transaction_summary::FlagColor>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
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
        let reimbursed = if let Some(color) = &t.flag_color {
            color == &ynab_api::models::transaction_summary::FlagColor::Green
        } else {
            false
        };
        Self {
            id: t.id.clone(),
            date: t.date.clone(),
            amount: t.amount,
            memo: t.memo.clone(),
            cleared: clone_cleared(&t.cleared),
            approved: t.approved,
            flag_color: t.flag_color.as_ref().map(clone_flag_color),
            account_id: t.account_id.clone(),
            payee_id: t.payee_id.clone(),
            category_id: t.category_id.clone(),
            import_id: t.import_id.clone(),

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
        let reimbursed = if let Some(color) = &t.flag_color {
            color == &ynab_api::models::transaction_summary::FlagColor::Green
        } else {
            false
        };
        Self {
            id: t.id.clone(),
            date: t.date.clone(),
            amount: st.amount,
            memo: t.memo.clone(),
            cleared: clone_cleared(&t.cleared),
            approved: t.approved,
            flag_color: t.flag_color.as_ref().map(clone_flag_color),
            account_id: t.account_id.clone(),
            payee_id: t.payee_id.clone(),
            category_id: t.category_id.clone(),
            import_id: t.import_id.clone(),

            account: None,
            payee: None,
            total_amount: t.amount,
            reimbursed,
            selected: false,
        }
    }

    pub fn to_update_transaction(
        &self,
    ) -> ynab_api::models::UpdateTransaction {
        let mut ut = ynab_api::models::UpdateTransaction::new(
            self.id.clone(),
            self.account_id.clone(),
            self.date.clone(),
            self.amount,
        );
        ut.payee_id = self.payee_id.clone();
        ut.category_id = self.category_id.clone();
        ut.memo = self.memo.clone();
        ut.cleared = Some(cleared_to_cleared(&self.cleared));
        ut.approved = Some(self.approved);
        ut.flag_color =
            self.flag_color.as_ref().map(flag_color_to_flag_color);
        ut.import_id = self.import_id.clone();

        ut
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            date: self.date.clone(),
            amount: self.amount,
            memo: self.memo.clone(),
            cleared: clone_cleared(&self.cleared),
            approved: self.approved,
            flag_color: self.flag_color.as_ref().map(clone_flag_color),
            account_id: self.account_id.clone(),
            payee_id: self.payee_id.clone(),
            category_id: self.category_id.clone(),
            import_id: self.import_id.clone(),
            account: self.account.clone(),
            payee: self.payee.clone(),
            total_amount: self.total_amount,
            reimbursed: self.reimbursed,
            selected: self.selected,
        }
    }
}

fn cleared_to_cleared(
    cleared: &ynab_api::models::transaction_summary::Cleared,
) -> ynab_api::models::update_transaction::Cleared {
    use ynab_api::models::transaction_summary::Cleared as TSCleared;
    use ynab_api::models::update_transaction::Cleared as UTCleared;

    match cleared {
        TSCleared::Cleared => UTCleared::Cleared,
        TSCleared::Uncleared => UTCleared::Uncleared,
        TSCleared::Reconciled => UTCleared::Reconciled,
    }
}

fn flag_color_to_flag_color(
    flag_color: &ynab_api::models::transaction_summary::FlagColor,
) -> ynab_api::models::update_transaction::FlagColor {
    use ynab_api::models::transaction_summary::FlagColor as TSFlagColor;
    use ynab_api::models::update_transaction::FlagColor as UTFlagColor;

    match flag_color {
        TSFlagColor::Red => UTFlagColor::Red,
        TSFlagColor::Orange => UTFlagColor::Orange,
        TSFlagColor::Yellow => UTFlagColor::Yellow,
        TSFlagColor::Green => UTFlagColor::Green,
        TSFlagColor::Blue => UTFlagColor::Blue,
        TSFlagColor::Purple => UTFlagColor::Purple,
    }
}

fn clone_cleared(
    cleared: &ynab_api::models::transaction_summary::Cleared,
) -> ynab_api::models::transaction_summary::Cleared {
    use ynab_api::models::transaction_summary::Cleared as TSCleared;

    match cleared {
        TSCleared::Cleared => TSCleared::Cleared,
        TSCleared::Uncleared => TSCleared::Uncleared,
        TSCleared::Reconciled => TSCleared::Reconciled,
    }
}

fn clone_flag_color(
    flag_color: &ynab_api::models::transaction_summary::FlagColor,
) -> ynab_api::models::transaction_summary::FlagColor {
    use ynab_api::models::transaction_summary::FlagColor as TSFlagColor;

    match flag_color {
        TSFlagColor::Red => TSFlagColor::Red,
        TSFlagColor::Orange => TSFlagColor::Orange,
        TSFlagColor::Yellow => TSFlagColor::Yellow,
        TSFlagColor::Green => TSFlagColor::Green,
        TSFlagColor::Blue => TSFlagColor::Blue,
        TSFlagColor::Purple => TSFlagColor::Purple,
    }
}
