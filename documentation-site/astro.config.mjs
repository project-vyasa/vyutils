// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
// Published at https://project-vyasa.github.io/vyutils/
export default defineConfig({
	site: 'https://project-vyasa.github.io',
	base: '/vyutils',
	trailingSlash: 'always',
	server: { port: 9800 },
	preview: { port: 9800 },
	integrations: [
		starlight({
			title: 'vyutils',
			pagination: false,
			customCss: ['./src/styles/custom.css'],
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/project-vyasa/vyutils',
				},
			],
			sidebar: [
				{
					label: 'Explanation',
					items: [{ autogenerate: { directory: 'explanation' } }],
				},
				{
					label: 'How-to guides',
					items: [{ autogenerate: { directory: 'guides' } }],
				},
				{
					label: 'Reference',
					items: [{ autogenerate: { directory: 'reference' } }],
				},
				{
					label: 'RFCs',
					items: [{ autogenerate: { directory: 'rfcs' } }],
				},
			],
		}),
	],
});
