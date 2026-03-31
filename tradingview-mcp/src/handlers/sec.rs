use crate::sec_edgar::SecEdgarClient;
use crate::tools::{DebtMaturity, GetDebtMaturityParams, GetDebtMaturityResponse};

pub async fn handle_get_debt_maturity(
    sec_client: &SecEdgarClient,
    params: GetDebtMaturityParams,
) -> Result<GetDebtMaturityResponse, Box<dyn std::error::Error>> {
    let schedule = sec_client.get_debt_maturity(&params.symbol).await?;
    let maturities = schedule
        .map(|s| {
            s.maturities
                .into_iter()
                .map(|m| DebtMaturity {
                    year: m.year,
                    amount: m.amount,
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(GetDebtMaturityResponse {
        symbol: params.symbol,
        maturities,
    })
}
