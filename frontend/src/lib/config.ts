type RuntimeConfig = {
  apiBaseUrl: string;
  stripePublishableKey: string;
  nodeEnv: string;
};

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
  apiBaseUrl: getEnv('NEXT_PUBLIC_API_URL', '/api/v1'),
  stripePublishableKey: getEnv('NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY', ''),
  nodeEnv: getEnv('NODE_ENV', 'production'),
});

export default config;
