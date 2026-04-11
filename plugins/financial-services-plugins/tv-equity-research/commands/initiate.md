# /initiate

Initiate coverage on a new company using TradingView data.

## Usage

```
/initiate [ticker] [task]
```

## Examples

```
/initiate AAPL task1
/initiate MSFT task2
/initiate TSLA task3
```

## Description

Creates a comprehensive equity research initiation report (30-50 pages) through a 5-task workflow using TradingView MCP as the primary data source.

**Tasks:**
1. **Company Research** - Business, management, industry analysis
2. **Financial Modeling** - 6-tab Excel model using TradingView historicals
3. **Valuation Analysis** - DCF + comps using TradingView data
4. **Chart Generation** - 25-35 professional charts
5. **Report Assembly** - Final DOCX report

## Data Sources

**TradingView MCP:**
- `get_fundamentals` - Historical financials (5 years)
- `get_financial_statements` - Detailed IS/BS/CF
- `get_quote` - Market data, sector, industry
- `scan_stocks` - Peer identification
- `get_debt_maturity` - Debt structure
- `get_credit_ratings` - Credit ratings

**User Input Required:**
- Management bios
- Business strategy description
- Industry analysis
- Risk factors
- DCF assumptions (risk-free rate, ERP, terminal growth)

## Output

Task-specific deliverables:
- Task 1: `[Company]_Research_Document_[Date].md`
- Task 2: `[Company]_Financial_Model_[Date].xlsx`
- Task 3: Valuation tabs added to Excel + `[Company]_Valuation_Analysis_[Date].md`
- Task 4: `[Company]_Charts_[Date].zip`
- Task 5: `[Company]_Initiation_Report_[Date].docx` (30-50 pages)

## Important

Tasks must be executed ONE AT A TIME. Each task verifies prerequisites before proceeding.

## See Also

- `initiating-coverage` skill for detailed workflow
