import { injectSpeedInsights } from '@vercel/speed-insights/sveltekit';
import { dev } from '$app/environment';
import { injectAnalytics } from '@vercel/analytics/sveltekit';

injectAnalytics({ mode: dev ? 'development' : 'production' });
injectSpeedInsights();
