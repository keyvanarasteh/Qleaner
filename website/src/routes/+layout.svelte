<script lang="ts">
	import type { Pathname } from '$app/types';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { locales, localizeHref } from '$lib/paraglide/runtime';
	import type { Snippet } from 'svelte';
	import './layout.css';
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import Footer from '$lib/components/ui/Footer.svelte';

	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<title>Qleaner | Cross-platform System Utility</title>

	<meta
		name="description"
		content="A blazing fast, cross-platform system utility and disk cleaner."
	/>

	<!-- Inter font for sleek typography -->

	<link rel="preconnect" href="https://fonts.googleapis.com" />

	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />

	<link
		href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="relative flex min-h-screen w-full flex-col bg-obsidian text-slate-200">
	<!-- Ambient background glow elements -->

	<div class="pointer-events-none absolute inset-0 overflow-hidden opacity-30 mix-blend-screen">
		<div
			class="absolute -top-[10%] -left-[10%] h-[50vh] w-[50vh] rounded-full bg-neon-cyan/20 blur-[120px]"
		></div>

		<div
			class="absolute top-[20%] -right-[10%] h-[40vh] w-[40vh] rounded-full bg-neon-magenta/20 blur-[100px]"
		></div>
	</div>

	<Navbar />

	<main
		class="relative z-10 mx-auto flex w-full max-w-7xl flex-1 flex-col items-center px-4 pt-24 pb-16 sm:px-6 lg:px-8"
	>
		{@render children()}
	</main>

	<Footer />
</div>

<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(localizeHref(page.url.pathname, { locale }) as Pathname)}>{locale}</a>
	{/each}
</div>
