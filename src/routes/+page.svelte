<script lang="ts">
	import { cleanerStore } from '$lib/stores/cleaner.svelte';
	import NumberFlow from '@number-flow/svelte';
	import { Dialog } from 'bits-ui';
	import { fade, fly } from 'svelte/transition';
	import { 
		HardDrive, 
		Cpu, 
		Search, 
		Trash2, 
		CheckCircle2, 
		Activity, 
		ShieldCheck,
		AlertTriangle
	} from 'lucide-svelte';
	
	let totalSelectedSize = $derived(
		cleanerStore.results
			.filter(r => r.selected)
			.reduce((acc, val) => acc + val.size, 0)
	);

	let isConfirmModalOpen = $state(false);

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

	async function confirmClean() {
		isConfirmModalOpen = false;
		await handleClean();
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
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
		<div class="lg:col-span-2 bg-card border border-border p-6 rounded-xl relative overflow-hidden group hover:border-primary/50 transition-colors shadow-sm">
			<div class="absolute -right-12 -top-12 w-48 h-48 bg-primary/5 rounded-full blur-3xl group-hover:bg-primary/10 transition-colors"></div>
			<div class="relative flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
						<HardDrive class="text-primary w-6 h-6" />
					</div>
					<div>
						<p class="text-sm font-medium text-neutral-400">Storage Free</p>
						<h3 class="text-3xl font-bold tracking-tight flex items-baseline gap-1">
							{#if cleanerStore.stats}
								<NumberFlow value={cleanerStore.stats.disk.free / 1073741824} format={{ maximumFractionDigits: 1 }} />
								<span class="text-xl text-neutral-500 font-medium">GB</span>
							{:else}
								--
							{/if}
						</h3>
					</div>
				</div>
				{#if cleanerStore.stats}
					<div class="text-right">
						<p class="text-sm font-medium text-neutral-400">Total Capacity</p>
						<p class="text-lg font-medium text-foreground">{cleanerStore.stats.disk.total_human}</p>
					</div>
				{/if}
			</div>
			{#if cleanerStore.stats}
			<div class="mt-8 w-full h-2 bg-neutral-800 rounded-full overflow-hidden">
				<div class="h-full bg-primary/80 transition-all duration-1000" style="width: {cleanerStore.stats.disk.percent}%"></div>
			</div>
			{/if}
		</div>

		<div class="bg-card border border-border p-6 rounded-xl flex flex-col justify-between hover:border-primary/50 transition-colors shadow-sm">
			<div class="flex items-center gap-3 mb-4">
				<Activity class="text-primary w-5 h-5" />
				<p class="text-sm font-medium text-neutral-400">Memory Used</p>
			</div>
			<div>
				<h3 class="text-2xl font-bold tracking-tight flex items-baseline gap-1">
					{#if cleanerStore.stats}
						<NumberFlow value={cleanerStore.stats.memory.used / 1073741824} format={{ maximumFractionDigits: 1 }} /> 
						<span class="text-base text-neutral-500 font-medium">/ {cleanerStore.stats.memory.total_human}</span>
					{:else}
						--
					{/if}
				</h3>
			</div>
		</div>

		<div class="bg-card border border-border p-6 rounded-xl flex flex-col justify-between hover:border-primary/50 transition-colors shadow-sm">
			<div class="flex items-center gap-3 mb-4">
				<Cpu class="text-primary w-5 h-5" />
				<p class="text-sm font-medium text-neutral-400">CPU Usage</p>
			</div>
			<div>
				<h3 class="text-2xl font-bold tracking-tight flex items-baseline">
					{#if cleanerStore.stats}
						<NumberFlow value={cleanerStore.stats.cpu_percent} format={{ maximumFractionDigits: 1 }} />
						<span class="text-xl text-neutral-500 font-medium">%</span>
					{:else}
						--%
					{/if}
				</h3>
			</div>
		</div>
	</div>

	<!-- Main Content Area -->
	<div class="flex-1 min-h-0 flex flex-col gap-4">
		{#if cleanerStore.isScanning}
			<div class="bg-card border border-border rounded-xl p-12 flex flex-col items-center justify-center text-center flex-1">
				<div class="relative w-24 h-24 mb-4">
					<div class="absolute inset-0 border-4 border-primary/20 rounded-full"></div>
					<div 
						class="absolute inset-0 border-4 border-primary rounded-full border-t-transparent animate-spin"
					></div>
					<div class="absolute inset-0 flex items-center justify-center text-xl font-bold">
						<NumberFlow value={cleanerStore.progress?.percent || 0} />%
					</div>
				</div>
				<h3 class="text-xl font-semibold mt-6 mb-2">Analyzing Storage...</h3>
				<div class="w-full max-w-md h-2 bg-neutral-800 rounded-full overflow-hidden mt-2 mb-4">
					<div class="h-full bg-primary transition-all duration-300 ease-out" style="width: {cleanerStore.progress?.percent || 0}%"></div>
				</div>
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
					class="bg-red-500/10 hover:bg-red-500/20 text-red-500 border border-red-500/20 px-6 py-2.5 rounded-lg font-medium shadow-sm transition-all active:scale-95 disabled:opacity-50 flex items-center gap-2"
					disabled={totalSelectedSize === 0 || cleanerStore.isCleaning}
					onclick={() => isConfirmModalOpen = true}
				>
					<Trash2 class="w-5 h-5" />
					Clean {formatBytes(totalSelectedSize)}
				</button>
			</div>

			<div class="bg-card border border-border rounded-xl overflow-hidden shadow-sm flex-1 flex flex-col">
				<div class="flex-1 overflow-auto">
					<table class="w-full text-left text-sm whitespace-nowrap min-w-[700px]">
						<thead class="bg-neutral-900/50 sticky top-0 z-10 backdrop-blur-md">
							<tr>
								<th class="px-6 py-4 font-medium text-neutral-400 w-12">
									<input 
										type="checkbox" 
										class="rounded border-border bg-transparent text-primary focus:ring-primary h-4 w-4"
										checked={cleanerStore.results.length > 0 && cleanerStore.results.every(r => r.selected)}
										onchange={(e) => cleanerStore.toggleAll(e.currentTarget.checked)}
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
									<tr class="hover:bg-neutral-900/40 transition-colors" in:fade={{duration: 200}} out:fly={{x: 20, duration: 300}}>
										<td class="px-6 py-4">
											<input 
												type="checkbox" 
												class="rounded border-border bg-transparent text-primary focus:ring-primary h-4 w-4"
												checked={item.selected}
												onchange={(e) => cleanerStore.toggleItem(item.id, e.currentTarget.checked)}
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
					
					{#if cleanerStore.results.filter(r => r.exists && r.size > 0).length === 0 && !cleanerStore.isScanning}
						<div class="text-center py-20 text-neutral-500 flex flex-col items-center justify-center h-full" in:fade={{duration: 400, delay: 300}}>
							<div class="relative mb-6">
								<div class="absolute inset-0 bg-green-500/20 blur-2xl rounded-full"></div>
								<ShieldCheck class="w-20 h-20 text-green-500 relative z-10" />
							</div>
							<h3 class="text-2xl font-semibold text-foreground tracking-tight">System is optimally clean</h3>
							<p class="text-neutral-400 mt-2 max-w-md">No temporary or junk files found. Your storage is healthy and performance is optimal.</p>
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

<Dialog.Root bind:open={isConfirmModalOpen}>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm transition-all" />
		<Dialog.Content class="fixed left-1/2 top-1/2 z-50 -translate-x-1/2 -translate-y-1/2 w-full max-w-lg bg-card border border-border p-6 rounded-2xl shadow-2xl outline-none">
			<div class="flex items-start gap-4">
				<div class="w-12 h-12 bg-red-500/10 rounded-full flex items-center justify-center shrink-0">
					<AlertTriangle class="text-red-500 w-6 h-6" />
				</div>
				<div>
					<Dialog.Title class="text-xl font-semibold text-foreground">Confirm Deletion</Dialog.Title>
					<Dialog.Description class="text-neutral-400 mt-2 text-sm leading-relaxed">
						You are about to permanently delete <strong>{cleanerStore.results.filter(r => r.selected).length}</strong> items, freeing up <strong>{formatBytes(totalSelectedSize)}</strong> of disk space. This action cannot be undone. Are you sure you want to proceed?
					</Dialog.Description>
				</div>
			</div>
			<div class="flex justify-end gap-3 mt-8">
				<Dialog.Close class="px-5 py-2.5 hover:bg-neutral-800 rounded-lg text-foreground font-medium transition-colors">Cancel</Dialog.Close>
				<button 
					class="px-5 py-2.5 bg-red-500 hover:bg-red-600 text-white rounded-lg font-medium shadow-md shadow-red-500/20 transition-all active:scale-95" 
					onclick={confirmClean}
				>
					Yes, Clean Now
				</button>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
