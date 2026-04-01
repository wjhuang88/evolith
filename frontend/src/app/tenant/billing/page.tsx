'use client';

import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui';
import { Modal } from '@/components/ui/Modal';
import { PaymentMethods, UsageDisplay } from '@/components/billing';
import { billingApi, type Plan, type Subscription } from '@/lib/api';

export default function BillingPage() {
  const { t } = useTranslation();
  const { user, tenant } = useAuthStore();
  const [billingCycle, setBillingCycle] = useState<'monthly' | 'yearly'>('monthly');
  const [plans, setPlans] = useState<Plan[]>([]);
  const [subscription, setSubscription] = useState<Subscription | null>(null);
  const [loading, setLoading] = useState(true);
  const [showUpgradeModal, setShowUpgradeModal] = useState(false);
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);
  const [processing, setProcessing] = useState(false);

  const tenantId = tenant?.id || '';

  useEffect(() => {
    loadData();
  }, [tenantId]);

  const loadData = async () => {
    try {
      setLoading(true);
      const [plansRes, subRes] = await Promise.all([
        billingApi.getPlans(),
        tenantId ? billingApi.getSubscription(tenantId) : null,
      ]);
      
      if (plansRes.success && plansRes.data) {
        setPlans(plansRes.data.plans);
      }
      
      if (subRes?.success && subRes.data?.subscription) {
        setSubscription(subRes.data.subscription);
      }
    } catch (error) {
      console.error('Failed to load billing data:', error);
    } finally {
      setLoading(false);
    }
  };

  const currentPlan = subscription?.plan || plans.find(p => p.name === tenant?.plan) || plans[0];

  const handleSelectPlan = (plan: Plan) => {
    setSelectedPlan(plan);
    setShowUpgradeModal(true);
  };

  const handleConfirmUpgrade = async () => {
    if (!selectedPlan || !tenantId) return;
    
    try {
      setProcessing(true);
      
      if (subscription) {
        await billingApi.updateSubscription(tenantId, {
          plan_id: selectedPlan.id,
          billing_cycle: billingCycle,
          proration: 'immediate',
        });
      } else {
        await billingApi.createSubscription(tenantId, {
          plan_id: selectedPlan.id,
          billing_cycle: billingCycle,
        });
      }
      
      await loadData();
      setShowUpgradeModal(false);
    } catch (error) {
      console.error('Failed to update subscription:', error);
    } finally {
      setProcessing(false);
    }
  };

  const handleCancelSubscription = async () => {
    if (!tenantId || !confirm(t('tenant.billing.confirmCancel'))) return;
    
    try {
      await billingApi.cancelSubscription(tenantId, {
        reason: 'User requested cancellation',
      });
      await loadData();
    } catch (error) {
      console.error('Failed to cancel subscription:', error);
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center py-12 text-muted-foreground">{t('common.loading')}</div>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{t('tenant.billing.title')}</h1>
        <p className="text-muted-foreground mt-1">
          {t('tenant.billing.subtitle')}
        </p>
      </div>

      <div className="grid gap-6 lg:grid-cols-2 mb-8">
        <Card>
          <CardHeader>
            <CardTitle>{t('tenant.billing.currentPlan')}</CardTitle>
            <CardDescription>{t('tenant.billing.currentPlanDesc')}</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex items-center justify-between">
              <div>
                <p className="text-2xl font-bold text-foreground">{currentPlan?.display_name}</p>
                <p className="text-muted-foreground">
                  {currentPlan?.monthly_price === 0 
                    ? t('tenant.billing.freeForever') 
                    : `$${billingCycle === 'yearly' && currentPlan?.yearly_price 
                        ? currentPlan.yearly_price 
                        : currentPlan?.monthly_price}${billingCycle === 'yearly' ? t('tenant.billing.perYear') : t('tenant.billing.perMonth')}`
                  }
                </p>
              </div>
              <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                {subscription?.status || t('tenant.billing.active')}
              </span>
            </div>
          </CardContent>
          {currentPlan?.name !== 'enterprise' && (
            <CardFooter className="border-t bg-muted/50">
              {subscription && (
                <Button variant="outline" className="w-full" onClick={handleCancelSubscription}>
                  {t('tenant.billing.cancelSubscription')}
                </Button>
              )}
            </CardFooter>
          )}
        </Card>

        {tenantId && <UsageDisplay tenantId={tenantId} />}
      </div>

      {tenantId && <PaymentMethods tenantId={tenantId} />}

      <div className="mt-8">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-4">
          <h2 className="text-2xl font-bold">{t('tenant.billing.availablePlans')}</h2>
          <div className="flex gap-2">
            <Button
              variant={billingCycle === 'monthly' ? 'primary' : 'outline'}
              size="sm"
              onClick={() => setBillingCycle('monthly')}
              className="w-full sm:w-auto"
            >
              {t('tenant.billing.monthly')}
            </Button>
            <Button
              variant={billingCycle === 'yearly' ? 'primary' : 'outline'}
              size="sm"
              onClick={() => setBillingCycle('yearly')}
              className="w-full sm:w-auto"
            >
              {t('tenant.billing.yearlySave')}
            </Button>
          </div>
        </div>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
          {plans.map((plan) => (
            <Card
              key={plan.id}
              className={`relative ${plan.name === 'pro' ? 'border-primary ring-2 ring-primary/20' : ''}`}
            >
              {plan.name === 'pro' && (
                <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                  <span className="rounded-full bg-primary px-3 py-1 text-xs font-medium text-white">
                    {t('tenant.billing.popular')}
                  </span>
                </div>
              )}
              <CardHeader>
                <CardTitle>{plan.display_name}</CardTitle>
                <div className="mt-2">
                  <span className="text-3xl font-bold text-foreground">
                    {plan.monthly_price === 0 
                      ? t('tenant.billing.free') 
                      : `$${billingCycle === 'yearly' && plan.yearly_price ? plan.yearly_price : plan.monthly_price}`
                    }
                  </span>
                  {plan.monthly_price > 0 && (
                    <span className="text-muted-foreground">{billingCycle === 'yearly' ? t('tenant.billing.perYear') : t('tenant.billing.perMonth')}</span>
                  )}
                </div>
              </CardHeader>
              <CardFooter>
                {plan.id === currentPlan?.id ? (
                  <Button variant="outline" className="w-full" disabled>
                    {t('tenant.billing.currentPlanBadge')}
                  </Button>
                ) : plan.name === 'enterprise' ? (
                  <Button variant="outline" className="w-full">
                    {t('tenant.billing.contactSales')}
                  </Button>
                ) : (
                  <Button
                    variant={plan.name === 'pro' ? 'primary' : 'outline'}
                    className="w-full"
                    onClick={() => handleSelectPlan(plan)}
                  >
                    {plan.monthly_price === 0 || (currentPlan && plans.findIndex(p => p.id === plan.id) < plans.findIndex(p => p.id === currentPlan.id)) 
                      ? t('tenant.billing.downgrade') 
                      : t('tenant.billing.upgrade')}
                  </Button>
                )}
              </CardFooter>
            </Card>
          ))}
        </div>
      </div>

      <Modal isOpen={showUpgradeModal} onClose={() => setShowUpgradeModal(false)}>
        <div className="p-6">
          <h2 className="text-xl font-bold mb-4">
            {t('tenant.billing.upgradeModal.title', { name: selectedPlan?.display_name })}
          </h2>
          <p className="text-muted-foreground mb-4">
            {subscription ? t('tenant.billing.upgradeModal.changeTo', { name: selectedPlan?.display_name }) : t('tenant.billing.upgradeModal.subscribeTo', { name: selectedPlan?.display_name })}
          </p>
          <div className="bg-muted p-4 rounded-lg mb-6">
            <div className="flex justify-between">
              <span>{t('tenant.billing.upgradeModal.planLabel')}</span>
              <span className="font-medium">{selectedPlan?.display_name}</span>
            </div>
            <div className="flex justify-between mt-2">
              <span>{t('tenant.billing.upgradeModal.billingLabel')}</span>
              <span className="font-medium">{billingCycle}</span>
            </div>
            <div className="flex justify-between mt-2">
              <span>{t('tenant.billing.upgradeModal.priceLabel')}</span>
              <span className="font-medium">
                ${billingCycle === 'yearly' && selectedPlan?.yearly_price 
                  ? selectedPlan.yearly_price 
                  : selectedPlan?.monthly_price}{billingCycle === 'yearly' ? t('tenant.billing.perYear') : t('tenant.billing.perMonth')}
              </span>
            </div>
          </div>
          <div className="flex flex-col sm:flex-row gap-4 justify-end">
            <Button variant="outline" onClick={() => setShowUpgradeModal(false)} className="w-full sm:w-auto">
              {t('common.cancel')}
            </Button>
            <Button onClick={handleConfirmUpgrade} disabled={processing} className="w-full sm:w-auto">
              {processing ? t('common.processing') : t('common.confirm')}
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}