use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::{
    GetEarningsCalendarParams, GetEarningsCalendarResponse,
    GetDividendCalendarParams, GetDividendCalendarResponse,
    EarningsEvent, DividendEvent,
};
use time::format_description::well_known::Rfc3339;

pub async fn handle_get_earnings_calendar(
    client: &TradingViewMcpClient,
    params: GetEarningsCalendarParams,
) -> Result<GetEarningsCalendarResponse, ClientError> {
    let events = client.get_earnings_calendar(params.days_ahead).await?;
    let earnings: Vec<EarningsEvent> = events
        .into_iter()
        .map(|e| EarningsEvent {
            symbol: e.symbol,
            date: e.date.format(&Rfc3339).unwrap_or_default(),
            eps_estimate: e.eps_estimate,
            revenue_estimate: None,
        })
        .collect();
    Ok(GetEarningsCalendarResponse { events: earnings })
}

pub async fn handle_get_dividend_calendar(
    client: &TradingViewMcpClient,
    params: GetDividendCalendarParams,
) -> Result<GetDividendCalendarResponse, ClientError> {
    let events = client.get_dividend_calendar(&params.exchange, params.days_ahead).await?;
    let dividends: Vec<DividendEvent> = events
        .into_iter()
        .map(|e| DividendEvent {
            symbol: e.symbol,
            date: e.date.format(&Rfc3339).unwrap_or_default(),
            dividend_amount: e.amount,
        })
        .collect();
    Ok(GetDividendCalendarResponse { events: dividends })
}
