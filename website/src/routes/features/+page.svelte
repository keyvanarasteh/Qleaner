<script lang="ts">
	import { Trash2, Search, ShieldCheck, Activity } from 'lucide-svelte';
</script>

<div class="w-full flex flex-col items-center py-12 space-y-16">
	<!-- Header -->
	<div class="text-center max-w-3xl">
		<h1 class="text-4xl md:text-5xl font-extrabold tracking-tight text-white">
			Next-Generation <br/>
			<span class="text-transparent bg-clip-text bg-gradient-to-r from-neon-teal to-neon-cyan">System Cleaning</span>
		</h1>
		<p class="mt-4 text-lg text-slate-400">
			Qleaner provides an extensive set of tools to reclaim storage space, boost system performance, and maintain privacy—all encapsulated in a stunning user interface.
		</p>
	</div>

	<!-- Extended Features Grid -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-8 w-full max-w-5xl">
		
		<!-- Comprehensive Scanning -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-neon-cyan/10 text-neon-cyan">
				<Search class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">Deep System Scanning</h2>
			<p class="text-slate-400 leading-relaxed">
				Powered by a highly concurrent Rust backend, Qleaner iterates across millions of files in seconds. Identify hidden caches, old log files, package manager remnants, and leftover configuration data instantly.
			</p>
		</div>

		<!-- Cache & Junk Removal -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-neon-magenta/10 text-neon-magenta">
				<Trash2 class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">Aggressive Cache Sweeping</h2>
			<p class="text-slate-400 leading-relaxed">
				Free up gigabytes of storage by safely clearing npm, cargo, docker, and pip caches, along with browser cache arrays that hoard storage space on your primary drive.
			</p>
		</div>

		<!-- Process Insights -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-neon-teal/10 text-neon-teal">
				<Activity class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">Real-time Visualization</h2>
			<p class="text-slate-400 leading-relaxed">
				Every scan visualizes your disk usage in a sleek, glassmorphic Svelte 5 interface. Track the largest files and folders dynamically as the background engine discovers them.
			</p>
		</div>

		<!-- Uncompromising Security -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-white/10 text-white group-hover:text-neon-cyan group-hover:bg-neon-cyan/10 transition-colors">
				<ShieldCheck class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">Absolute Safety</h2>
			<p class="text-slate-400 leading-relaxed">
				Written in Rust without any `unsafe` blocks. Qleaner ensures that core system files are strictly blacklisted, utilizing user-level permissions to prevent accidental system bricks.
			</p>
		</div>

	</div>
</div>
