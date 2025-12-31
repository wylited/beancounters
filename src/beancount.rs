use crate::model::{Transaction, Posting, Account, VerifyResult};
use anyhow::Result;
use beancount_parser_lima::{BeancountParser, BeancountSources, DirectiveVariant};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn list_transactions(data_dir: &Path) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();

    for entry in WalkDir::new(data_dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "bean") {
            if path.file_name().map_or(false, |n| n == "main.bean") {
                continue;
            }
            
            let txs = parse_file_transactions(path)?;
            transactions.extend(txs);
        }
    }
    
    transactions.sort_by(|a, b| b.date.cmp(&a.date));
    
    Ok(transactions)
}

fn parse_file_transactions(path: &Path) -> Result<Vec<Transaction>> {
    let sources = BeancountSources::try_from(path.to_path_buf())
        .map_err(|e| anyhow::anyhow!("Failed to load sources: {}", e))?;
    let parser = BeancountParser::new(&sources);
    let result = parser.parse().map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

    let mut transactions = Vec::new();
    let path_str = path.to_string_lossy().to_string();

    for directive in result.directives {
        if let DirectiveVariant::Transaction(t) = directive.variant() {
            let date = directive.date().item().to_string();
            let flag = t.flag().to_string();
            let payee = t.payee().map(|p| p.item().to_string());
            let narration = t.narration().map(|n| n.item().to_string());
            
            let mut postings = Vec::new();
            for p in t.postings() {
                postings.push(Posting {
                    account: p.account().item().to_string(),
                    amount: p.amount().map(|a| a.item().to_string()).unwrap_or_default(),
                    currency: p.currency().map(|c| c.item().to_string()).unwrap_or_default(),
                    cost: None, 
                    price: None, 
                });
            }

            let start = directive.date().span().start;
            let id = format!("{}:{}", path_str, start);

            transactions.push(Transaction {
                id: Some(id),
                date,
                flag,
                payee,
                narration,
                tags: vec![], 
                postings,
            });
        }
    }
    
    Ok(transactions)
}

pub fn list_accounts(data_dir: &Path) -> Result<Vec<Account>> {
    let path = data_dir.join("accounts.bean");
    if !path.exists() {
        return Ok(vec![]);
    }
    
    let sources = BeancountSources::try_from(path)
        .map_err(|e| anyhow::anyhow!("Failed to load sources: {}", e))?;
    let parser = BeancountParser::new(&sources);
    let result = parser.parse().map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

    let mut accounts = Vec::new();
    
    for directive in result.directives {
        if let DirectiveVariant::Open(o) = directive.variant() {
            accounts.push(Account {
                name: o.account().item().to_string(),
                open_date: directive.date().item().to_string(),
                currencies: o.currencies().map(|c| c.item().to_string()).collect(),
                close_date: None,
            });
        }
    }
    
    Ok(accounts)
}

pub fn verify(data_dir: &Path) -> Result<VerifyResult> {
    let path = data_dir.join("main.bean");
    let sources = BeancountSources::try_from(path)
        .map_err(|e| anyhow::anyhow!("Failed to load sources: {}", e))?;
    let parser = BeancountParser::new(&sources);
    
    let (errors, warnings) = match parser.parse() {
        Ok(success) => (vec![], success.warnings),
        Err(error) => (error.errors, error.warnings),
    };
    
    let temp_path = std::env::temp_dir().join("beancount_verify.log");
    let file = fs::File::create(&temp_path)?;
    sources.write_errors_or_warnings(&file, errors).unwrap();
    let errors_str = fs::read_to_string(&temp_path)?;
    
    let file = fs::File::create(&temp_path)?;
    sources.write_errors_or_warnings(&file, warnings).unwrap();
    let warnings_str = fs::read_to_string(&temp_path)?;
    
    fs::remove_file(temp_path).ok();
    
    Ok(VerifyResult {
        errors: errors_str.lines().map(|s| s.to_string()).collect(),
        warnings: warnings_str.lines().map(|s| s.to_string()).collect(),
    })
}

