<script lang="ts">
	import { Trash2, Search, ShieldCheck, Activity } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
</script>

<div class="w-full flex flex-col items-center py-12 space-y-16">
	<!-- Header -->
	<div class="text-center max-w-3xl">
		<h1 class="text-4xl md:text-5xl font-extrabold tracking-tight text-white">
			{m.feat_hero_1()} <br/>
			<span class="text-transparent bg-clip-text bg-gradient-to-r from-neon-teal to-neon-cyan">{m.feat_hero_2()}</span>
		</h1>
		<p class="mt-4 text-lg text-slate-400">
			{m.feat_desc()}
		</p>
	</div>

	<!-- Extended Features Grid -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-8 w-full max-w-5xl">
		
		<!-- Comprehensive Scanning -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-neon-cyan/10 text-neon-cyan">
				<Search class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">{m.feat_item1_title()}</h2>
			<p class="text-slate-400 leading-relaxed">
				{m.feat_item1_desc()}
			</p>
		</div>

		<!-- Cache & Junk Removal -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-neon-magenta/10 text-neon-magenta">
				<Trash2 class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">{m.feat_item2_title()}</h2>
			<p class="text-slate-400 leading-relaxed">
				{m.feat_item2_desc()}
			</p>
		</div>

		<!-- Process Insights -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-neon-teal/10 text-neon-teal">
				<Activity class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">{m.feat_item3_title()}</h2>
			<p class="text-slate-400 leading-relaxed">
				{m.feat_item3_desc()}
			</p>
		</div>

		<!-- Uncompromising Security -->
		<div class="group relative rounded-2xl border border-obsidian-border bg-obsidian-light/40 backdrop-blur-sm p-8 transition-all hover:bg-obsidian-light/60">
			<div class="mb-6 flex h-14 w-14 items-center justify-center rounded-xl bg-white/10 text-white group-hover:text-neon-cyan group-hover:bg-neon-cyan/10 transition-colors">
				<ShieldCheck class="w-7 h-7" />
			</div>
			<h2 class="text-2xl font-bold text-white mb-3">{m.feat_item4_title()}</h2>
			<p class="text-slate-400 leading-relaxed">
				{m.feat_item4_desc()}
			</p>
		</div>

	</div>
</div>
