# tvdata-service 使用手册

基于 Rust 构建的 TradingView 数据监控与告警服务。

## 功能特性

- **实时价格监控**：追踪价格变动，根据阈值触发告警
- **技术指标监控**：支持 RSI、MACD 指标监控
- **日历事件监控**：关注财报、分红、IPO 等事件
- **飞书告警**：通过飞书 Webhook 发送富文本告警卡片
- **REST API**：完整的监控、规则、告警 CRUD 操作
- **SQLite 存储**：零依赖本地数据库
- **代理支持**：自动支持 HTTP_PROXY/HTTPS_PROXY 环境变量
- **中文股票支持**：支持 SSE（上海）、HKEX（香港）等中文交易所

## 快速开始

### 环境要求

| 组件 | 版本要求 |
|------|---------|
| Rust | 1.85+ |
| SQLite | 3.x |

### 启动服务

```bash
# 进入服务目录
cd ~/tvservice

# 启动服务（使用代理）
export http_proxy=http://127.0.0.1:10809
export https_proxy=http://127.0.0.1:10809
./tvdata-service
```

服务默认监听 `http://localhost:8080`

### 配置说明

编辑 `config.yaml` 配置文件：

```yaml
server:
  host: "0.0.0.0"    # 绑定地址
  port: 8080          # 端口

database:
  path: "./data/tvmonitor.db"  # 数据库路径

monitor:
  check_interval_secs: 60      # 检查间隔（秒）
  max_concurrent_checks: 10    # 最大并发检查数
  cooldown_secs: 300            # 系统级冷却时间
  max_monitors: 500            # 最大监控数量

alert:
  rate_limit_per_hour: 10     # 告警速率限制
  webhook:
    url: "https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-id"
    msg_type: "interactive"

search:
  language: "cn"              # 搜索语言: cn(中文), en(英文)
```

### 代理配置

服务会自动读取以下环境变量：

```bash
export HTTP_PROXY=http://127.0.0.1:10809
export HTTPS_PROXY=http://127.0.0.1:10809
```

---

## API 接口文档

### 健康检查

```bash
GET /health
```

响应：
```json
{"status": "ok"}
```

---

### 实时报价

获取股票的实时价格数据：

```bash
GET /api/v1/quotes/:symbol
```

**参数：**
- `symbol` - TradingView 股票代码，格式：`交易所:代码`

**示例：**
```bash
# 苹果股票
curl http://localhost:8080/api/v1/quotes/NASDAQ:AAPL

# 英伟达股票
curl http://localhost:8080/api/v1/quotes/NASDAQ:NVDA

# 外汇
curl http://localhost:8080/api/v1/quotes/FOREX:EURUSD

# 加密货币
curl http://localhost:8080/api/v1/quotes/CRYPTO:BTCUSD
```

**响应：**
```json
{
  "symbol": "NASDAQ:AAPL",
  "price": 252.86,
  "change": 1.37,
  "change_percent": 0.54,
  "volume": 45152288,
  "high": 254.82,
  "low": 249.55,
  "open": 250.35,
  "previous_close": 251.49
}
```

---

### 股票搜索

搜索 TradingView 支持的股票代码（支持中文搜索）：

```bash
GET /api/v1/search?q=关键词&type=类型
```

**参数：**
- `q` - 搜索关键词（必填），支持中文
- `type` - 资产类型：`equity`（股票）、`forex`（外汇）、`crypto`（加密货币）

**搜索语言：**
- 默认语言为中文 (`cn`)，可通过 `config.yaml` 配置
- 搜索"苹果"返回中文描述的苹果公司
- 搜索"apple"返回英文描述的结果

**示例：**
```bash
# 中文搜索（返回中文描述）
curl "http://localhost:8080/api/v1/search?q=苹果"
curl "http://localhost:8080/api/v1/search?q=阿里"

# 英文搜索
curl "http://localhost:8080/api/v1/search?q=apple"

# 搜索加密货币
curl "http://localhost:8080/api/v1/search?q=btc&type=crypto"

# 搜索外汇
curl "http://localhost:8080/api/v1/search?q=eur&type=forex"
```

**响应：**
```json
[
  {
    "symbol": "NASDAQ:AAPL",
    "description": "Apple Inc",
    "exchange": "NASDAQ",
    "instrument_type": "stock"
  }
]
```

---

### 历史数据查询

> **注意**：无需手动刷新历史数据。创建监控时会自动获取全部历史数据，每日自动更新。

查询本地数据库中存储的历史 K 线数据：

```bash
GET /api/v1/history/:symbol?from=开始日期&to=结束日期
```

