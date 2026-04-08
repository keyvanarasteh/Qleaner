<script lang="ts">
	import { cleanerStore } from '$lib/stores/cleaner.svelte';
	import { 
		HardDrive, 
		Cpu, 
		Search, 
		Trash2, 
		CheckCircle2, 
		Activity, 
		ShieldCheck 
	} from 'lucide-svelte';
	
	let totalSelectedSize = $derived(
		cleanerStore.results
			.filter(r => r.selected)
			.reduce((acc, val) => acc + val.size, 0)
	);

	function formatBytes(bytes: number, decimals = 2) {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
	}

	async function handleClean() {
		const selectedIds = cleanerStore.results.filter(r => r.selected).map(r => r.id);
		if (selectedIds.length > 0) {
			await cleanerStore.cleanItems(selectedIds);
		}
	}
</script>

<div class="flex-1 flex flex-col p-8 gap-8 overflow-y-auto w-full h-full">
	
	<!-- Header -->
	<header class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
				<Activity class="w-8 h-8 text-primary" />
				Qleaner Workspace
			</h1>
			<p class="text-neutral-400 mt-2">Accelerate your system securely across all platforms.</p>
		</div>
		<button 
			onclick={() => cleanerStore.startScan()} 
			disabled={cleanerStore.isScanning || cleanerStore.isCleaning}
			class="bg-primary hover:bg-primary/90 text-primary-foreground px-6 py-3 rounded-lg font-medium shadow-md shadow-primary/20 transition-all active:scale-95 disabled:opacity-50 flex items-center gap-2"
		>
			<Search class="w-5 h-5" />
			{cleanerStore.isScanning ? 'Scanning...' : 'Smart Scan'}
		</button>
	</header>

	<!-- Stats Row -->
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		<div class="bg-card border border-border p-6 rounded-xl flex items-center gap-4 hover:border-primary/50 transition-colors shadow-sm">
			<div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
				<Cpu class="text-primary w-6 h-6" />
			</div>
			<div>
				<p class="text-sm font-medium text-neutral-400">CPU Usage</p>
				<h3 class="text-2xl font-bold tracking-tight">
					{cleanerStore.stats ? cleanerStore.stats.cpu_percent.toFixed(1) : '--'}%
				</h3>
			</div>
		</div>
		
		<div class="bg-card border border-border p-6 rounded-xl flex items-center gap-4 hover:border-primary/50 transition-colors shadow-sm">
			<div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
				<Activity class="text-primary w-6 h-6" />
			</div>
			<div>
				<p class="text-sm font-medium text-neutral-400">Memory Used</p>
				<h3 class="text-2xl font-bold tracking-tight">
					{cleanerStore.stats ? cleanerStore.stats.memory.used_human : '--'}
					<span class="text-sm font-normal text-neutral-500">of {cleanerStore.stats?.memory.total_human || '--'}</span>
				</h3>
			</div>
		</div>

		<div class="bg-card border border-border p-6 rounded-xl flex items-center gap-4 hover:border-primary/50 transition-colors shadow-sm">
			<div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
				<HardDrive class="text-primary w-6 h-6" />
			</div>
			<div>
				<p class="text-sm font-medium text-neutral-400">Storage Free</p>
				<h3 class="text-2xl font-bold tracking-tight">
					{cleanerStore.stats ? cleanerStore.stats.disk.free_human : '--'}
				</h3>
			</div>
		</div>
	</div>

	<!-- Main Content Area -->
	<div class="flex-1 min-h-0 flex flex-col gap-4">
		{#if cleanerStore.isScanning}
			<div class="bg-card border border-border rounded-xl p-8 flex flex-col items-center justify-center text-center">
				<div class="relative w-24 h-24">
					<div class="absolute inset-0 border-4 border-primary/20 rounded-full"></div>
					<div 
						class="absolute inset-0 border-4 border-primary rounded-full border-t-transparent animate-spin"
					></div>
					<div class="absolute inset-0 flex items-center justify-center text-xl font-bold">
						{cleanerStore.progress?.percent || 0}%
					</div>
				</div>
				<h3 class="text-xl font-semibold mt-6 mb-2">Analyzing Storage...</h3>
				<p class="text-neutral-400 text-sm max-w-md truncate">
					{cleanerStore.progress?.current_location || 'Initializing...'}
				</p>
			</div>
		{:else if cleanerStore.results.length > 0}
			<div class="flex items-center justify-between mt-4">
				<h2 class="text-xl font-semibold flex items-center gap-2">
					<ShieldCheck class="w-6 h-6 text-green-500" />
					Found {cleanerStore.results.filter(r => r.exists && r.size > 0).length} Junks
				</h2>
				<button 
					class="bg-red-500 hover:bg-red-600 text-white px-6 py-2.5 rounded-lg font-medium shadow-md transition-all active:scale-95 disabled:opacity-50 flex items-center gap-2"
					disabled={totalSelectedSize === 0 || cleanerStore.isCleaning}
					onclick={handleClean}
				>
					<Trash2 class="w-5 h-5" />
					Clean {formatBytes(totalSelectedSize)}
				</button>
			</div>

			<div class="bg-card border border-border rounded-xl overflow-hidden shadow-sm flex-1">
				<div class="max-h-full overflow-y-auto">
					<table class="w-full text-left text-sm whitespace-nowrap">
						<thead class="bg-neutral-900/50 sticky top-0 z-10 backdrop-blur-md">
							<tr>
								<th class="px-6 py-4 font-medium text-neutral-400 w-12">
									<input 
										type="checkbox" 
										class="rounded border-border bg-transparent text-primary focus:ring-primary h-4 w-4"
										checked={cleanerStore.results.every(r => r.selected)}
										onchange={(e) => {
											const checked = e.currentTarget.checked;
											cleanerStore.results.forEach(r => r.selected = checked);
										}}
									/>
								</th>
								<th class="px-6 py-4 font-medium text-neutral-400">Target</th>
								<th class="px-6 py-4 font-medium text-neutral-400">Category</th>
								<th class="px-6 py-4 font-medium text-neutral-400 text-right">Size</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-border">
							{#each cleanerStore.results as item (item.id)}
								{#if item.exists && item.size > 0}
									<tr class="hover:bg-neutral-900/40 transition-colors">
										<td class="px-6 py-4">
											<input 
												type="checkbox" 
												class="rounded border-border bg-transparent text-primary focus:ring-primary h-4 w-4"
												bind:checked={item.selected}
											/>
										</td>
										<td class="px-6 py-4">
											<div class="flex flex-col">
												<span class="font-medium text-foreground">{item.name}</span>
												<span class="text-neutral-500 text-xs truncate max-w-sm" title={item.path}>{item.path}</span>
											</div>
										</td>
										<td class="px-6 py-4 text-neutral-400">{item.category}</td>
										<td class="px-6 py-4 text-right font-medium text-foreground">{item.size_human}</td>
									</tr>
								{/if}
							{/each}
						</tbody>
					</table>
					
					{#if cleanerStore.results.filter(r => r.exists && r.size > 0).length === 0}
						<div class="text-center py-16 text-neutral-500 flex flex-col items-center">
							<CheckCircle2 class="w-16 h-16 mb-4 opacity-50" />
							<p class="text-lg">Your system is optimally clean.</p>
						</div>
					{/if}
				</div>
			</div>
		{:else}
			<div class="bg-card border border-border rounded-xl p-16 flex flex-col items-center justify-center text-center flex-1">
				<Search class="w-16 h-16 text-neutral-600 mb-6" />
				<h3 class="text-2xl font-semibold mt-2 mb-2">Ready to Clean</h3>
				<p class="text-neutral-400 max-w-md">
					Click Smart Scan to discover space-saving opportunities and maintain optimal performance across your environment.
				</p>
			</div>
		{/if}
	</div>
</div>
