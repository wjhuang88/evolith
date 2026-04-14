# 套餐与计费系统设计

## 1. 概述

Evolith 作为 SaaS 平台，需要完整的套餐和计费系统来：
- 提供多个订阅计划
- 管理租户配额
- 处理账单和支付
- 提供使用量统计

## 2. 套餐定义

### 2.1 套餐表

```sql
CREATE TABLE plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 套餐信息
    name VARCHAR(64) NOT NULL,           -- free, starter, pro, enterprise
    display_name VARCHAR(128) NOT NULL, -- 免费版, 专业版, 企业版
    description TEXT,
    
    -- 定价
    monthly_price DECIMAL(10,2) DEFAULT 0,
    yearly_price DECIMAL(10,2),
    price_per_user DECIMAL(10,2),       -- 每个额外用户的价格
    
    -- 配额
    max_users INTEGER DEFAULT 3,
    max_tools INTEGER DEFAULT 5,
    max_skills INTEGER DEFAULT 10,
    max_snippets INTEGER DEFAULT 50,
    max_api_calls_per_month INTEGER DEFAULT 1000,
    max_storage_mb INTEGER DEFAULT 100,
    max_members INTEGER DEFAULT 3,
    
    -- 功能开关
    features JSONB DEFAULT '{}',
    
    -- 限制
    is_active BOOLEAN DEFAULT TRUE,
    is_builtin BOOLEAN DEFAULT TRUE,  -- 系统内置不可删除
    
    -- 排序
    sort_order INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 插入内置套餐
INSERT INTO plans (name, display_name, monthly_price, yearly_price, max_users, max_tools, max_skills, max_snippets, max_api_calls_per_month, features, sort_order) VALUES
('free', '免费版', 0, 0, 3, 5, 10, 50, 1000, '{"custom_domain": false, "sso": false, "priority_support": false, "audit_logs": false}', 1),
('starter', '基础版', 29, 290, 10, 20, 50, 200, 10000, '{"custom_domain": false, "sso": false, "priority_support": false, "audit_logs": true}', 2),
('pro', '专业版', 99, 990, 50, 100, 200, 1000, 100000, '{"custom_domain": true, "sso": false, "priority_support": true, "audit_logs": true}', 3),
('enterprise', '企业版', NULL, NULL, -1, -1, -1, -1, -1, '{"custom_domain": true, "sso": true, "priority_support": true, "audit_logs": true, "dedicated_support": true, "sla": "99.99%"}', 4);
```

### 2.2 套餐对比

| 功能 | Free | Starter | Pro | Enterprise |
|------|------|---------|-----|------------|
| 价格 | ¥0/月 | ¥29/月 | ¥99/月 | 定制 |
| 用户数 | 3 | 10 | 50 | 无限 |
| 工具数 | 5 | 20 | 100 | 无限 |
| 技能数 | 10 | 50 | 200 | 无限 |
| 代码片段 | 50 | 200 | 1000 | 无限 |
| API调用/月 | 1,000 | 10,000 | 100,000 | 无限 |
| 存储 | 100MB | 1GB | 10GB | 无限 |
| 自定义域名 | ❌ | ❌ | ✅ | ✅ |
| SSO/SAML | ❌ | ❌ | ❌ | ✅ |
| 审计日志 | ❌ | ✅ | ✅ | ✅ |
| 优先支持 | ❌ | ❌ | ✅ | ✅ |
| SLA | ❌ | ❌ | 99.9% | 99.99% |
| 专属支持 | ❌ | ❌ | ❌ | ✅ |

### 2.3 功能 JSON 结构

```json
{
  "custom_domain": true,
  "sso": false,
  "saml": false,
  "priority_support": true,
  "dedicated_support": false,
  "audit_logs": true,
  "api_access": true,
  "webhooks": true,
  "advanced_analytics": false,
  "sla": "99.9%",
  "max_file_size_mb": 10,
  "allowed_ip_whitelist": false
}
```

## 3. 订阅管理