**参数：**
- `symbol` - TradingView 股票代码（必填）
- `from` - 开始日期，格式：`YYYY-MM-DD`（必填）
- `to` - 结束日期，格式：`YYYY-MM-DD`（必填）

**示例：**
```bash
# 查询 2024 年数据
curl "http://localhost:8080/api/v1/history/NASDAQ:AAPL?from=2024-01-01&to=2024-12-31"

# 查询 2025 年数据
curl "http://localhost:8080/api/v1/history/NASDAQ:AAPL?from=2025-01-01&to=2025-12-31"

# 查询所有可用数据
curl "http://localhost:8080/api/v1/history/NASDAQ:AAPL?from=1980-01-01&to=2030-12-31"
```

**响应：**
```json
[
  {
    "timestamp": 1734964200,
    "open": 254.77,
    "high": 255.65,
    "low": 253.45,
    "close": 255.27,
    "volume": 40858774
  },
  {
    "timestamp": 1735050600,
    "open": 255.49,
    "high": 258.21,
    "low": 255.29,
    "close": 258.20,
    "volume": 23234705
  }
]
```

**返回数据说明：**
| 字段 | 说明 |
|------|------|
| timestamp | Unix 时间戳（秒） |
| open | 开盘价 |
| high | 最高价 |
| low | 最低价 |
| close | 收盘价 |
| volume | 成交量 |

---

**说明：**
- 创建监控时会自动获取并存储全部历史数据
- 每日凌晨自动刷新最新数据
- 后续查询直接从本地数据库返回，速度更快
- 使用 `INSERT OR IGNORE` 增量更新，不会重复插入

**数据范围：**
- AAPL：1980年12月至今（约550+条日线数据）
- 其他股票：数据范围可能不同

---

### 监控管理

#### 创建监控

添加一个需要监控的股票（会自动获取历史数据）：

```bash
POST /api/v1/monitors
Content-Type: application/json

{
  "symbol": "NASDAQ:AAPL",
  "name": "苹果公司"
}
```

**说明：**
- 创建时会同步验证 symbol 是否有效
- 自动获取并存储全部历史数据（创建完成后即可查询）
- 相同 symbol 只能创建一次（防止重复监控）



**错误响应：**
- 无效 symbol：`{"error": "symbol NASDAQ:FAKE has no data on TradingView"}`
- 重复 symbol：`{"error": "monitor for symbol NASDAQ:AAPL already exists"}`



#### 获取监控列表

```bash
GET /api/v1/monitors
```

#### 获取单个监控

```bash
GET /api/v1/monitors/:id
```

#### 更新监控

```bash
PUT /api/v1/monitors/:id
Content-Type: application/json

{
  "name": "新名称",
  "enabled": false
}
```

#### 删除监控

```bash
DELETE /api/v1/monitors/:id
```

---

### 告警规则

#### 创建规则

```bash
POST /api/v1/rules
Content-Type: application/json

{
  "monitor_id": "监控ID",
  "rule_type": "price",
  "name": "价格告警",
  "condition": {
    "op": ">",
    "threshold": 5.0
  },
  "severity": "warning",
  "cooldown_secs": 300
}
```

**规则类型：**

| 类型 | 说明 | 条件字段 |
|------|------|---------|
| `price` | 价格变动监控 | `op`：比较运算符，`threshold`：百分比 |
| `indicator` | 技术指标监控 | `indicator`：RSI/MACD，`op`，`threshold`，`period` |
| `calendar` | 日历事件监控 | `calendar_type`：earnings/dividends/ipo |

**比较运算符：**
- `>`、`>=`、`<`、`<=`、`==`、`!=`

**告警级别：**
- `info` - 信息
- `warning` - 警告
- `critical` - 严重

**示例 - RSI 超买告警：**
```json
{
  "monitor_id": "uuid-xxxx",
  "rule_type": "indicator",
  "name": "RSI 超买",
  "condition": {
    "indicator": "RSI",
    "op": ">",
    "threshold": 70,
    "period": 14
  },
  "severity": "warning",
  "cooldown_secs": 300
}
```

**示例 - MACD 金叉告警：**
```json
{
  "monitor_id": "uuid-xxxx",
  "rule_type": "indicator",
  "name": "MACD 金叉",
  "condition": {
    "indicator": "MACD",
    "op": ">",
    "threshold": 0,
    "fast": 12,
    "slow": 26,
    "signal": 9
  },
  "severity": "info",
  "cooldown_secs": 300
}
```

#### 获取规则列表

```bash
GET /api/v1/rules
```

#### 删除规则

```bash
DELETE /api/v1/rules/:id
```

---

### 告警管理

#### 获取告警列表

