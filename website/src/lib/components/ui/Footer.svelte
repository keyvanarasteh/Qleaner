<script lang="ts">
	import { resolve } from '$app/paths';
	const currYear = new Date().getFullYear();
</script>

<footer class="mt-auto border-t border-obsidian-border bg-obsidian-light/50 backdrop-blur-md py-8">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex flex-col md:flex-row items-center justify-between gap-4">
			<div class="flex flex-col md:flex-row items-center gap-4 text-sm text-slate-500">
				<span>&copy; {currYear} Qleaner Contributors.</span>
				<span class="hidden md:inline">•</span>
				<span>A cross-platform system utility built with Rust & Svelte 5.</span>
			</div>
			
			<div class="flex gap-6">
				<a href={resolve('/docs')} class="text-sm text-slate-400 hover:text-neon-cyan transition-colors">Documentation</a>
				<a href="https://github.com/keyvanarasteh/Qleaner/releases" target="_blank" rel="noopener noreferrer" class="text-sm text-slate-400 hover:text-neon-cyan transition-colors">Releases</a>
				<a href="https://github.com/keyvanarasteh/Qleaner/issues" target="_blank" rel="noopener noreferrer" class="text-sm text-slate-400 hover:text-neon-cyan transition-colors">Report Issue</a>
			</div>
		</div>
	</div>
</footer>
