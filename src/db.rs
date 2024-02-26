use postgres::{Client, Error, NoTls};

pub struct CompanyInfoRow {
    name: String,
    symbol: String,
    url: String
}

pub fn query_db() -> Result<Vec<CompanyInfoRow>, Error> {
    let mut query_rows: Vec<CompanyInfoRow> = Vec::new();

    let mut client = Client::connect("host=winhost user=postgres password=postgres", NoTls)?;

    for row in client.query("SELECT company_name, stock_symbol, ir_url FROM company_info", &[])? {
        let name: String = String::from(row.get::<usize, &str>(0));
        let symbol: String = String::from(row.get::<usize, &str>(1));
        let url: String = String::from(row.get::<usize, &str>(2));


        let company_info_row = CompanyInfoRow {name, symbol, url};
        query_rows.push(company_info_row);
        //println!("Company number:{} Symbol:{} Name:{}", &url, &symbol, &name);
    }
    Ok(query_rows)
}