### 3.1 订阅表

```sql
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 关联
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    plan_id UUID NOT NULL REFERENCES plans(id),
    
    -- 订阅状态
    status VARCHAR(32) DEFAULT 'trialing',  -- trialing, active, paused, canceled, past_due
    
    -- 期限
    billing_cycle VARCHAR(16) DEFAULT 'monthly',  -- monthly, yearly
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end TIMESTAMPTZ NOT NULL,
    
    -- 试用
    trial_start_at TIMESTAMPTZ,
    trial_end_at TIMESTAMPTZ,
    
    -- 取消
    canceled_at TIMESTAMPTZ,
    cancellation_reason TEXT,
    
    -- 支付
    payment_method_id TEXT,  -- 外部支付提供商 ID
    last_payment_at TIMESTAMPTZ,
    next_payment_at TIMESTAMPTZ,
    
    -- 时间戳
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_subscriptions_tenant ON subscriptions(tenant_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);
```

### 3.2 创建订阅

```typescript
// POST /api/v1/tenant/subscription
Request:
{
  "plan_id": "uuid",
  "billing_cycle": "monthly",
  "payment_method": {
    "type": "card",
    "token": "tok_xxx"  // Stripe token
  }
}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "status": "trialing",
    "plan": { "name": "pro", "display_name": "专业版" },
    "current_period_start": "2024-01-15T00:00:00Z",
    "current_period_end": "2024-02-15T00:00:00Z",
    "trial_end_at": "2024-01-22T00:00:00Z"
  }
}
```

### 3.3 升级/降级套餐

```typescript
// PATCH /api/v1/tenant/subscription
Request:
{
  "plan_id": "pro-plan-uuid",
  "proration": "immediate"  -- immediate | next_billing_cycle
}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "status": "active",
    "plan": { "name": "pro", "display_name": "专业版" },
    "updated_at": "2024-01-15T00:00:00Z"
  }
}
```

### 3.4 取消订阅

```typescript
// DELETE /api/v1/tenant/subscription
Request:
{
  "reason": "太贵了",
  "feedback": "希望有更便宜的选项"
}

Response:
{
  "success": true,
  "data": {
    "status": "canceled",
    "expires_at": "2024-02-15T00:00:00Z",
    "message": "您的订阅将在 2024-02-15 到期"
  }
}
```

## 4. 支付与发票

### 4.1 支付记录表

```sql
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 关联
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    subscription_id UUID REFERENCES subscriptions(id),
    
    -- 金额
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'CNY',
    
    -- 支付信息
    payment_method VARCHAR(32),  -- card, alipay, wechat
    provider_payment_id TEXT,     -- Stripe/支付宝 ID
    provider_response JSONB,     -- 原始响应
    
    -- 状态
    status VARCHAR(32) DEFAULT 'pending',  -- pending, succeeded, failed, refunded
    
    -- 发票
    invoice_id TEXT,
    invoice_url TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_tenant ON payments(tenant_id);
CREATE INDEX idx_payments_status ON payments(status);
```

### 4.2 发票表

```sql
CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 关联
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    payment_id UUID REFERENCES payments(id),
    
    -- 发票信息
    invoice_number VARCHAR(64) UNIQUE NOT NULL,
    status VARCHAR(32) DEFAULT 'draft',  -- draft, issued, paid, void
    
    -- 金额
    subtotal DECIMAL(10,2) NOT NULL,
    tax DECIMAL(10,2) DEFAULT 0,
    total DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'CNY',
    
    -- 周期
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    
    -- 税率
    tax_rate DECIMAL(5,2) DEFAULT 0,
    tax_number TEXT,  -- 税号
    
    -- 地址
    billing_address JSONB,
    
    -- PDF
    pdf_url TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    issued_at TIMESTAMPTZ,
    paid_at TIMESTAMPTZ
);

CREATE INDEX idx_invoices_tenant ON invoices(tenant_id);
CREATE INDEX idx_invoices_status ON invoices(status);
```

