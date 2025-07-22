use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub description: String,
    pub amount: f64,
    pub ttype: TransactionType,
}

fn get_data_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".financeapp_transactions.json")
}

async fn save_transactions(transactions: &[Transaction]) {
    let path = get_data_path();
    if let Ok(json) = serde_json::to_string(transactions) {
        let _ = tokio::fs::write(path, json).await;
    }
}

async fn load_transactions() -> Vec<Transaction> {
    let path = get_data_path();
    if let Ok(data) = tokio::fs::read_to_string(path).await {
        if let Ok(txs) = serde_json::from_str(&data) {
            return txs;
        }
    }
    Vec::new()
}

#[component]
pub fn Finance() -> Element {
    let transactions = use_signal(|| Vec::<Transaction>::new());
    let description = use_signal(|| String::new());
    let amount = use_signal(|| String::new());
    let ttype = use_signal(|| TransactionType::Expense);
    let edit_index = use_signal(|| None as Option<usize>);

    // Load transactions on mount
    use_hook(|| {
        let transactions = transactions.clone();
        spawn(async move {
            let txs = load_transactions().await;
            transactions.set(txs);
        });
    });

    // Save transactions on change
    use_effect(move || {
        let txs = transactions.read().clone();
        spawn(async move {
            save_transactions(&txs).await;
        });
    });

    let add_or_update_transaction = move |_| {
        if let Ok(amt) = amount.read().parse::<f64>() {
            let mut txs = transactions.write();
            if let Some(idx) = *edit_index.read() {
                // Edit
                if let Some(tx) = txs.get_mut(idx) {
                    tx.description = description.read().clone();
                    tx.amount = amt;
                    tx.ttype = ttype.read().clone();
                }
                edit_index.set(None);
            } else {
                // Add
                txs.push(Transaction {
                    description: description.read().clone(),
                    amount: amt,
                    ttype: ttype.read().clone(),
                });
            }
            description.set(String::new());
            amount.set(String::new());
            ttype.set(TransactionType::Expense);
        }
    };

    let start_edit = move |idx: usize| {
        let txs = transactions.read();
        if let Some(tx) = txs.get(idx) {
            description.set(tx.description.clone());
            amount.set(tx.amount.to_string());
            ttype.set(tx.ttype.clone());
            edit_index.set(Some(idx));
        }
    };

    let delete_transaction = move |idx: usize| {
        let mut txs = transactions.write();
        if idx < txs.len() {
            txs.remove(idx);
        }
        // If editing this, cancel edit
        if edit_index.read() == &Some(idx) {
            edit_index.set(None);
            description.set(String::new());
            amount.set(String::new());
            ttype.set(TransactionType::Expense);
        }
    };

    let balance = transactions.read().iter().fold(0.0, |acc, tx| {
        match tx.ttype {
            TransactionType::Income => acc + tx.amount,
            TransactionType::Expense => acc - tx.amount,
        }
    });

    rsx! {
        div { class: "p-8 max-w-xl mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Finance Manager" }
            div { class: "mb-4 flex gap-2",
                input {
                    class: "border rounded px-2 py-1 w-1/2",
                    r#type: "text",
                    placeholder: "Description",
                    value: description,
                    oninput: move |evt| description.set(evt.value().clone()),
                }
                input {
                    class: "border rounded px-2 py-1 w-1/4",
                    r#type: "number",
                    placeholder: "Amount",
                    value: amount,
                    oninput: move |evt| amount.set(evt.value().clone()),
                }
                select {
                    class: "border rounded px-2 py-1",
                    value: match ttype.read() { TransactionType::Income => "income", TransactionType::Expense => "expense" },
                    oninput: move |evt| {
                        ttype.set(match evt.value().as_str() {
                            "income" => TransactionType::Income,
                            _ => TransactionType::Expense,
                        })
                    },
                    option { value: "income", "Income" }
                    option { value: "expense", "Expense" }
                }
                button {
                    class: "bg-blue-600 text-white px-4 py-1 rounded hover:bg-blue-700",
                    onclick: add_or_update_transaction,
                    if edit_index.read().is_some() { "Update" } else { "+ Add" }
                }
                if edit_index.read().is_some() {
                    button {
                        class: "bg-gray-400 text-white px-2 py-1 rounded hover:bg-gray-500",
                        onclick: move |_| {
                            edit_index.set(None);
                            description.set(String::new());
                            amount.set(String::new());
                            ttype.set(TransactionType::Expense);
                        },
                        "Cancel"
                    }
                }
            }
            div { class: "mb-4 text-lg font-semibold", "Current Balance: $"{format!("{:.2}", balance)} }
            table { class: "w-full border mt-4",
                thead { tr {
                    th { class: "border px-2 py-1", "Type" }
                    th { class: "border px-2 py-1", "Description" }
                    th { class: "border px-2 py-1", "Amount" }
                    th { class: "border px-2 py-1", "Actions" }
                }}
                tbody {
                    for (idx, tx) in transactions.read().iter().enumerate() {
                        tr {
                            td { class: "border px-2 py-1", match tx.ttype { TransactionType::Income => "Income", TransactionType::Expense => "Expense" } }
                            td { class: "border px-2 py-1", &tx.description }
                            td { class: "border px-2 py-1", format!("${:.2}", tx.amount) }
                            td { class: "border px-2 py-1 flex gap-1",
                                button {
                                    class: "bg-yellow-400 text-white px-2 py-1 rounded hover:bg-yellow-500",
                                    onclick: move |_| start_edit(idx),
                                    "Edit"
                                }
                                button {
                                    class: "bg-red-600 text-white px-2 py-1 rounded hover:bg-red-700",
                                    onclick: move |_| delete_transaction(idx),
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}