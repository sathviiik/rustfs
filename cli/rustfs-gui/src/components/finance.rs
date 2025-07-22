use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Clone, PartialEq)]
pub struct Transaction {
    pub description: String,
    pub amount: f64,
    pub ttype: TransactionType,
}

#[component]
pub fn Finance() -> Element {
    let transactions = use_signal(|| Vec::<Transaction>::new());
    let description = use_signal(|| String::new());
    let amount = use_signal(|| String::new());
    let ttype = use_signal(|| TransactionType::Expense);

    let add_transaction = move |_| {
        if let Ok(amt) = amount.read().parse::<f64>() {
            let mut txs = transactions.write();
            txs.push(Transaction {
                description: description.read().clone(),
                amount: amt,
                ttype: ttype.read().clone(),
            });
            description.set(String::new());
            amount.set(String::new());
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
                    onclick: add_transaction,
                    "+ Add"
                }
            }
            div { class: "mb-4 text-lg font-semibold", "Current Balance: $"{format!("{:.2}", balance)} }
            table { class: "w-full border mt-4",
                thead { tr {
                    th { class: "border px-2 py-1", "Type" }
                    th { class: "border px-2 py-1", "Description" }
                    th { class: "border px-2 py-1", "Amount" }
                }}
                tbody {
                    for tx in transactions.read().iter() {
                        tr {
                            td { class: "border px-2 py-1", match tx.ttype { TransactionType::Income => "Income", TransactionType::Expense => "Expense" } }
                            td { class: "border px-2 py-1", &tx.description }
                            td { class: "border px-2 py-1", format!("${:.2}", tx.amount) }
                        }
                    }
                }
            }
        }
    }
}