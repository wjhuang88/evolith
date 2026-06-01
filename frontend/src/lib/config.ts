type RuntimeConfig = {
  apiBaseUrl: string;
  stripePublishableKey: string;
  nodeEnv: string;
};

declare global {
  interface Window {
    __EVOLITH_CONFIG__?: Partial<RuntimeConfig>;
  }
}

function getEnv(key: string, fallback: string = ''): string {
  if (typeof import.meta !== 'undefined' && (import.meta as unknown as Record<string, unknown>).env) {
    const env = (import.meta as unknown as Record<string, Record<string, string>>).env;
    const viteKey = key.replace('NEXT_PUBLIC_', 'VITE_');
    return env[viteKey] || env[key] || fallback;
  }
  if (typeof process !== 'undefined' && process.env) {
    return process.env[key] || fallback;
  }
  return fallback;
}

const config: RuntimeConfig = Object.freeze({
  apiBaseUrl:
    typeof window !== 'undefined' && window.__EVOLITH_CONFIG__?.apiBaseUrl
      ? window.__EVOLITH_CONFIG__.apiBaseUrl
      : getEnv('NEXT_PUBLIC_API_URL', '/api/v1'),
  stripePublishableKey:
    typeof window !== 'undefined' && window.__EVOLITH_CONFIG__?.stripePublishableKey
      ? window.__EVOLITH_CONFIG__.stripePublishableKey
      : getEnv('NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY', ''),
  nodeEnv:
    typeof window !== 'undefined' && window.__EVOLITH_CONFIG__?.nodeEnv
      ? window.__EVOLITH_CONFIG__.nodeEnv
      : getEnv('NODE_ENV', 'production'),
});

export default config;
