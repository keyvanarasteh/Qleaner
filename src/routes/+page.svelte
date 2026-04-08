<script lang="ts">
	import { cleanerStore } from '$lib/stores/cleaner.svelte';
	import NumberFlow from '@number-flow/svelte';
	import { Dialog, DropdownMenu } from 'bits-ui';
	import { fade, fly } from 'svelte/transition';
	import TreemapWidget from '$lib/components/ui/TreemapWidget.svelte';
	import { 
		HardDrive, 
		Cpu, 
		Search, 
		Trash2, 
		CheckCircle2, 
		Activity, 
		ShieldCheck,
		ShieldAlert,
		AlertTriangle,
		ArrowDownWideNarrow,
		ArrowUpNarrowWide,
		MoreHorizontal,
		FolderOpen,
		EyeOff,
		Info,
		XOctagon,
		Globe
	} from 'lucide-svelte';
	
	let totalSelectedSize = $derived(
		cleanerStore.results
			.filter(r => r.selected)
			.reduce((acc, val) => acc + val.size, 0)
	);

	let isConfirmModalOpen = $state(false);
	
	let sortKey = $state<'name' | 'category' | 'size'>('size');
	let sortAsc = $state(false);

	let sortedResults = $derived(
		[...cleanerStore.results].sort((a, b) => {
			const valA = a[sortKey];
			const valB = b[sortKey];
			if (typeof valA === 'string' && typeof valB === 'string') {
				return sortAsc ? valA.localeCompare(valB) : valB.localeCompare(valA);
			}
			if (typeof valA === 'number' && typeof valB === 'number') {
				return sortAsc ? valA - valB : valB - valA;
			}
			return 0;
		})
	);

	let rowHeight = 73;
	let scrollY = $state(0);
	let viewportHeight = $state(600);
	
	let activeResults = $derived(sortedResults.filter(i => i.exists && i.size > 0).map((r, idx) => ({ ...r, virtualIndex: idx })));
	let startIndex = $derived(Math.max(0, Math.floor(scrollY / rowHeight) - 5));
	let endIndex = $derived(Math.min(activeResults.length, Math.ceil((scrollY + viewportHeight) / rowHeight) + 5));
	let virtualResults = $derived(activeResults.slice(startIndex, endIndex));

	function toggleSort(key: 'name' | 'category' | 'size') {
		if (sortKey === key) {
			sortAsc = !sortAsc;
		} else {
			sortKey = key;
			sortAsc = key === 'size' ? false : true;
		}
	}

	function formatBytes(bytes: number, decimals = 2) {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
	}

	function truncatePath(path: string, maxLength: number = 60) {
		if (path.length <= maxLength) return path;
		const start = path.slice(0, Math.floor(maxLength / 2) - 3);
		const end = path.slice(-Math.floor(maxLength / 2) + 2);
		return `${start}...${end}`;
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

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && cleanerStore.isScanning) {
			cleanerStore.abortScan();
		}
		if (e.key === 'Enter' && (e.metaKey || e.ctrlKey) && !cleanerStore.isCleaning && totalSelectedSize > 0) {
			isConfirmModalOpen = true;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

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
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6">
		<div class="lg:col-span-2 bg-card border border-border p-6 rounded-xl relative overflow-hidden group hover:border-primary/50 transition-colors shadow-sm flex flex-col justify-between">
			<div class="absolute -right-12 -top-12 w-48 h-48 bg-primary/5 rounded-full blur-3xl group-hover:bg-primary/10 transition-colors"></div>
			<div class="relative flex items-center justify-between mb-8">
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
						<HardDrive class="text-primary w-6 h-6" />
					</div>
					<div>
						<p class="text-sm font-medium text-neutral-400">Total System Capacity</p>
						<h3 class="text-3xl font-bold tracking-tight flex items-baseline gap-1">
							{#if cleanerStore.stats}
								<NumberFlow value={cleanerStore.stats.disk.total / 1073741824} format={{ maximumFractionDigits: 1 }} />
								<span class="text-xl text-neutral-500 font-medium">GB</span>
							{:else}
								--
							{/if}
						</h3>
					</div>
				</div>
			</div>
			
			{#if cleanerStore.results.filter(r => r.size > 0 && r.exists).length > 0}
				<div class="w-full relative z-10 pt-4 border-t border-border/50 flex-1 flex flex-col justify-end">
					<p class="text-sm font-medium text-neutral-400 mb-4 px-1">Segmented Optimization Payloads</p>
					<TreemapWidget 
						items={
							Object.entries(
								cleanerStore.results
									.filter(r => r.size > 0 && r.exists)
									.reduce((acc, curr) => { 
										acc[curr.category] = (acc[curr.category] || 0) + curr.size; 
										return acc; 
									}, {} as Record<string, number>)
							).map(([name, size]) => ({ name, category: name, size })).sort((a,b) => b.size - a.size)
						} 
					/>
				</div>
			{:else if cleanerStore.stats}
				<div class="w-full relative z-10 flex-1 flex flex-col justify-end">
					<div class="flex items-center justify-between text-xs text-neutral-500 mb-2 font-mono px-1">
						<span>Used: {cleanerStore.stats.disk.used_human}</span>
						<span>Free: {cleanerStore.stats.disk.free_human}</span>
					</div>
					<div class="w-full h-3 bg-neutral-800 rounded-full overflow-hidden shadow-inner">
						<div class="h-full bg-primary/80 relative" style="width: {cleanerStore.stats.disk.percent}%">
							<div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/10 to-transparent w-[200%] animate-[shimmer_2s_infinite]"></div>
						</div>
					</div>
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
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-3">
					<Cpu class="text-primary w-5 h-5" />
					<p class="text-sm font-medium text-neutral-400">CPU Status</p>
				</div>
				{#if cleanerStore.stats}
					<span class="text-xs font-semibold bg-primary/20 text-primary px-2 py-1 rounded-md">
						{cleanerStore.stats.cpu_temp}°C
					</span>
				{/if}
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

		<div class="bg-card border border-border p-6 rounded-xl flex flex-col justify-between hover:border-primary/50 transition-colors shadow-sm">
			<div class="flex items-center gap-3 mb-4">
				<Globe class="text-primary w-5 h-5" />
				<p class="text-sm font-medium text-neutral-400">Network Link</p>
			</div>
			<div>
				<div class="flex items-center justify-between">
					<div class="flex flex-col">
						<span class="text-xs text-neutral-500 font-medium">Out</span>
						<span class="text-base font-bold tracking-tight text-foreground">
							{#if cleanerStore.stats}
								{cleanerStore.stats.network.tx_human}
							{:else}
								--
							{/if}
						</span>
					</div>
					<div class="flex flex-col text-right">
						<span class="text-xs text-neutral-500 font-medium">In</span>
						<span class="text-base font-bold tracking-tight text-foreground">
							{#if cleanerStore.stats}
								{cleanerStore.stats.network.rx_human}
							{:else}
								--
							{/if}
						</span>
					</div>
				</div>
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
				<button 
					class="mt-8 border border-red-500/20 text-red-500 hover:bg-red-500/10 px-6 py-2.5 rounded-lg font-medium transition-colors flex items-center gap-2"
					onclick={() => cleanerStore.abortScan()}
				>
					<XOctagon size={18} />
					Abort Operation
				</button>
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

			<div class="bg-card border border-border rounded-xl overflow-hidden shadow-sm flex-1 flex flex-col relative w-full">
				<div class="flex-1 overflow-auto" onscroll={(e) => scrollY = e.currentTarget.scrollTop} bind:clientHeight={viewportHeight}>
					{#if activeResults.length > 0}
					<div role="table" class="w-full text-left text-sm whitespace-nowrap min-w-[700px] flex flex-col">
						<div class="bg-neutral-900/50 sticky top-0 z-20 backdrop-blur-md grid grid-cols-[4rem_minmax(0,1fr)_12rem_8rem_4rem] border-b border-border items-center">
							<div class="px-6 py-4 font-medium text-neutral-400">
								<input 
									type="checkbox" 
									aria-label="Select all targets"
									class="rounded border-border bg-transparent text-primary focus:ring-primary h-4 w-4"
									checked={cleanerStore.results.length > 0 && cleanerStore.results.every(r => r.selected)}
									onchange={(e) => cleanerStore.toggleAll(e.currentTarget.checked)}
								/>
							</div>
							<button class="px-6 py-4 font-medium text-neutral-400 hover:text-foreground transition-colors group select-none flex items-center justify-start gap-2" onclick={() => toggleSort('name')}>
								Target {#if sortKey === 'name'} <span class="text-primary">{#if sortAsc}<ArrowUpNarrowWide size={14}/>{:else}<ArrowDownWideNarrow size={14}/>{/if}</span> {/if}
							</button>
							<button class="px-6 py-4 font-medium text-neutral-400 hover:text-foreground transition-colors group select-none flex items-center justify-start gap-2" onclick={() => toggleSort('category')}>
								Category {#if sortKey === 'category'} <span class="text-primary">{#if sortAsc}<ArrowUpNarrowWide size={14}/>{:else}<ArrowDownWideNarrow size={14}/>{/if}</span> {/if}
							</button>
							<button class="px-6 py-4 font-medium text-neutral-400 hover:text-foreground transition-colors group select-none flex items-center justify-end gap-2 text-right" onclick={() => toggleSort('size')}>
								Size {#if sortKey === 'size'} <span class="text-primary">{#if sortAsc}<ArrowUpNarrowWide size={14}/>{:else}<ArrowDownWideNarrow size={14}/>{/if}</span> {/if}
							</button>
							<div class="px-6 py-4 text-center"></div>
						</div>
						<div class="relative" style="height: {activeResults.length * rowHeight}px;">
							{#each virtualResults as item (item.id)}
								<div class="absolute w-full hover:bg-neutral-900/40 transition-colors grid grid-cols-[4rem_minmax(0,1fr)_12rem_8rem_4rem] items-center border-b border-border/50 group" style="top: {item.virtualIndex * rowHeight}px; height: {rowHeight}px;">
									<div class="px-6 py-4">
										<input 
											type="checkbox" 
											aria-label="Select {item.name}"
											class="rounded border-border bg-transparent text-primary focus:ring-primary h-4 w-4"
											checked={item.selected}
											onchange={(e) => cleanerStore.toggleItem(item.id, e.currentTarget.checked)}
										/>
									</div>
									<div class="px-6 py-4 flex flex-col justify-center overflow-hidden">
										<span class="font-medium text-foreground truncate">{item.name}</span>
										<span class="text-neutral-500 text-xs truncate" title={item.path}>{truncatePath(item.path)}</span>
									</div>
									<div class="px-6 py-4 flex items-center">
										{#if item.category.toLowerCase().includes('privacy') || item.risk === 'High'}
											<span class="px-2.5 py-1 text-xs font-semibold rounded-md bg-purple-500/10 text-purple-400 border border-purple-500/20 shadow-sm flex items-center w-max gap-1">
												<ShieldAlert size={12}/> {item.category}
											</span>
										{:else if item.category.toLowerCase().includes('system')}
											<span class="px-2.5 py-1 text-xs font-medium rounded-md bg-neutral-800 text-neutral-400 border border-border shadow-sm w-max inline-block">
												{item.category}
											</span>
										{:else}
											<span class="px-2.5 py-1 text-xs font-medium rounded-md bg-primary/10 text-primary border border-primary/20 shadow-sm w-max inline-block">
												{item.category}
											</span>
										{/if}
									</div>
									<div class="px-6 py-4 text-right font-medium text-foreground">{item.size_human}</div>
									<div class="px-6 py-4 flex justify-end">
										<DropdownMenu.Root>
											<DropdownMenu.Trigger class="p-2 hover:bg-neutral-800 rounded-md transition-colors text-neutral-400 hover:text-foreground">
												<MoreHorizontal size={16} />
											</DropdownMenu.Trigger>
											<DropdownMenu.Content class="w-48 bg-card border border-border rounded-xl shadow-xl py-1 z-50 overflow-hidden">
												<DropdownMenu.Item class="px-3 py-2 text-sm text-neutral-300 hover:bg-primary/20 hover:text-primary cursor-pointer flex items-center gap-2 outline-none transition-colors" onclick={() => cleanerStore.openFolder(item.path)}>
													<FolderOpen size={14} /> Open Location
												</DropdownMenu.Item>
												<DropdownMenu.Item class="px-3 py-2 text-sm text-neutral-300 hover:bg-neutral-800 hover:text-foreground cursor-pointer flex items-center gap-2 outline-none transition-colors" onclick={() => cleanerStore.ignoreItem(item.id)}>
													<EyeOff size={14} /> Add to Ignore List
												</DropdownMenu.Item>
												<DropdownMenu.Separator class="h-px bg-border my-1" />
												<DropdownMenu.Item class="px-3 py-2 text-sm text-neutral-300 hover:bg-neutral-800 hover:text-foreground cursor-pointer flex items-center gap-2 outline-none transition-colors">
													<Info size={14} /> View Properties
												</DropdownMenu.Item>
											</DropdownMenu.Content>
										</DropdownMenu.Root>
									</div>
								</div>
							{/each}
						</div>
					</div>
					{/if}
					
					{#if activeResults.length === 0 && !cleanerStore.isScanning}
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
