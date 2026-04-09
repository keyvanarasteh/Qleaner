<script lang="ts">
	import { resolve } from '$app/paths';
	import { Download, Zap, Shield, HardDrive, Terminal } from 'lucide-svelte';
</script>

<div class="w-full flex-1 flex flex-col items-center justify-center text-center space-y-20 min-h-[80vh]">
	
	<!-- Hero Section -->
	<section class="flex flex-col items-center max-w-4xl space-y-6 pt-10">
		<div class="inline-flex items-center gap-2 rounded-full border border-neon-cyan/30 bg-neon-cyan/10 px-4 py-1.5 text-sm font-medium text-neon-cyan backdrop-blur-md">
			<span class="flex h-2 w-2 rounded-full bg-neon-cyan animate-pulse"></span>
			v1.0.0 is now available
		</div>
		
		<h1 class="text-5xl md:text-7xl font-extrabold tracking-tight text-white">
			Reclaim <span class="text-transparent bg-clip-text bg-gradient-to-r from-neon-cyan to-neon-teal">Disk Space.</span> <br/>
			Boost <span class="text-transparent bg-clip-text bg-gradient-to-r from-neon-magenta to-neon-cyan">Performance.</span>
		</h1>
		
		<p class="mt-4 max-w-2xl text-xl text-slate-400">
			A blazing fast, cross-platform system utility built with Rust, Svelte 5, and Tauri.
			Deep clean your OS safely, locally, and beautifully.
		</p>
		
		<div class="mt-10 flex flex-col sm:flex-row gap-4 w-full sm:w-auto">
			<a 
				href={resolve('/downloads')} 
				class="group relative inline-flex items-center justify-center gap-2 overflow-hidden rounded-xl bg-gradient-to-r from-neon-cyan to-neon-teal px-8 py-4 font-semibold text-black shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all hover:scale-105 hover:shadow-[0_0_30px_rgba(6,182,212,0.6)] focus:outline-none focus:ring-2 focus:ring-neon-cyan focus:ring-offset-2 focus:ring-offset-obsidian"
			>
				<span class="absolute inset-0 bg-white/20 opacity-0 transition-opacity group-hover:opacity-100"></span>
				<Download class="w-5 h-5" />
				Download for Free
			</a>
			<a 
				href={resolve('/docs')} 
				class="inline-flex items-center justify-center gap-2 rounded-xl border border-obsidian-border bg-obsidian-glass backdrop-blur-md px-8 py-4 font-semibold text-white transition-colors hover:bg-white/5 hover:text-neon-cyan"
			>
				<Terminal class="w-5 h-5" />
				View Documentation
			</a>
		</div>
	</section>

	<!-- Quick Features Grid -->
	<section class="w-full">
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
			<!-- Feature 1 -->
			<div class="group relative overflow-hidden rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:border-neon-cyan/50 hover:bg-obsidian-light/60">
				<div class="absolute -right-10 -top-10 h-32 w-32 rounded-full bg-neon-cyan/10 blur-[50px] transition-all group-hover:bg-neon-cyan/20"></div>
				<div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-neon-cyan/10 text-neon-cyan">
					<Zap class="w-6 h-6" />
				</div>
				<h3 class="text-xl font-bold text-white mb-2">Blazing Fast</h3>
				<p class="text-slate-400">Powered by Rust and Tokio async I/O. Scans and cleans your filesystem with unparalleled speed, unlocking gigabytes in seconds.</p>
			</div>

			<!-- Feature 2 -->
			<div class="group relative overflow-hidden rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:border-neon-teal/50 hover:bg-obsidian-light/60">
				<div class="absolute -right-10 -top-10 h-32 w-32 rounded-full bg-neon-teal/10 blur-[50px] transition-all group-hover:bg-neon-teal/20"></div>
				<div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-neon-teal/10 text-neon-teal">
					<HardDrive class="w-6 h-6" />
				</div>
				<h3 class="text-xl font-bold text-white mb-2">Cross-Platform</h3>
				<p class="text-slate-400">Native performance and identical UI across Linux, macOS, and Windows. Build once, clean anywhere using Tauri v2.</p>
			</div>

			<!-- Feature 3 -->
			<div class="group relative overflow-hidden rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:border-neon-magenta/50 hover:bg-obsidian-light/60">
				<div class="absolute -right-10 -top-10 h-32 w-32 rounded-full bg-neon-magenta/10 blur-[50px] transition-all group-hover:bg-neon-magenta/20"></div>
				<div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-neon-magenta/10 text-neon-magenta">
					<Shield class="w-6 h-6" />
				</div>
				<h3 class="text-xl font-bold text-white mb-2">Memory Safe</h3>
				<p class="text-slate-400">Strict zero-unsafe Rust architecture ensures zero crashes, zero memory leaks, and absolute safety for your operating system.</p>
			</div>
		</div>
	</section>
</div>
