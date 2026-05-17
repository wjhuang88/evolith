'use client';

import { useState, useEffect } from 'react';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui/Button';
import { Modal } from '@/components/ui/Modal';
import { paymentMethodApi, type PaymentMethod } from '@/lib/api';
import config from '@/lib/config';

interface PaymentMethodsProps {
  tenantId: string;
}

const cardBrandIcons: Record<string, string> = {
  visa: '💳',
  mastercard: '💳',
  amex: '💳',
  discover: '💳',
  default: '💳',
};

export function PaymentMethods({ tenantId }: PaymentMethodsProps) {
  const [paymentMethods, setPaymentMethods] = useState<PaymentMethod[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddModal, setShowAddModal] = useState(false);
  const [addingPayment, setAddingPayment] = useState(false);

  useEffect(() => {
    loadPaymentMethods();
  }, [tenantId]);

  const loadPaymentMethods = async () => {
    try {
      setLoading(true);
      const response = await paymentMethodApi.list(tenantId);
      if (response.success && response.data) {
        setPaymentMethods(response.data.payment_methods);
      }
    } catch (error) {
      console.error('Failed to load payment methods:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSetDefault = async (paymentMethodId: string) => {
    try {
      await paymentMethodApi.setDefault(tenantId, paymentMethodId);
      await loadPaymentMethods();
    } catch (error) {
      console.error('Failed to set default payment method:', error);
    }
  };

  const handleRemove = async (paymentMethodId: string) => {
    if (!confirm('Are you sure you want to remove this payment method?')) return;
    
    try {
      await paymentMethodApi.remove(tenantId, paymentMethodId);
      await loadPaymentMethods();
    } catch (error) {
      console.error('Failed to remove payment method:', error);
    }
  };

  const handleAddPaymentMethod = async () => {
    try {
      setAddingPayment(true);
      const response = await paymentMethodApi.createSetupIntent(tenantId, {
        return_url: window.location.href,
      });
      
      if (response.success && response.data) {
        const stripe = await loadStripe(config.stripePublishableKey);
        if (stripe) {
          const { error } = await stripe.confirmCardSetup(response.data.client_secret);
          if (error) {
            console.error('Stripe setup error:', error);
          } else {
            await loadPaymentMethods();
            setShowAddModal(false);
          }
        }
      }
    } catch (error) {
      console.error('Failed to add payment method:', error);
    } finally {
      setAddingPayment(false);
    }
  };

  if (loading) {
    return (
      <Card>
        <CardContent className="py-8 text-center text-muted-foreground">
          Loading payment methods...
        </CardContent>
      </Card>
    );
  }

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>Payment Methods</CardTitle>
          <CardDescription>Manage your payment methods for subscription billing</CardDescription>
        </CardHeader>
        <CardContent>
          {paymentMethods.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-muted-foreground mb-4">No payment methods added yet</p>
              <Button onClick={() => setShowAddModal(true)}>
                Add Payment Method
              </Button>
            </div>
          ) : (
            <div className="space-y-4">
              {paymentMethods.map((pm) => (
                <div
                  key={pm.id}
                  className="flex items-center justify-between p-4 border rounded-lg"
                >
                  <div className="flex items-center gap-4">
                    <span className="text-2xl">
                      {pm.card ? cardBrandIcons[pm.card.brand] || cardBrandIcons.default : '💳'}
                    </span>
                    <div>
                      <p className="font-medium">
                        {pm.card ? `${pm.card.brand.toUpperCase()} •••• ${pm.card.last4}` : pm.type}
                      </p>
                      {pm.card && (
                        <p className="text-sm text-muted-foreground">
                          Expires {pm.card.exp_month}/{pm.card.exp_year}
                        </p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    {pm.is_default && (
                      <span className="text-xs bg-primary/10 text-primary px-2 py-1 rounded">
                        Default
                      </span>
                    )}
                    {!pm.is_default && (
                      <Button variant="ghost" size="sm" onClick={() => handleSetDefault(pm.id)}>
                        Set Default
                      </Button>
                    )}
                    <Button
                      variant="ghost"
                      size="sm"
                      className="text-destructive hover:text-destructive"
                      onClick={() => handleRemove(pm.id)}
                    >
                      Remove
                    </Button>
                  </div>
                </div>
              ))}
              <Button variant="outline" className="w-full" onClick={() => setShowAddModal(true)}>
                Add Payment Method
              </Button>
            </div>
          )}
        </CardContent>
      </Card>

      <Modal isOpen={showAddModal} onClose={() => setShowAddModal(false)}>
        <div className="p-6">
          <h2 className="text-xl font-bold mb-4">Add Payment Method</h2>
          <p className="text-muted-foreground mb-6">
            You will be redirected to Stripe to securely add your payment method.
          </p>
          <div className="flex gap-4 justify-end">
            <Button variant="outline" onClick={() => setShowAddModal(false)}>
              Cancel
            </Button>
            <Button onClick={handleAddPaymentMethod} disabled={addingPayment}>
              {addingPayment ? 'Processing...' : 'Continue to Stripe'}
            </Button>
          </div>
        </div>
      </Modal>
    </>
  );
}

async function loadStripe(publishableKey: string): Promise<import('@stripe/stripe-js').Stripe | null> {
  const { loadStripe } = await import('@stripe/stripe-js');
  return loadStripe(publishableKey);
}