pub fn add_transaction(data_dir: &Path, tx: Transaction) -> Result<()> {
    let date = chrono::NaiveDate::parse_from_str(&tx.date, "%Y-%m-%d")?;
    let filename = format!("{}-{:02}.bean", date.format("%Y"), date.format("%m"));
    let path = data_dir.join(&filename);
    
    let mut text = format!("\n{} {} \"{}\" \"{}\"\n", tx.date, tx.flag, tx.payee.unwrap_or_default(), tx.narration.unwrap_or_default());
    for p in tx.postings {
        text.push_str(&format!("  {} {} {}\n", p.account, p.amount, p.currency));
    }
    
    use std::io::Write;
    let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
    file.write_all(text.as_bytes())?;
    
    let main_path = data_dir.join("main.bean");
    let main_content = fs::read_to_string(&main_path).unwrap_or_default();
    let include_line = format!("include \"{}\"", filename);
    if !main_content.contains(&include_line) {
        let mut main_file = fs::OpenOptions::new().create(true).append(true).open(&main_path)?;
        main_file.write_all(format!("\n{}\n", include_line).as_bytes())?;
    }
    
    Ok(())
}

pub fn delete_transaction(id: &str) -> Result<()> {
    let parts: Vec<&str> = id.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid ID"));
    }
    let path_str = parts[0];
    let start_byte: usize = parts[1].parse()?;
    
    let path = PathBuf::from(path_str);
    let content = fs::read_to_string(&path)?;
    
    let sources = BeancountSources::try_from(path.clone())
        .map_err(|e| anyhow::anyhow!("Failed to load sources: {}", e))?;
    let parser = BeancountParser::new(&sources);
    let result = parser.parse().map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;
    
    let mut target_span = None;
    
    for directive in result.directives {
        if let DirectiveVariant::Transaction(t) = directive.variant() {
             if directive.date().span().start == start_byte {
                 let mut end = directive.date().span().end;
                 if let Some(n) = t.narration() {
                     end = std::cmp::max(end, n.span().end);
                 }
                 for p in t.postings() {
                     end = std::cmp::max(end, p.account().span().end);
                     if let Some(a) = p.amount() {
                         end = std::cmp::max(end, a.span().end);
                     }
                 }
                 
                 target_span = Some(start_byte..end);
                 break;
             }
        }
    }
    
    if let Some(span) = target_span {
        let mut end = span.end;
        while end < content.len() && &content[end..end+1] != "\n" {
            end += 1;
        }
        if end < content.len() && &content[end..end+1] == "\n" {
            end += 1;
        }
        
        let new_content = format!("{}{}", &content[..start_byte], &content[end..]);
        fs::write(&path, new_content)?;
    } else {
        return Err(anyhow::anyhow!("Transaction not found"));
    }
    
    Ok(())
}

pub fn update_transaction(id: &str, tx: Transaction) -> Result<()> {
    delete_transaction(id)?;
    add_transaction(Path::new("data"), tx)?; 
    Ok(())
}

pub fn add_account(data_dir: &Path, account: Account) -> Result<()> {
    let path = data_dir.join("accounts.bean");
    let text = format!("{} open {} {}\n", account.open_date, account.name, account.currencies.join(","));
    use std::io::Write;
    let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn delete_account(data_dir: &Path, name: &str) -> Result<()> {
    let path = data_dir.join("accounts.bean");
    let content = fs::read_to_string(&path)?;
    
    let lines: Vec<&str> = content.lines().filter(|l| !l.contains(&format!("open {}", name))).collect();
    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

pub fn update_account(data_dir: &Path, name: &str, account: Account) -> Result<()> {
    delete_account(data_dir, name)?;
    add_account(data_dir, account)?;
    Ok(())
}
