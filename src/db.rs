use postgres::{Client, Error, NoTls};

#[derive(Debug)]
pub struct CompanyInfoRow {
    pub name: String,
    pub symbol: String,
    pub url: String
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
    }
    Ok(query_rows)
}