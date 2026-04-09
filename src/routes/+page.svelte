<script lang="ts">
	import { cleanerStore } from '$lib/stores/cleaner.svelte';
	import NumberFlow from '@number-flow/svelte';
	import { Dialog, DropdownMenu } from 'bits-ui';
	import { fade } from 'svelte/transition';
	import { 
		HardDrive, Cpu, Search, Trash2, Activity, ShieldCheck,
		AlertTriangle, ArrowDownWideNarrow, ArrowUpNarrowWide,
		MoreHorizontal, FolderOpen, EyeOff, Info, XOctagon,
		Globe, Terminal, MemoryStick, Wifi
	} from 'lucide-svelte';
	
	let totalSelectedSize = $derived(
		cleanerStore.results
			.filter(r => r.selected)
			.reduce((acc, val) => acc + val.size, 0)
	);

	let isConfirmModalOpen = $state(false);
	let sortKey = $state<'name' | 'category' | 'size'>('size');
	let sortAsc = $state(false);

	let sortedResults = $derived.by(() => {
		if (cleanerStore.isScanning || cleanerStore.isCleaning) {
			return cleanerStore.results;
		}
		return [...cleanerStore.results].sort((a, b) => {
			const valA = a[sortKey];
			const valB = b[sortKey];
			if (typeof valA === 'string' && typeof valB === 'string') {
				return sortAsc ? valA.localeCompare(valB) : valB.localeCompare(valA);
			}
			if (typeof valA === 'number' && typeof valB === 'number') {
				return sortAsc ? valA - valB : valB - valA;
			}
			return 0;
		});
	});

	let rowHeight = 36;
	let scrollY = $state(0);
	let viewportHeight = $state(400);
	
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

	function formatBytes(bytes: number, decimals = 1) {
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

	let elapsedSec = $state(0);
	$effect(() => {
		if (cleanerStore.isScanning || cleanerStore.isCleaning) {
			const timer = setInterval(() => {
				elapsedSec = Math.floor((Date.now() - cleanerStore.scanStartMs) / 1000);
			}, 1000);
			return () => clearInterval(timer);
		} else {
			elapsedSec = 0;
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && (cleanerStore.isScanning || cleanerStore.isCleaning)) {
			cleanerStore.abortScan();
		}
		if (e.key === 'Enter' && (e.metaKey || e.ctrlKey) && !cleanerStore.isCleaning && totalSelectedSize > 0) {
			isConfirmModalOpen = true;
		}
	}

	let selectedCount = $derived(cleanerStore.results.filter(r => r.selected).length);
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex-1 flex flex-col overflow-hidden bg-background h-full text-foreground">
	
	<!-- ─── Compact Toolbar ─── -->
	<div class="shrink-0 border-b border-border bg-card/50">
		<!-- Row 1: Title + Actions -->
		<div class="flex items-center justify-between gap-3 px-3 py-2">
			<div class="flex items-center gap-2 min-w-0">
				<HardDrive size={14} class="text-primary shrink-0" />
				<span class="text-xs font-bold uppercase tracking-widest text-foreground truncate">Deep Space Purge</span>
			</div>
			
			<div class="flex items-center gap-2 shrink-0">
				<button 
					onclick={() => isConfirmModalOpen = true}
					disabled={totalSelectedSize === 0 || cleanerStore.isCleaning || cleanerStore.isScanning}
					class="bg-destructive/10 hover:bg-destructive/20 text-destructive border border-destructive/20 px-2.5 py-1 rounded text-xs font-semibold transition-all active:scale-95 disabled:opacity-40 flex items-center gap-1.5"
					title="Delete selected items"
					aria-label="Shred selected items"
				>
					<Trash2 size={12} />
					<span class="hidden sm:inline">SHRED</span>
					{formatBytes(totalSelectedSize)}
				</button>
				<button 
					onclick={() => cleanerStore.startScan()} 
					disabled={cleanerStore.isScanning || cleanerStore.isCleaning}
					class="bg-primary/10 hover:bg-primary/20 border border-primary/20 text-primary px-2.5 py-1 rounded text-xs font-semibold transition-all active:scale-95 disabled:opacity-40 flex items-center gap-1.5"
					title="Start deep scan"
					aria-label="Start deep scan"
				>
					<Search size={12} />
					{cleanerStore.isScanning ? 'SCANNING...' : 'DEEP SCAN'}
				</button>
			</div>
		</div>

		<!-- Row 2: System stats (compact, wrapping) -->
		{#if cleanerStore.stats}
			<div class="flex flex-wrap items-center gap-x-4 gap-y-1 px-3 pb-2 text-[11px] font-mono text-muted-foreground">
				<div class="flex items-center gap-1.5" title="Disk usage">
					<HardDrive size={10} class="text-primary/70" />
					<span>{cleanerStore.stats.disk.used_human}<span class="opacity-50">/{cleanerStore.stats.disk.total_human}</span></span>
				</div>
				<div class="flex items-center gap-1.5" title="RAM usage">
					<Activity size={10} class="text-primary/70" />
					<span>RAM <NumberFlow value={cleanerStore.stats.memory.used / 1073741824} format={{ maximumFractionDigits: 1 }} />G</span>
				</div>
				<div class="flex items-center gap-1.5" title="Network I/O">
					<Globe size={10} class="text-primary/70" />
					<span>{cleanerStore.stats.network.tx_human}↑ {cleanerStore.stats.network.rx_human}↓</span>
				</div>
				{#if selectedCount > 0}
					<div class="flex items-center gap-1.5 text-primary">
						<span>{selectedCount} selected</span>
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- ─── Scan Progress Banner ─── -->
	{#if cleanerStore.isScanning || cleanerStore.isCleaning}
		<div class="shrink-0 bg-card border-b border-border px-3 py-2.5 relative" in:fade={{duration: 150}} out:fade={{duration: 150}}>
			<div class="flex items-center justify-between gap-3 mb-2 text-xs">
				<div class="flex items-center gap-2 min-w-0">
					<div class="w-2 h-2 rounded-full bg-primary animate-pulse shrink-0"></div>
					<span class="font-semibold text-foreground truncate">
						{cleanerStore.isCleaning ? 'SECURE_SHRED' : 'TRACE_SCAN'}
					</span>
					<span class="text-muted-foreground truncate text-[11px] hidden sm:inline">
						{cleanerStore.progress?.current_location || 'Initializing...'}
					</span>
				</div>
				<div class="flex items-center gap-3 text-muted-foreground shrink-0 text-[11px] font-mono">
					<span>Found: <span class="text-primary font-bold"><NumberFlow value={cleanerStore.progress?.found_count || 0} /></span></span>
					<span class="hidden sm:inline">{formatBytes(cleanerStore.progress?.total_size || 0)}</span>
					<span>{elapsedSec}s</span>
					<button 
						class="text-destructive hover:text-destructive/80 bg-destructive/10 px-1.5 py-0.5 rounded text-[10px] font-bold flex items-center gap-1"
						onclick={() => cleanerStore.abortScan()}
						title="Abort scan"
						aria-label="Abort scan"
					>
						<XOctagon size={10} /> HALT
					</button>
				</div>
			</div>
			<div class="w-full h-1 bg-muted rounded-full overflow-hidden">
				<div class="h-full bg-primary transition-all duration-300 ease-out rounded-full" style="width: {cleanerStore.progress?.percent || 0}%"></div>
			</div>
		</div>
	{/if}

	<!-- ─── Main Content Area ─── -->
	<div class="flex-1 min-h-0 flex flex-col relative">
		
		{#if !cleanerStore.isScanning && activeResults.length === 0}
			<!-- Empty State -->
			<div class="flex-1 flex flex-col items-center justify-center text-muted-foreground p-6 text-center">
				{#if cleanerStore.results.length === 0}
					<div class="w-16 h-16 rounded-2xl bg-card border border-border flex items-center justify-center mb-4">
						<Terminal size={28} class="opacity-40" />
					</div>
					<h3 class="text-sm font-semibold text-foreground mb-1">Awaiting Commands</h3>
					<p class="text-xs max-w-xs leading-relaxed">No data paths mapped. Click <strong>DEEP SCAN</strong> or press <kbd class="px-1 py-0.5 bg-card border border-border rounded text-[10px]">Ctrl+Enter</kbd> to begin.</p>
				{:else}
					<div class="w-16 h-16 rounded-2xl bg-success/10 border border-success/20 flex items-center justify-center mb-4">
						<ShieldCheck size={28} class="text-success" />
					</div>
					<h3 class="text-sm font-semibold text-foreground mb-1">System Optimal</h3>
					<p class="text-xs max-w-xs leading-relaxed">All target paths are clean. No remaining orphan structures.</p>
				{/if}
			</div>
		{:else if activeResults.length > 0}
			<!-- Table Header -->
			<div class="shrink-0 bg-card/80 border-b border-border sticky top-0 z-20 flex items-center text-[10px] font-bold tracking-wider text-muted-foreground uppercase h-7 select-none">
				<div class="w-9 shrink-0 flex items-center justify-center border-r border-border/30 h-full">
					<input 
						type="checkbox" 
						aria-label="Select all targets"
						class="rounded bg-background text-primary border border-border h-3 w-3 focus:ring-0 focus:ring-offset-0 disabled:opacity-40"
						disabled={cleanerStore.isScanning || cleanerStore.isCleaning}
						checked={cleanerStore.results.length > 0 && cleanerStore.results.every(r => r.selected)}
						onchange={(e) => cleanerStore.toggleAll(e.currentTarget.checked)}
					/>
				</div>
				<button 
					disabled={cleanerStore.isScanning || cleanerStore.isCleaning} 
					class="flex-1 min-w-0 px-2 h-full hover:text-foreground text-left border-r border-border/30 focus:outline-none flex items-center gap-1 transition-colors"
					onclick={() => toggleSort('name')}
					aria-label="Sort by target path"
				>
					NAME
					{#if sortKey === 'name'}
						<span class="text-primary">{#if sortAsc}<ArrowUpNarrowWide size={10}/>{:else}<ArrowDownWideNarrow size={10}/>{/if}</span>
					{/if}
				</button>
				<button 
					disabled={cleanerStore.isScanning || cleanerStore.isCleaning} 
					class="w-24 shrink-0 px-2 h-full hover:text-foreground text-left border-r border-border/30 focus:outline-none flex items-center gap-1 transition-colors hidden md:flex"
					onclick={() => toggleSort('category')}
					aria-label="Sort by category"
				>
					TYPE
					{#if sortKey === 'category'}
						<span class="text-primary">{#if sortAsc}<ArrowUpNarrowWide size={10}/>{:else}<ArrowDownWideNarrow size={10}/>{/if}</span>
					{/if}
				</button>
				<button 
					disabled={cleanerStore.isScanning || cleanerStore.isCleaning} 
					class="w-20 shrink-0 px-2 h-full hover:text-foreground text-right border-r border-border/30 focus:outline-none flex items-center justify-end gap-1 transition-colors"
					onclick={() => toggleSort('size')}
					aria-label="Sort by size"
				>
					SIZE
					{#if sortKey === 'size'}
						<span class="text-primary">{#if sortAsc}<ArrowUpNarrowWide size={10}/>{:else}<ArrowDownWideNarrow size={10}/>{/if}</span>
					{/if}
				</button>
				<div class="w-8 shrink-0"></div>
			</div>

			<!-- Virtual Scroll Area -->
			<div 
				class="flex-1 scroll-optimized relative" 
				onscroll={(e) => scrollY = e.currentTarget.scrollTop} 
				bind:clientHeight={viewportHeight}
			>
				<div class="absolute w-full" style="height: {activeResults.length * rowHeight}px;">
					{#each virtualResults as item (item.id)}
						<div 
							class="absolute w-full hover:bg-accent/50 transition-colors flex items-center border-b border-border/10 group cursor-default" 
							style="top: {item.virtualIndex * rowHeight}px; height: {rowHeight}px;"
						>
							<!-- Checkbox -->
							<div class="w-9 shrink-0 flex items-center justify-center h-full">
								<input 
									type="checkbox" 
									aria-label="Select {item.name}"
									class="rounded bg-background text-primary border border-border h-3 w-3 focus:ring-0"
									checked={item.selected}
									onchange={(e) => cleanerStore.toggleItem(item.id, e.currentTarget.checked)}
								/>
							</div>
							
							<!-- Name + Path -->
							<div class="flex-1 min-w-0 px-2 flex items-center gap-2 h-full overflow-hidden">
								<span class="font-medium text-foreground text-xs truncate shrink-0 max-w-[40%]">{item.name}</span>
								<span class="text-muted-foreground text-[10px] truncate opacity-60" title={item.path}>{item.path}</span>
							</div>
							
							<!-- Category (hidden on small) -->
							<div class="w-24 shrink-0 px-2 items-center h-full hidden md:flex">
								<span class="text-[10px] flex items-center gap-1.5 font-medium text-muted-foreground">
									{#if item.category.toLowerCase().includes('privacy')}
										<div class="w-1.5 h-1.5 rounded-full bg-purple-500"></div>
									{:else if item.category.toLowerCase().includes('system')}
										<div class="w-1.5 h-1.5 rounded-full bg-muted-foreground"></div>
									{:else}
										<div class="w-1.5 h-1.5 rounded-full bg-primary"></div>
									{/if}
									<span class="truncate">{item.category}</span>
								</span>
							</div>
							
							<!-- Size -->
							<div class="w-20 shrink-0 px-2 flex items-center justify-end h-full text-xs text-primary font-medium font-mono">
								{item.size_human}
							</div>
							
							<!-- Actions -->
							<div class="w-8 shrink-0 flex items-center justify-center h-full">
								<DropdownMenu.Root>
									<DropdownMenu.Trigger 
										class="text-muted-foreground hover:text-foreground hover:bg-muted p-1 rounded transition-colors opacity-0 group-hover:opacity-100" 
										disabled={cleanerStore.isScanning || cleanerStore.isCleaning}
										aria-label="Row actions for {item.name}"
									>
										<MoreHorizontal size={12} />
									</DropdownMenu.Trigger>
									<DropdownMenu.Content class="w-44 bg-card border border-border rounded-md shadow-xl py-1 z-50 text-xs">
										<DropdownMenu.Item 
											class="px-3 py-1.5 hover:bg-accent hover:text-accent-foreground cursor-pointer flex items-center gap-2 outline-none" 
											onclick={() => cleanerStore.openFolder(item.path)}
										>
											<FolderOpen size={12} /> Open Directory
										</DropdownMenu.Item>
										<DropdownMenu.Item 
											class="px-3 py-1.5 hover:bg-accent hover:text-accent-foreground cursor-pointer flex items-center gap-2 outline-none" 
											onclick={() => cleanerStore.ignoreItem(item.id)}
										>
											<EyeOff size={12} /> Exclude
										</DropdownMenu.Item>
										<DropdownMenu.Separator class="h-px bg-border my-1" />
										<DropdownMenu.Item class="px-3 py-1.5 hover:bg-accent hover:text-accent-foreground cursor-pointer flex items-center gap-2 outline-none">
											<Info size={12} /> Details
										</DropdownMenu.Item>
									</DropdownMenu.Content>
								</DropdownMenu.Root>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Status Bar -->
			<div class="shrink-0 h-6 bg-card/50 border-t border-border flex items-center justify-between px-3 text-[10px] text-muted-foreground font-mono select-none">
				<span>{activeResults.length} items • {selectedCount} selected</span>
				<span>{formatBytes(totalSelectedSize)} marked for removal</span>
			</div>
		{/if}
	</div>
</div>

<!-- ─── Confirm Modal ─── -->
<Dialog.Root bind:open={isConfirmModalOpen}>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm" />
		<Dialog.Content class="fixed left-1/2 top-1/2 z-50 -translate-x-1/2 -translate-y-1/2 w-[calc(100%-2rem)] max-w-md bg-card border border-border p-5 rounded-lg shadow-2xl outline-none">
			<div class="flex items-start gap-3">
				<div class="w-9 h-9 rounded-lg bg-destructive/10 flex items-center justify-center shrink-0">
					<AlertTriangle class="text-destructive" size={18} />
				</div>
				<div class="min-w-0">
					<Dialog.Title class="text-sm font-bold text-foreground">Confirm Purge</Dialog.Title>
					<Dialog.Description class="text-xs text-muted-foreground mt-2 leading-relaxed">
						This will permanently delete <strong class="text-foreground">{selectedCount}</strong> item(s) 
						totaling <strong class="text-foreground">{formatBytes(totalSelectedSize)}</strong>. 
						This action cannot be undone.
					</Dialog.Description>
				</div>
			</div>
			<div class="flex justify-end gap-2 mt-5">
				<Dialog.Close class="px-3 py-1.5 border border-border hover:bg-accent rounded text-xs font-medium text-foreground transition-colors">Cancel</Dialog.Close>
				<button 
					class="px-3 py-1.5 bg-destructive hover:bg-destructive/90 text-white rounded text-xs font-bold transition-all active:scale-95" 
					onclick={confirmClean}
				>
					Delete {selectedCount} Items
				</button>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
