use crate::sec_edgar::DebtMaturitySchedule;

pub struct UnifiedDcfData {
    pub revenue: Option<f64>,
    pub ebitda: Option<f64>,
    pub net_income: Option<f64>,
    pub free_cash_flow: Option<f64>,
    pub total_debt: Option<f64>,
    pub debt_maturities: Vec<DebtMaturityYear>,
}

pub struct DebtMaturityYear {
    pub year: i32,
    pub amount: f64,
}

pub fn merge_for_dcf(
    fundamentals: &tvdata_rs::equity::FinancialStatementsDetail,
    debt_schedule: Option<&DebtMaturitySchedule>,
) -> UnifiedDcfData {
    let total_debt = calculate_total_debt(fundamentals);

    UnifiedDcfData {
        revenue: fundamentals.revenue_fy,
        ebitda: fundamentals.ebitda_fy,
        net_income: fundamentals.net_income_fy,
        free_cash_flow: fundamentals.free_cash_flow_fy,
        total_debt,
        debt_maturities: debt_schedule
            .map(|d| {
                d.maturities
                    .iter()
                    .map(|m| DebtMaturityYear {
                        year: m.year,
                        amount: m.amount,
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn calculate_total_debt(
    fundamentals: &tvdata_rs::equity::FinancialStatementsDetail,
) -> Option<f64> {
    match (
        fundamentals.long_term_debt_fq,
        fundamentals.short_term_debt_fq,
    ) {
        (Some(ltd), Some(std)) => Some(ltd + std),
        (Some(ltd), None) => Some(ltd),
        (None, Some(std)) => Some(std),
        (None, None) => None,
    }
}

pub fn calculate_total_debt_from_schedule(debt_schedule: &DebtMaturitySchedule) -> f64 {
    debt_schedule.maturities.iter().map(|m| m.amount).sum()
}

pub fn has_debt_maturity_for_year(data: &UnifiedDcfData, year: i32) -> bool {
    data.debt_maturities.iter().any(|m| m.year == year)
}

pub fn get_debt_maturity_for_year(data: &UnifiedDcfData, year: i32) -> Option<f64> {
    data.debt_maturities
        .iter()
        .find(|m| m.year == year)
        .map(|m| m.amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_debt_schedule() -> DebtMaturitySchedule {
        DebtMaturitySchedule {
            maturities: vec![
                DebtMaturity {
                    year: 2024,
                    amount: 100_000_000.0,
                },
                DebtMaturity {
                    year: 2025,
                    amount: 150_000_000.0,
                },
                DebtMaturity {
                    year: 2026,
                    amount: 200_000_000.0,
                },
            ],
        }
    }

    #[test]
    fn test_calculate_total_debt_from_schedule() {
        let schedule = create_test_debt_schedule();
        let total = calculate_total_debt_from_schedule(&schedule);
        assert_eq!(total, 450_000_000.0);
    }

    #[test]
    fn test_has_debt_maturity_for_year() {
        let data = UnifiedDcfData {
            revenue: Some(1_000_000_000.0),
            ebitda: Some(200_000_000.0),
            net_income: Some(100_000_000.0),
            free_cash_flow: Some(150_000_000.0),
            total_debt: Some(500_000_000.0),
            debt_maturities: vec![
                DebtMaturityYear {
                    year: 2024,
                    amount: 100_000_000.0,
                },
                DebtMaturityYear {
                    year: 2025,
                    amount: 150_000_000.0,
                },
            ],
        };

        assert!(has_debt_maturity_for_year(&data, 2024));
        assert!(has_debt_maturity_for_year(&data, 2025));
        assert!(!has_debt_maturity_for_year(&data, 2026));
    }

    #[test]
    fn test_get_debt_maturity_for_year() {
        let data = UnifiedDcfData {
            revenue: Some(1_000_000_000.0),
            ebitda: Some(200_000_000.0),
            net_income: Some(100_000_000.0),
            free_cash_flow: Some(150_000_000.0),
            total_debt: Some(500_000_000.0),
            debt_maturities: vec![
                DebtMaturityYear {
                    year: 2024,
                    amount: 100_000_000.0,
                },
                DebtMaturityYear {
                    year: 2025,
                    amount: 150_000_000.0,
                },
            ],
        };

        assert_eq!(get_debt_maturity_for_year(&data, 2024), Some(100_000_000.0));
        assert_eq!(get_debt_maturity_for_year(&data, 2025), Some(150_000_000.0));
        assert_eq!(get_debt_maturity_for_year(&data, 2026), None);
    }
}
