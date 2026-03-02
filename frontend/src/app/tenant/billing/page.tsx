'use client';

import { useState } from 'react';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui';

interface Plan {
  id: string;
  name: string;
  displayName: string;
  price: number;
  yearlyPrice?: number;
  features: string[];
  highlighted?: boolean;
}

const plans: Plan[] = [
  {
    id: 'free',
    name: 'free',
    displayName: 'Free',
    price: 0,
    features: ['3 team members', '5 tools', '10 skills', '50 snippets', '1,000 API calls/month'],
  },
  {
    id: 'starter',
    name: 'starter',
    displayName: 'Starter',
    price: 29,
    yearlyPrice: 290,
    features: ['10 team members', '20 tools', '50 skills', '200 snippets', '10,000 API calls/month'],
  },
  {
    id: 'pro',
    name: 'pro',
    displayName: 'Professional',
    price: 99,
    yearlyPrice: 990,
    highlighted: true,
    features: ['50 team members', '100 tools', '200 skills', '1,000 snippets', '100,000 API calls/month'],
  },
  {
    id: 'enterprise',
    name: 'enterprise',
    displayName: 'Enterprise',
    price: 0,
    features: ['Unlimited everything', 'Dedicated support', 'SSO/SAML', 'SLA guarantee'],
  },
];

export default function BillingPage() {
  const { user, tenant } = useAuthStore();
  const [billingCycle, setBillingCycle] = useState<'monthly' | 'yearly'>('monthly');
  
  const currentPlan = tenant?.plan || 'free';
  const currentPlanData = plans.find(p => p.name === currentPlan) || plans[0];

  return (
    <div className="container mx-auto py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">Billing & Plans</h1>
        <p className="text-muted-foreground mt-1">
          Manage your subscription and billing information
        </p>
      </div>

      <Card className="mb-8">
        <CardHeader>
          <CardTitle>Current Plan</CardTitle>
          <CardDescription>Your active subscription</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-2xl font-bold text-foreground">{currentPlanData.displayName}</p>
              <p className="text-muted-foreground">
                {currentPlanData.price === 0 ? 'Free forever' : `$${currentPlanData.price}/month`}
              </p>
            </div>
            <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
              Active
            </span>
          </div>
        </CardContent>
        {currentPlan !== 'enterprise' && (
          <CardFooter className="border-t bg-muted/50">
            <Button variant="outline" className="w-full">
              {currentPlan === 'free' ? 'Upgrade Plan' : 'Change Plan'}
            </Button>
          </CardFooter>
        )}
      </Card>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        {plans.map((plan) => (
          <Card 
            key={plan.id} 
            className={`relative ${plan.highlighted ? 'border-primary ring-2 ring-primary/20' : ''}`}
          >
            {plan.highlighted && (
              <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                <span className="rounded-full bg-primary px-3 py-1 text-xs font-medium text-white">
                  Popular
                </span>
              </div>
            )}
            <CardHeader>
              <CardTitle>{plan.displayName}</CardTitle>
              <div className="mt-2">
                <span className="text-3xl font-bold text-foreground">
                  {plan.price === 0 ? 'Free' : `$${billingCycle === 'yearly' && plan.yearlyPrice ? plan.yearlyPrice : plan.price}`}
                </span>
                {plan.price > 0 && (
                  <span className="text-muted-foreground">/{billingCycle === 'yearly' ? 'year' : 'month'}</span>
                )}
              </div>
            </CardHeader>
            <CardFooter>
              {plan.id === currentPlan ? (
                <Button variant="outline" className="w-full" disabled>
                  Current Plan
                </Button>
              ) : plan.id === 'enterprise' ? (
                <Button variant="outline" className="w-full">
                  Contact Sales
                </Button>
              ) : (
                <Button variant={plan.highlighted ? 'primary' : 'outline'} className="w-full">
                  {plan.price === 0 ? 'Downgrade' : 'Upgrade'}
                </Button>
              )}
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}