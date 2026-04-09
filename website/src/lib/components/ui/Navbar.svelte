<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { Menu, X, Trash2 } from 'lucide-svelte';

	let isOpen = $state(false);

	const links: Array<{ name: string; href: '/' | '/features' | '/docs' | '/downloads' }> = [
		{ name: 'Home', href: '/' },
		{ name: 'Features', href: '/features' },
		{ name: 'Docs', href: '/docs' },
		{ name: 'Downloads', href: '/downloads' }
	];

	let currentPath = $derived(page.url.pathname);
</script>

<header class="fixed top-0 left-0 right-0 z-50 transition-all duration-300 bg-obsidian-glass backdrop-blur-md border-b border-obsidian-border">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 items-center justify-between">
			<!-- Logo -->
			<div class="flex items-center gap-2">
				<a href={resolve('/')} class="flex items-center gap-2 group">
					<div class="flex h-9 w-9 items-center justify-center rounded-lg bg-gradient-to-br from-neon-cyan to-neon-teal shadow-[0_0_15px_rgba(6,182,212,0.5)] transition-all group-hover:scale-105 group-hover:shadow-[0_0_20px_rgba(6,182,212,0.8)]">
						<Trash2 class="text-white w-5 h-5" />
					</div>
					<span class="text-xl font-bold tracking-tight text-white transition-colors group-hover:text-neon-cyan">Qleaner</span>
				</a>
			</div>

			<!-- Desktop Nav -->
			<nav class="hidden md:flex gap-8">
				{#each links as link (link.name)}
					<a
						href={resolve(link.href)}
						class="text-sm font-medium transition-colors hover:text-neon-cyan {currentPath === link.href ? 'text-neon-cyan drop-shadow-[0_0_5px_rgba(6,182,212,0.8)]' : 'text-slate-400'}"
					>
						{link.name}
					</a>
				{/each}
			</nav>

			<!-- Desktop CTA -->
			<div class="hidden md:flex items-center gap-4">
				<a href="https://github.com/keyvanarasteh/Qleaner" aria-label="Qleaner GitHub Repository" target="_blank" rel="noopener noreferrer" class="text-slate-400 hover:text-white transition-colors">
					<svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
						<path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
					</svg>
				</a>
				<a href={resolve('/downloads')} class="rounded-md bg-white/10 px-4 py-2 text-sm font-medium text-white ring-1 ring-inset ring-white/20 transition-all hover:bg-neon-cyan/90 hover:text-black hover:ring-neon-cyan hover:shadow-[0_0_15px_rgba(6,182,212,0.5)]">
					Install Now
				</a>
			</div>

			<!-- Mobile Menu Toggle -->
			<div class="flex items-center md:hidden">
				<button onclick={() => (isOpen = !isOpen)} class="text-slate-400 hover:text-white p-2">
					<span class="sr-only">Open main menu</span>
					{#if isOpen}
						<X class="h-6 w-6" />
					{:else}
						<Menu class="h-6 w-6" />
					{/if}
				</button>
			</div>
		</div>
	</div>

	<!-- Mobile Menu -->
	{#if isOpen}
		<div class="md:hidden border-t border-obsidian-border bg-obsidian-light/95 backdrop-blur-xl">
			<div class="space-y-1 px-4 pb-3 pt-2">
				{#each links as link (link.name)}
					<a
						href={resolve(link.href)}
						onclick={() => (isOpen = false)}
						class="block rounded-md px-3 py-2 text-base font-medium {currentPath === link.href ? 'text-neon-cyan bg-neon-cyan/10' : 'text-slate-400 hover:bg-white/5 hover:text-white'}"
					>
						{link.name}
					</a>
				{/each}
				<a
					href={resolve('/downloads')}
					onclick={() => (isOpen = false)}
					class="block rounded-md px-3 py-2 text-base font-medium text-neon-teal hover:bg-white/5"
				>
					Install Now
				</a>
			</div>
		</div>
	{/if}
</header>
