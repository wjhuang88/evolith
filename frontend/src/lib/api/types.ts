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

export interface AuthToken {
  token: string;
  expires_at: number;
  user: User;
  tenant?: TenantInfo;
}

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

export type UpdateToolRequest = Partial<CreateToolRequest>;

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

export type UpdateSkillRequest = Partial<CreateSkillRequest>;

// ============================================
// CLI Interface Types
// ============================================

export interface CliInterface {
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

export interface CreateCliInterfaceRequest {
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

export type UpdateCliInterfaceRequest = Partial<CreateCliInterfaceRequest>;

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
  max_cli_interfaces: number;
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

// ============================================
// API Key Types
// ============================================

export type ApiKeyCapability = 'read' | 'repo:read' | 'repo:write' | 'execute' | 'promote';

export interface ApiKey {
  id: string;
  name: string;
  key_prefix: string;
  // Existing keys may still expose legacy strings until they are rotated.
  permissions: string[];
  expires_at: string | null;
  rate_limit: number;
  status: 'active' | 'revoked' | 'expired';
  request_count: number;
  last_used_at: string | null;
  created_at: string;
}

export interface ApiKeyWithSecret extends ApiKey {
  key?: string; // Only shown once after creation
}

export interface CreateApiKeyRequest {
  name: string;
  permissions?: ApiKeyCapability[];
  rate_limit?: number;
  expires_in_days?: number;
}

// ============================================
// Member & Invitation Types
// ============================================

export interface Member {
  id: string;
  email: string;
  username: string;
  full_name: string | null;
  role: string;
  tenant_role: string;
  status: string;
  joined_at: string | null;
  last_login_at: string | null;
  avatar_url: string | null;
}

export interface Invitation {
  id: string;
  email: string;
  role: string;
  status: string;
  invited_by: string;
  expires_at: string;
  created_at: string;
}

export interface InviteMemberRequest {
  email: string;
  role: string;
  message?: string;
}

// ============================================
// Repo Types (EVO-103 / EVO-112-A)
// ============================================

export interface Repo {
  id: string;
  tenant_id: string;
  name: string;
  description: string;
  default_branch: string;
  storage_path: string;
  visibility: 'public' | 'private';
  auto_merge: boolean;
  require_review: boolean;
  last_commit_sha: string | null;
  last_committed_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateRepoRequest {
  name: string;
  description?: string;
  default_branch?: string;
  visibility?: 'public' | 'private';
  auto_merge?: boolean;
  require_review?: boolean;
  seed_template?: boolean;
}

export type UpdateRepoRequest = Partial<CreateRepoRequest>;

export interface FileTreeEntry {
  name: string;
  kind: 'blob' | 'tree';
  oid: string;
  is_tree: boolean;
}

export interface CommitInfo {
  sha: string;
  author_name: string;
  author_email: string;
  message: string;
  timestamp: number;
}

export interface DiffEntry {
  path: string;
  change_type: 'added' | 'deleted' | 'modified';
  old_oid: string;
  new_oid: string;
}

export interface CommitDetail {
  sha: string;
  author_name: string;
  author_email: string;
  authored_at: number;
  committer_name: string;
  committer_email: string;
  committed_at: number;
  message: string;
  parents: string[];
  ref_name: string;
  changes: DiffEntry[];
}
