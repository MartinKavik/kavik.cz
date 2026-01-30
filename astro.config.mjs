// @ts-check
import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';

// https://astro.build/config
export default defineConfig({
	site: 'https://kavik.cz',
	integrations: [mdx(), sitemap({ filter: (page) => !page.includes('og-preview') })],
	devToolbar: { enabled: false },
});