## 5. 使用量跟踪

### 5.1 使用量表

```sql
CREATE TABLE usage_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 关联
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    subscription_id UUID REFERENCES subscriptions(id),
    
    -- 资源类型
    resource_type VARCHAR(32) NOT NULL,  -- api_call, storage, user, tool, skill, snippet
    
    -- 使用量
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(10,4),           -- 超出配额后的单价
    overage_amount DECIMAL(10,2),        -- 超出费用
    
    -- 周期
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_usage_tenant_period ON usage_records(tenant_id, period_start);
CREATE INDEX idx_usage_type ON usage_records(resource_type);

-- 按天汇总的使用量
CREATE TABLE daily_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    resource_type VARCHAR(32) NOT NULL,
    
    date DATE NOT NULL,
    quantity INTEGER NOT NULL,
    
    UNIQUE(tenant_id, resource_type, date)
);

CREATE INDEX idx_daily_usage ON daily_usage(tenant_id, date);
```

### 5.2 实时使用量 API

```typescript
// GET /api/v1/tenant/usage
Response:
{
  "success": true,
  "data": {
    "period": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-01-31T23:59:59Z",
      "remaining_days": 15
    },
    "resources": [
      {
        "type": "api_calls",
        "used": 45000,
        "limit": 100000,
        "percent": 45,
        "overage": 0
      },
      {
        "type": "users",
        "used": 8,
        "limit": 10,
        "percent": 80,
        "can_add": 2
      },
      {
        "type": "tools",
        "used": 45,
        "limit": 100,
        "percent": 45,
        "can_add": 55
      },
      {
        "type": "storage_mb",
        "used": 2500,
        "limit": 10240,  -- 10GB
        "percent": 24
      }
    ]
  }
}
```

### 5.3 超配额处理

```rust
pub async fn check_quota(
    tenant_id: Uuid,
    resource_type: ResourceType,
) -> Result<QuotaCheck> {
    let tenant = get_tenant(tenant_id).await?;
    let plan = get_plan(tenant.plan_id).await?;
    let usage = get_current_usage(tenant_id, resource_type).await?;
    
    let limit = match resource_type {
        ResourceType::User => plan.max_users,
        ResourceType::Tool => plan.max_tools,
        ResourceType::Skill => plan.max_skills,
        ResourceType::Snippet => plan.max_snippets,
        ResourceType::ApiCall => plan.max_api_calls_per_month,
        ResourceType::Storage => plan.max_storage_mb,
    };
    
    if limit == -1 {
        return Ok(QuotaCheck { 
            allowed: true, 
            limit: None, 
            used, 
            remaining: None 
        });
    }
    
    let remaining = limit - usage;
    let allowed = remaining > 0;
    
    // 如果是 API 调用，超出后可以计费
    if !allowed && resource_type == ResourceType::ApiCall {
        // 允许超出，按量计费
        return Ok(QuotaCheck {
            allowed: true,
            limit: Some(limit),
            used,
            remaining: 0,
            overage_rate: 0.001,  // 每1000次1元
        });
    }
    
    Ok(QuotaCheck { 
        allowed, 
        limit: Some(limit), 
        used, 
        remaining: Some(remaining) 
    })
}
```

## 6. 前端实现

### 6.1 套餐选择页面

```typescript
// app/tenant/billing/plans/page.tsx
export default function PlansPage() {
  const { data: plans } = useQuery(['plans'], () => api.getPlans());
  const { data: current } = useQuery(['subscription'], () => api.getSubscription());
  
  return (
    <div className="plans-page">
      <h1>选择套餐</h1>
      <p>选择适合您团队的套餐</p>
      
      <div className="plans-grid">
        {plans?.map(plan => (
          <PlanCard 
            key={plan.id} 
            plan={plan}
            isCurrent={current?.plan_id === plan.id}
            onSelect={() => selectPlan(plan)}
          />
        ))}
      </div>
    </div>
  );
}
```

### 6.2 账单页面

