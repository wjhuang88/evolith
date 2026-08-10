import { apiClient } from './client';
import type {
  ApiResponse,
  Plan,
  Subscription,
  PaymentMethod,
  Invoice,
  UsageResponse,
  CreateSubscriptionRequest,
  UpdateSubscriptionRequest,
  CancelSubscriptionRequest,
  SetupPaymentMethodRequest,
  SetupPaymentMethodResponse,
} from './types';

interface PlanListResponse {
  plans: Plan[];
}

interface InvoiceListResponse {
  invoices: Invoice[];
  total: number;
}

interface SubscriptionResponse {
  subscription: Subscription | null;
}

export const billingApi = {
  getPlans: async () => {
    const response = await apiClient.get<ApiResponse<PlanListResponse>>('/billing/plans');
    return response.data;
  },

  getSubscription: async (tenantId: string) => {
    const response = await apiClient.get<ApiResponse<SubscriptionResponse>>(
      `/tenant/${tenantId}/billing/subscription`
    );
    return response.data;
  },

  createSubscription: async (tenantId: string, data: CreateSubscriptionRequest) => {
    const response = await apiClient.post<ApiResponse<Subscription>>(
      `/tenant/${tenantId}/billing/subscription`,
      data
    );
    return response.data;
  },

  updateSubscription: async (tenantId: string, data: UpdateSubscriptionRequest) => {
    const response = await apiClient.patch<ApiResponse<Subscription>>(
      `/tenant/${tenantId}/billing/subscription`,
      data
    );
    return response.data;
  },

  cancelSubscription: async (tenantId: string, data?: CancelSubscriptionRequest) => {
    const response = await apiClient.delete<ApiResponse<{ status: string; message: string }>>(
      `/tenant/${tenantId}/billing/subscription`,
      { data }
    );
    return response.data;
  },

  getInvoices: async (tenantId: string) => {
    const response = await apiClient.get<ApiResponse<InvoiceListResponse>>(
      `/tenant/${tenantId}/billing/invoices`
    );
    return response.data;
  },

  getUsage: async (tenantId: string) => {
    const response = await apiClient.get<ApiResponse<UsageResponse>>(
      `/tenant/${tenantId}/billing/usage`
    );
    return response.data;
  },
};

export const paymentMethodApi = {
  list: async (tenantId: string) => {
    const response = await apiClient.get<ApiResponse<{ payment_methods: PaymentMethod[] }>>(
      `/tenant/${tenantId}/billing/payment-methods`
    );
    return response.data;
  },

  createSetupIntent: async (tenantId: string, data?: SetupPaymentMethodRequest) => {
    const response = await apiClient.post<ApiResponse<SetupPaymentMethodResponse>>(
      `/tenant/${tenantId}/billing/payment-methods/setup`,
      data
    );
    return response.data;
  },

  setDefault: async (tenantId: string, paymentMethodId: string) => {
    const response = await apiClient.patch<ApiResponse<PaymentMethod>>(
      `/tenant/${tenantId}/billing/payment-methods/${paymentMethodId}/default`
    );
    return response.data;
  },

  remove: async (tenantId: string, paymentMethodId: string) => {
    const response = await apiClient.delete<ApiResponse<{ message: string }>>(
      `/tenant/${tenantId}/billing/payment-methods/${paymentMethodId}`
    );
    return response.data;
  },
};
