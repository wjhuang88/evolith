-- Payment integration migration
-- Adds Stripe payment support and overage billing.
-- PostgreSQL-native version. Plan identifiers intentionally remain stable text
-- values (for example, plan-free), matching SQLite and the public billing API.

ALTER TABLE tenants ADD COLUMN stripe_customer_id TEXT;
ALTER TABLE tenants ADD COLUMN stripe_subscription_id TEXT;

CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plan_id TEXT NOT NULL,
    stripe_subscription_id TEXT,
    stripe_customer_id TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    billing_cycle TEXT DEFAULT 'monthly',
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end TIMESTAMPTZ NOT NULL,
    trial_start_at TIMESTAMPTZ,
    trial_end_at TIMESTAMPTZ,
    canceled_at TIMESTAMPTZ,
    cancel_at_period_end BOOLEAN DEFAULT FALSE,
    cancellation_reason TEXT,
    payment_method_id TEXT,
    last_payment_at TIMESTAMPTZ,
    next_payment_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_subscriptions_tenant ON subscriptions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_subscriptions_stripe_id ON subscriptions(stripe_subscription_id);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id UUID REFERENCES subscriptions(id),
    amount NUMERIC(12,2) NOT NULL,
    currency TEXT DEFAULT 'CNY',
    payment_method TEXT,
    provider_payment_id TEXT,
    provider_response JSONB DEFAULT '{}'::jsonb,
    status TEXT DEFAULT 'pending',
    invoice_id TEXT,
    invoice_url TEXT,
    stripe_payment_intent_id TEXT,
    stripe_charge_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_payments_tenant ON payments(tenant_id);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_payments_stripe_id ON payments(stripe_payment_intent_id);

CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    payment_id UUID REFERENCES payments(id),
    subscription_id UUID REFERENCES subscriptions(id),
    invoice_number TEXT UNIQUE NOT NULL,
    status TEXT DEFAULT 'draft',
    subtotal NUMERIC(12,2) NOT NULL,
    tax NUMERIC(12,2) DEFAULT 0,
    total NUMERIC(12,2) NOT NULL,
    currency TEXT DEFAULT 'CNY',
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    tax_rate NUMERIC(5,4) DEFAULT 0,
    tax_number TEXT,
    billing_address JSONB DEFAULT '{}'::jsonb,
    pdf_url TEXT,
    stripe_invoice_id TEXT,
    invoice_type TEXT DEFAULT 'subscription',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    issued_at TIMESTAMPTZ,
    paid_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_invoices_tenant ON invoices(tenant_id);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoices_number ON invoices(invoice_number);
CREATE INDEX IF NOT EXISTS idx_invoices_stripe_id ON invoices(stripe_invoice_id);

CREATE TABLE IF NOT EXISTS usage_records (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id UUID REFERENCES subscriptions(id),
    resource_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price NUMERIC(12,2),
    overage_units INTEGER DEFAULT 0,
    overage_amount NUMERIC(12,2) DEFAULT 0,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    billed BOOLEAN DEFAULT FALSE,
    invoice_id UUID REFERENCES invoices(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_usage_tenant_period ON usage_records(tenant_id, period_start);
CREATE INDEX IF NOT EXISTS idx_usage_type ON usage_records(resource_type);
CREATE INDEX IF NOT EXISTS idx_usage_billed ON usage_records(billed);

CREATE TABLE IF NOT EXISTS daily_usage (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL,
    date TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    UNIQUE(tenant_id, resource_type, date)
);
CREATE INDEX IF NOT EXISTS idx_daily_usage_tenant_date ON daily_usage(tenant_id, date);

CREATE TABLE IF NOT EXISTS webhook_events (
    id UUID PRIMARY KEY,
    event_type TEXT NOT NULL,
    provider TEXT NOT NULL DEFAULT 'stripe',
    provider_event_id TEXT NOT NULL UNIQUE,
    payload JSONB NOT NULL,
    status TEXT DEFAULT 'pending',
    attempts INTEGER DEFAULT 0,
    last_error TEXT,
    tenant_id UUID,
    action_taken TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_webhook_status ON webhook_events(status);
CREATE INDEX IF NOT EXISTS idx_webhook_provider_id ON webhook_events(provider_event_id);

CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    description TEXT,
    monthly_price NUMERIC(12,2) DEFAULT 0,
    yearly_price NUMERIC(12,2),
    price_per_user NUMERIC(12,2),
    max_users INTEGER DEFAULT 3,
    max_tools INTEGER DEFAULT 5,
    max_skills INTEGER DEFAULT 10,
    max_snippets INTEGER DEFAULT 50,
    max_api_calls_per_month INTEGER DEFAULT 1000,
    max_storage_mb INTEGER DEFAULT 100,
    features JSONB DEFAULT '{}'::jsonb,
    stripe_monthly_price_id TEXT,
    stripe_yearly_price_id TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    is_builtin BOOLEAN DEFAULT TRUE,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO plans (
    id, name, display_name, description, monthly_price, yearly_price,
    max_users, max_tools, max_skills, max_snippets,
    max_api_calls_per_month, max_storage_mb, features, sort_order
) VALUES
('plan-free', 'free', '免费版', '适合个人开发者的免费套餐', 0, 0, 3, 5, 10, 50, 1000, 100, '{"custom_domain": false, "sso": false, "priority_support": false, "audit_logs": false}', 1),
('plan-starter', 'starter', '基础版', '适合小团队的入门套餐', 29, 290, 10, 20, 50, 200, 10000, 1024, '{"custom_domain": false, "sso": false, "priority_support": false, "audit_logs": true}', 2),
('plan-pro', 'pro', '专业版', '适合成长中团队的专业套餐', 99, 990, 50, 100, 200, 1000, 100000, 10240, '{"custom_domain": true, "sso": false, "priority_support": true, "audit_logs": true}', 3),
('plan-enterprise', 'enterprise', '企业版', '适合大型企业的定制套餐', 0, NULL, -1, -1, -1, -1, -1, -1, '{"custom_domain": true, "sso": true, "priority_support": true, "audit_logs": true, "dedicated_support": true, "sla": "99.99%"}', 4);

CREATE TABLE IF NOT EXISTS payment_methods (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    stripe_payment_method_id TEXT NOT NULL,
    stripe_customer_id TEXT NOT NULL,
    type TEXT NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    card_brand TEXT,
    card_last4 TEXT,
    card_exp_month INTEGER,
    card_exp_year INTEGER,
    status TEXT DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_payment_methods_tenant ON payment_methods(tenant_id);
CREATE INDEX IF NOT EXISTS idx_payment_methods_stripe ON payment_methods(stripe_payment_method_id);
