<script lang="ts">
	import { FileText, Cpu, Command } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
</script>

<div class="w-full flex flex-col items-center py-12 max-w-4xl mx-auto space-y-12">
	<!-- Docs Header -->
	<div class="text-center w-full border-b border-obsidian-border pb-8">
		<h1 class="text-4xl font-bold text-white mb-4">{m.docs_hero_title()}</h1>
		<p class="text-lg text-slate-400">{m.docs_hero_desc()}</p>
	</div>

	<!-- Content Section -->
	<div class="w-full space-y-12">
		<!-- Getting Started -->
		<section class="space-y-4">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-neon-cyan/20 text-neon-cyan">
					<Command class="w-5 h-5" />
				</div>
				<h2 class="text-2xl font-semibold text-white">{m.docs_install_title()}</h2>
			</div>
			
			<div class="rounded-xl border border-obsidian-border bg-obsidian-light/50 p-6 space-y-4">
				<h3 class="text-white font-medium">Linux</h3>
				<p class="text-slate-400 text-sm">{@html m.docs_install_linux()}</p>
				<pre class="bg-black/50 p-4 rounded-lg text-neon-cyan text-sm"><code class="font-mono">chmod +x Qleaner-x86_64.AppImage
./Qleaner-x86_64.AppImage</code></pre>

				<h3 class="text-white font-medium mt-6">macOS & Windows</h3>
				<p class="text-slate-400 text-sm">{@html m.docs_install_macwin()}</p>
			</div>
		</section>

		<!-- Usage -->
		<section class="space-y-4">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-neon-magenta/20 text-neon-magenta">
					<FileText class="w-5 h-5" />
				</div>
				<h2 class="text-2xl font-semibold text-white">{m.docs_usage_title()}</h2>
			</div>
			
			<div class="rounded-xl border border-obsidian-border bg-obsidian-light/50 p-6 space-y-4 text-slate-300">
				<ol class="list-decimal list-inside space-y-3">
					<li>{@html m.docs_usage_li1()}</li>
					<li>{@html m.docs_usage_li2()}</li>
					<li>{@html m.docs_usage_li3()}</li>
					<li>{@html m.docs_usage_li4()}</li>
					<li>{@html m.docs_usage_li5()}</li>
				</ol>
			</div>
		</section>

		<!-- Architecture Overview -->
		<section class="space-y-4">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-neon-teal/20 text-neon-teal">
					<Cpu class="w-5 h-5" />
				</div>
				<h2 class="text-2xl font-semibold text-white">{m.docs_arch_title()}</h2>
			</div>
			
			<div class="rounded-xl border border-obsidian-border bg-obsidian-light/50 p-6 space-y-4 text-slate-300">
				<p>{@html m.docs_arch_p1()}</p>
				<p>{@html m.docs_arch_p2()}</p>
			</div>
		</section>
	</div>
</div>
