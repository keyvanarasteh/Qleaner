<script lang="ts">
	import { FileText, Cpu, Command } from 'lucide-svelte';
</script>

<div class="w-full flex flex-col items-center py-12 max-w-4xl mx-auto space-y-12">
	<!-- Docs Header -->
	<div class="text-center w-full border-b border-obsidian-border pb-8">
		<h1 class="text-4xl font-bold text-white mb-4">Documentation</h1>
		<p class="text-lg text-slate-400">Everything you need to know about setting up and using Qleaner.</p>
	</div>

	<!-- Content Section -->
	<div class="w-full space-y-12">
		<!-- Getting Started -->
		<section class="space-y-4">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-neon-cyan/20 text-neon-cyan">
					<Command class="w-5 h-5" />
				</div>
				<h2 class="text-2xl font-semibold text-white">Installation</h2>
			</div>
			
			<div class="rounded-xl border border-obsidian-border bg-obsidian-light/50 p-6 space-y-4">
				<h3 class="text-white font-medium">Linux</h3>
				<p class="text-slate-400 text-sm">Download the AppImage, make it executable, and run it.</p>
				<pre class="bg-black/50 p-4 rounded-lg text-neon-cyan text-sm"><code class="font-mono">chmod +x Qleaner-x86_64.AppImage
./Qleaner-x86_64.AppImage</code></pre>

				<h3 class="text-white font-medium mt-6">macOS & Windows</h3>
				<p class="text-slate-400 text-sm">Download the appropriate `.dmg` or `.msi` installer from the Downloads page and follow the setup wizard.</p>
			</div>
		</section>

		<!-- Usage -->
		<section class="space-y-4">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-neon-magenta/20 text-neon-magenta">
					<FileText class="w-5 h-5" />
				</div>
				<h2 class="text-2xl font-semibold text-white">Running a Clean</h2>
			</div>
			
			<div class="rounded-xl border border-obsidian-border bg-obsidian-light/50 p-6 space-y-4 text-slate-300">
				<ol class="list-decimal list-inside space-y-3">
					<li>Open Qleaner from your application launcher.</li>
					<li>Navigate to the <strong class="text-white">Scanner</strong> tab on the sidebar.</li>
					<li>Select the cache categories you wish to target (e.g., Cargo, NPM, Browsers).</li>
					<li>Click <strong class="text-neon-magenta">Deep Scan</strong> to analyze the directories.</li>
					<li>Review the detected junk files, then hit <strong class="text-neon-cyan">Clean Selected</strong> to permanently remove them.</li>
				</ol>
			</div>
		</section>

		<!-- Architecture Overview -->
		<section class="space-y-4">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-neon-teal/20 text-neon-teal">
					<Cpu class="w-5 h-5" />
				</div>
				<h2 class="text-2xl font-semibold text-white">Under the Hood</h2>
			</div>
			
			<div class="rounded-xl border border-obsidian-border bg-obsidian-light/50 p-6 space-y-4 text-slate-300">
				<p>
					Qleaner leverages <strong>Tauri v2</strong> to bridge the gap between a high-performance system-level Rust core and a beautiful Svelte 5 frontend. 
				</p>
				<p>
					The core analysis engine utilizes <code>tokio::fs</code> for asynchronous parallel traversal of directories, ensuring the UI remains perfectly fluid at 60fps during intense deep scans.
				</p>
			</div>
		</section>
	</div>
</div>
