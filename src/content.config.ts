import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

export const collections = {
	work: defineCollection({
		// Load Markdown files in the src/content/work directory.
		loader: glob({ base: './src/content/work', pattern: '**/*.md' }),
		schema: z.object({
			title: z.string(),
			description: z.string(),
			tags: z.array(z.string()),
			img: z.string(),
			img_alt: z.string().optional(),
			sortOrder: z.number().optional(),
			links: z
				.array(z.object({ name: z.string(), url: z.string().url() }))
				.default([])
				.optional(),
		}),
	}),
	posts: defineCollection({
		loader: glob({ base: './src/content/posts', pattern: '**/*.md' }),
		schema: z.object({
			title: z.string(),
			description: z.string(),
			publishDate: z.coerce.date(),
			tags: z.array(z.string()).default([]),
			img: z.string().optional(),
			img_alt: z.string().optional(),
		}),
	}),
};
