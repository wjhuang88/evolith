// ============================================
// Types
// ============================================

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  meta?: PaginationMeta;
  error?: ErrorInfo;
}

export interface ErrorInfo {
  code: string;
  message: string;
  retryable?: boolean;
  details?: Record<string, unknown>;
}

export interface PaginationMeta {
  page: number;
  per_page: number;
  total: number;
  total_pages?: number;
}

// ============================================
// Auth Types
// ============================================

export interface User {
  id: string;
  email: string;
  username: string;
  role: string;
  created_at?: string;
}

export interface AuthToken {
  token: string;
  expires_at: number;
  user: User;
  tenant?: TenantInfo;
}

// Updated User interface with tenant fields
export interface User {
  id: string;
  email: string;
  username: string;
  role: string;
  tenant_id: string;
  tenant_role: string;
  email_verified?: boolean;
  created_at?: string;
}

// Tenant info from auth response
export interface TenantInfo {
  id: string;
  name: string;
  slug: string;
  plan: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  email: string;
  username: string;
  password: string;
  // Tenant info (for registration)
  tenant_name?: string;
  tenant_slug?: string;
}

// ============================================
// Tenant Types
// ============================================

export type TenantPlan = 'free' | 'starter' | 'pro' | 'enterprise';
export type TenantStatus = 'active' | 'suspended' | 'deleted';
export type TenantRole = 'owner' | 'admin' | 'member';

export interface Tenant {
  id: string;
  name: string;
  slug: string;
  domain?: string;
  plan: TenantPlan;
  status: TenantStatus;
}

// ============================================
// Tool Types
// ============================================

export interface Tool {
  id: string;
  name: string;
  description: string;
  category: string;
  input_schema: Record<string, unknown>;
  schema?: Record<string, unknown>;
  handler?: Record<string, unknown>;
  is_public: boolean;
  created_by?: string;
  owner_id?: string;
  tenant_id?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateToolRequest {
  name: string;
  description: string;
  category?: string;
  input_schema: Record<string, unknown>;
  type?: 'http' | 'function';
  handler_url?: string;
  handler_method?: string;
  handler_timeout?: number;
  is_public?: boolean;
}

export interface UpdateToolRequest extends Partial<CreateToolRequest> {}

// ============================================
// Skill Types
// ============================================

export interface Skill {
  id: string;
  name: string;
  description: string;
  version: string;
  content: string;
  category: string;
  tags: string[];
  is_public: boolean;
  created_by?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateSkillRequest {
  name: string;
  description: string;
  version: string;
  content: string;
  runtime?: 'python311' | 'node20' | 'wasm';
  dependencies?: Array<{ name: string; version: string }>;
  category?: string;
  tags?: string[];
  is_public?: boolean;
}

export interface UpdateSkillRequest extends Partial<CreateSkillRequest> {}

// ============================================
// Snippet Types
// ============================================

export interface Snippet {
  id: string;
  title: string;
  name?: string;
  description: string;
  code: string;
  language: string;
  category: string;
  framework?: string;
  tags: string[];
  content?: string;
  dependencies?: Array<{ name: string; version: string; required: boolean }>;
  estimated_tokens?: number;
  is_public: boolean;
  created_by?: string;
  owner_id?: string;
  tenant_id?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateSnippetRequest {
  title: string;
  description: string;
  name?: string;
  content?: string;
  code: string;
  language: string;
  framework?: string;
  category?: string;
  tags?: string[];
  dependencies?: Array<{ name: string; version: string; required?: boolean }>;
  estimated_tokens?: number;
  is_public?: boolean;
}

export interface UpdateSnippetRequest extends Partial<CreateSnippetRequest> {}

// ============================================
// Payment & Billing Types
// ============================================

export interface Plan {
  id: string;
  name: string;
  display_name: string;
  description?: string;
  monthly_price: number;
  yearly_price?: number;
  max_users: number;
  max_tools: number;
  max_skills: number;
  max_snippets: number;
  max_api_calls_per_month: number;
  max_storage_mb: number;
  features: Record<string, unknown>;
  is_builtin: boolean;
}

export interface Subscription {
  id: string;
  plan: Plan;
  status: 'active' | 'trialing' | 'past_due' | 'canceled' | 'unpaid';
  billing_cycle: 'monthly' | 'yearly';
  current_period_start: string;
  current_period_end: string;
  trial_end_at?: string;
  cancel_at_period_end: boolean;
}

export interface PaymentMethod {
  id: string;
  type: 'card' | 'bank_account' | 'other';
  is_default: boolean;
  card?: {
    brand: string;
    last4: string;
    exp_month: number;
    exp_year: number;
  };
  status: 'active' | 'expired' | 'canceled';
  created_at: string;
}

export interface Invoice {
  id: string;
  invoice_number: string;
  status: 'draft' | 'issued' | 'paid' | 'void';
  subtotal: number;
  tax: number;
  total: number;
  currency: string;
  period_start: string;
  period_end: string;
  pdf_url?: string;
  created_at: string;
  paid_at?: string;
}

export interface ResourceUsage {
  resource_type: string;
  used: number;
  limit: number;
  percent: number;
  remaining?: number;
  overage?: number;
}

export interface UsagePeriod {
  start: string;
  end: string;
  remaining_days: number;
}

export interface UsageResponse {
  period: UsagePeriod;
  resources: ResourceUsage[];
}

export interface CreateSubscriptionRequest {
  plan_id: string;
  billing_cycle?: 'monthly' | 'yearly';
  payment_method_id?: string;
}

export interface UpdateSubscriptionRequest {
  plan_id?: string;
  billing_cycle?: 'monthly' | 'yearly';
  proration?: 'immediate' | 'next_billing_cycle';
}

export interface CancelSubscriptionRequest {
  reason?: string;
  feedback?: string;
}

export interface SetupPaymentMethodRequest {
  return_url?: string;
}

export interface SetupPaymentMethodResponse {
  setup_intent_id: string;
  client_secret: string;
}

// ============================================
// Query Options
// ============================================

export interface ListQueryParams {
  page?: number;
  per_page?: number;
  search?: string;
  category?: string;
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}

export interface ListResponse<T> {
  items: T[];
  meta: PaginationMeta;
}

export interface PaginatedResponse<T> {
  success: boolean;
  data: T[];
  meta: PaginationMeta;
  error?: ErrorInfo;
}