```typescript
// app/tenant/billing/page.tsx
export default function BillingPage() {
  const { data: subscription } = useQuery(['subscription'], () => api.getSubscription());
  const { data: invoices } = useQuery(['invoices'], () => api.getInvoices());
  const { data: usage } = useQuery(['usage'], () => api.getUsage());
  
  return (
    <div className="billing-page">
      <h1>账单管理</h1>
      
      {/* 当前订阅 */}
      <Card>
        <CardHeader>
          <CardTitle>当前套餐</CardTitle>
        </CardHeader>
        <CardContent>
          <PlanInfo plan={subscription.plan} status={subscription.status} />
          <UsageBar 
            used={usage.api_calls.used} 
            limit={usage.api_calls.limit} 
          />
        </CardContent>
      </Card>
      
      {/* 发票列表 */}
      <Card>
        <CardHeader>
          <CardTitle>发票记录</CardTitle>
        </CardHeader>
        <CardContent>
          <table>
            <thead>
              <tr>
                <th>发票号</th>
                <th>金额</th>
                <th>状态</th>
                <th>日期</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {invoices?.map(inv => (
                <tr key={inv.id}>
                  <td>{inv.invoice_number}</td>
                  <td>¥{inv.total}</td>
                  <td><Badge>{inv.status}</Badge></td>
                  <td>{inv.created_at}</td>
                  <td>
                    <a href={inv.pdf_url} target="_blank">下载</a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </CardContent>
      </Card>
    </div>
  );
}
```

### 6.3 使用量仪表盘

```typescript
// app/tenant/usage/page.tsx
export default function UsagePage() {
  const { data } = useQuery(['usage'], () => api.getUsage());
  
  return (
    <div className="usage-page">
      <h1>使用量统计</h1>
      <p>本月使用情况 ({data.period.start} - {data.period.end})</p>
      
      <div className="usage-cards">
        {data.resources.map(res => (
          <UsageCard 
            key={res.type}
            type={res.type}
            used={res.used}
            limit={res.limit}
          />
        ))}
      </div>
    </div>
  );
}
```

## 7. 支付集成

### 7.1 Stripe 集成（示例）

```rust
// 支付服务
pub struct PaymentService {
    stripe_client: StripeClient,
}

impl PaymentService {
    // 创建 Stripe Customer
    pub async fn create_customer(&self, tenant: &Tenant) -> Result<String> {
        let customer = self.stripe_client.customers().create(
            CreateCustomer {
                email: &tenant.billing_email,
                metadata: serde_json::json!({
                    "tenant_id": tenant.id
                }),
                ..Default::default()
            }
        ).await?;
        
        Ok(customer.id)
    }
    
    // 创建订阅
    pub async fn create_subscription(
        &self,
        customer_id: &str,
        price_id: &str,
    ) -> Result<Subscription> {
        let subscription = self.stripe_client.subscriptions().create(
            CreateSubscription {
                customer: customer_id.into(),
                items: vec![PriceId {
                    price: price_id.into(),
                    quantity: 1,
                }],
                ..Default::default()
            }
        ).await?;
        
        Ok(subscription)
    }
}
```

## 8. Webhook 处理

```sql
CREATE TABLE webhook_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 事件信息
    event_type VARCHAR(64) NOT NULL,
    provider VARCHAR(32) NOT NULL,  -- stripe
    provider_event_id TEXT NOT NULL,
    payload JSONB NOT NULL,
    
    -- 处理状态
    status VARCHAR(32) DEFAULT 'pending',  -- pending, processed, failed
    attempts INTEGER DEFAULT 0,
    last_error TEXT,
    
    -- 时间戳
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- 事件类型
-- subscription.created
-- subscription.updated
-- subscription.deleted
-- invoice.paid
-- invoice.payment_failed
-- customer.subscription.trial_will_end
```

## 9. 相关文档

- [多租户设计](./multi-tenant.md)
- [用户与权限](./permissions.md)
- [API 合约](./api-contract.md)