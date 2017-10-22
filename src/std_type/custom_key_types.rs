use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
    username: &'a str, password: &'a str) {
    println!("user : {}, pass : {}", username, password);

    let login = Account {
        username:username,
        password: password,
    };

    match accounts.get(&login) {
        Some(account_info) => {
            println!("success, name : {}, email : {}", account_info.name, account_info.email);
        },
        _ => println!("failure"),
    }
}

pub fn test() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.every@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "password123");

    try_logon(&accounts, "j.everyman", "password1233");
}
