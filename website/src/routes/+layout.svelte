<script lang="ts">
	import type { Snippet } from 'svelte';
	import './layout.css';
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import Footer from '$lib/components/ui/Footer.svelte';

	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<title>Qleaner | Cross-platform System Utility</title>
	<meta name="description" content="A blazing fast, cross-platform system utility and disk cleaner." />
	<!-- Inter font for sleek typography -->
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="flex min-h-screen flex-col relative w-full bg-obsidian text-slate-200">
	<!-- Ambient background glow elements -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden mix-blend-screen opacity-30">
		<div class="absolute -top-[10%] -left-[10%] h-[50vh] w-[50vh] rounded-full bg-neon-cyan/20 blur-[120px]"></div>
		<div class="absolute top-[20%] -right-[10%] h-[40vh] w-[40vh] rounded-full bg-neon-magenta/20 blur-[100px]"></div>
	</div>

	<Navbar />

	<main class="flex-1 relative z-10 w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-24 pb-16 flex flex-col items-center">
		{@render children()}
	</main>

	<Footer />
</div>