```bash
GET /api/v1/alerts?page=1&page_size=20&symbol=NASDAQ:AAPL&acknowledged=false
```

**查询参数：**
| 参数 | 类型 | 说明 |
|------|------|------|
| page | int | 页码（默认1） |
| page_size | int | 每页数量（默认20） |
| symbol | string | 按股票代码筛选 |
| acknowledged | bool | 按已确认状态筛选 |

**响应：**
```json
{
  "data": [
    {
      "id": "uuid-xxxx",
      "rule_id": "uuid-xxxx",
      "symbol": "NASDAQ:AAPL",
      "message": "NASDAQ:AAPL: 涨幅 5.50% (阈值: 5.0 >)",
      "severity": "warning",
      "triggered_at": "2024-01-01T00:00:00Z",
      "acknowledged": false,
      "acknowledged_at": null,
      "ack_by": null
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

#### 确认告警

```bash
POST /api/v1/alerts/:id/ack
Content-Type: application/json

{
  "ack_by": "管理员"
}
```

---

### 优雅关闭

平稳停止服务，等待进行中的请求完成：

```bash
POST /shutdown
```

---

## 使用示例

### 完整告警流程

**步骤 1：创建监控**
```bash
curl -X POST http://localhost:8080/api/v1/monitors \
  -H "Content-Type: application/json" \
  -d '{"symbol": "NASDAQ:AAPL", "name": "苹果公司"}'
```

返回的 ID：`abc-123`

**步骤 2：创建价格告警规则**
```bash
curl -X POST http://localhost:8080/api/v1/rules \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": "abc-123",
    "rule_type": "price",
    "name": "涨幅告警",
    "condition": {"op": ">", "threshold": 5.0},
    "severity": "warning",
    "cooldown_secs": 300
  }'
```

**步骤 3：查看告警**
```bash
curl "http://localhost:8080/api/v1/alerts?acknowledged=false"
```

**步骤 4：确认告警**
```bash
curl -X POST http://localhost:8080/api/v1/alerts/alert-uuid/ack \
  -H "Content-Type: application/json" \
  -d '{"ack_by": "admin"}'
```

### 查询历史数据

```bash
# 1. 创建监控时自动加载历史数据
curl -X POST http://localhost:8080/api/v1/monitors -d '{"symbol":"NASDAQ:AAPL","name":"苹果"}'

# 2. 查询 2024 年数据
curl "http://localhost:8080/api/v1/history/NASDAQ:AAPL?from=2024-01-01&to=2024-12-31" | jq 'length'

# 3. 查看数据详情
curl "http://localhost:8080/api/v1/history/NASDAQ:AAPL?from=2024-01-01&to=2024-12-31" | jq '.[-5:]'
```

---

## 股票代码格式

TradingView 使用 `交易所:代码` 格式：

| 类型 | 示例 |
|------|------|
| 股票 - 纳斯达克 | `NASDAQ:AAPL`, `NASDAQ:NVDA` |
| 股票 - 纽约证交所 | `NYSE:TSLA`, `NYSE:MSFT` |
| 外汇 | `FOREX:EURUSD`, `FOREX:GBPUSD` |
| 加密货币 | `CRYPTO:BTCUSD`, `CRYPTO:ETHUSD` |

---

## 故障排除

### 服务无法启动

```bash
# 检查端口是否被占用
lsof -i :8080

# 查看详细日志
cd ~/tvservice && RUST_LOG=debug ./tvdata-service 2>&1
```

### 代理不生效

```bash
# 确认环境变量已设置
echo $http_proxy
echo $https_proxy
```

### 数据库问题

```bash
# 删除数据库重新初始化
rm ~/tvservice/data/tvmonitor.db
cd ~/tvservice && ./tvdata-service
```

---

## 项目结构

```
tvdata-service/
├── src/
│   ├── main.rs           # 入口点
│   ├── lib.rs           # 库导出
│   ├── config.rs        # 配置管理
│   ├── db.rs            # 数据库操作
│   ├── error.rs         # 错误类型
│   ├── models.rs        # 数据模型
│   ├── tvclient.rs      # TradingView 客户端
│   └── api/             # REST API 处理器
│       ├── monitors.rs  # 监控 CRUD
│       ├── rules.rs     # 规则 CRUD
│       ├── alerts.rs    # 告警管理
│       ├── quotes.rs   # 报价获取
│       └── history.rs   # 历史数据
├── config.yaml           # 配置文件
└── Cargo.toml           # 依赖
```

---

## 开发指南

```bash
# 格式化代码
cargo fmt --all

# 运行测试
cargo test

# 代码检查
cargo clippy --all-targets -- -D warnings

# 构建发布版本
cargo build --release
```
