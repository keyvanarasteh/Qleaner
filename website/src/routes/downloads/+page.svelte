<script lang="ts">
	import { DownloadCloud, Info } from 'lucide-svelte';

	// Mock OS Download Data
	const downloads = [
		{
			os: 'Linux',
			icon: '🐧',
			format: '.AppImage',
			arch: 'x86_64',
			color: 'from-neon-magenta/20 to-transparent border-neon-magenta/30',
			text: 'text-neon-magenta',
			button: 'bg-neon-magenta/20 hover:bg-neon-magenta/40 text-neon-magenta'
		},
		{
			os: 'macOS',
			icon: '🍎',
			format: '.dmg',
			arch: 'Universal (Apple Silicon & Intel)',
			color: 'from-white/10 to-transparent border-white/20',
			text: 'text-white',
			button: 'bg-white/10 hover:bg-white/20 text-white'
		},
		{
			os: 'Windows',
			icon: '🪟',
			format: '.msi',
			arch: 'x64',
			color: 'from-neon-cyan/20 to-transparent border-neon-cyan/30',
			text: 'text-neon-cyan',
			button: 'bg-neon-cyan/20 hover:bg-neon-cyan/40 text-neon-cyan'
		}
	];
</script>

<div class="w-full flex flex-col items-center py-12 space-y-12">
	
	<div class="text-center max-w-2xl">
		<h1 class="text-4xl md:text-5xl font-extrabold tracking-tight text-white mb-4">
			Download <span class="text-transparent bg-clip-text bg-gradient-to-r from-neon-cyan to-neon-teal">Qleaner</span>
		</h1>
		<p class="text-lg text-slate-400">
			Choose your operating system below to get the latest stable version of Qleaner (v1.0.0). No installation required for portable formats.
		</p>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-3 gap-8 w-full max-w-5xl">
		{#each downloads as dl}
			<div class="relative flex flex-col rounded-2xl border bg-gradient-to-br bg-obsidian p-8 transition-transform hover:-translate-y-1 {dl.color}">
				<div class="text-4xl mb-4">{dl.icon}</div>
				
				<h2 class="text-2xl font-bold {dl.text} mb-1">{dl.os}</h2>
				<div class="text-sm text-slate-400 mb-6">
					<p>Format: <strong>{dl.format}</strong></p>
					<p>Arch: <strong>{dl.arch}</strong></p>
				</div>

				<div class="mt-auto pt-6 border-t border-white/10">
					<button class="w-full inline-flex items-center justify-center gap-2 rounded-xl py-3 px-4 font-semibold transition-colors {dl.button}">
						<DownloadCloud class="w-5 h-5" />
						Download
					</button>
				</div>
			</div>
		{/each}
	</div>

	<!-- Info Alert -->
	<div class="flex items-start gap-3 rounded-lg border border-neon-teal/20 bg-neon-teal/5 p-4 max-w-3xl text-sm text-slate-300">
		<Info class="w-5 h-5 text-neon-teal shrink-0 mt-0.5" />
		<div>
			<strong class="text-neon-teal">System Requirements:</strong> Qleaner runs securely entirely locally on your machine. You do not need an internet connection after download. Administrative privileges might be required for deep system cache clearing on Windows and macOS.
		</div>
	</div>

</div>
