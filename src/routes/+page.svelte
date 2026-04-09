<script lang="ts">
	import { cleanerStore } from '$lib/stores/cleaner.svelte';
	import NumberFlow from '@number-flow/svelte';
	import { Dialog, DropdownMenu } from 'bits-ui';
	import { fade } from 'svelte/transition';
	import { 
		HardDrive, 
		Cpu, 
		Search, 
		Trash2, 
		Activity, 
		ShieldCheck,
		AlertTriangle,
		ArrowDownWideNarrow,
		ArrowUpNarrowWide,
		MoreHorizontal,
		FolderOpen,
		EyeOff,
		Info,
		XOctagon,
		Globe,
		Terminal
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
			return cleanerStore.results; // Debounce heavy sorting while IPC floods the store
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

	let rowHeight = 32; // VSCode strict dense row height
	let scrollY = $state(0);
	let viewportHeight = $state(600);
	
	let activeResults = $derived(sortedResults.filter(i => i.exists && i.size > 0).map((r, idx) => ({ ...r, virtualIndex: idx })));
	let startIndex = $derived(Math.max(0, Math.floor(scrollY / rowHeight) - 8));
	let endIndex = $derived(Math.min(activeResults.length, Math.ceil((scrollY + viewportHeight) / rowHeight) + 8));
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

	function truncatePath(path: string, maxLength: number = 75) {
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
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex-1 flex flex-col overflow-hidden bg-background h-full text-foreground relative">
	
	<!-- VSCode Editor Tab Bar -->
	<div class="h-9 flex items-center bg-neutral-900 border-b border-border pl-2 shrink-0 select-none pt-1">
		<div class="px-4 h-full flex items-center gap-2 bg-background border-x border-t border-border rounded-t-sm text-[13px] text-foreground font-medium relative top-px">
			<HardDrive size={14} class="text-primary" />
			<span>disk_analyzer.rs</span>
		</div>
	</div>

	<!-- Top App Toolbar (Dense Console Version) -->
	<div class="h-12 flex items-center justify-between px-4 border-b border-border/50 shrink-0 bg-background">
		<div class="flex flex-col">
			<span class="text-xs uppercase tracking-widest text-muted-foreground font-bold">Deep Space Purge</span>
			<span class="text-[10px] text-primary/70 font-mono">Workspace directory mapping & secure extraction</span>
		</div>

		<div class="flex items-center gap-6 text-xs font-mono text-muted-foreground bg-neutral-900/50 border border-border/50 rounded px-3 py-1.5 shadow-inner">
			{#if cleanerStore.stats}
				<div class="flex items-center gap-2 border-r border-border/50 pr-4" title="Physical Storage">
					<HardDrive size={12} class="text-primary"/> 
					<span>{cleanerStore.stats.disk.used_human} / {cleanerStore.stats.disk.total_human}</span>
				</div>
				<div class="flex items-center gap-2 border-r border-border/50 pr-4" title="System Memory">
					<Activity size={12} class="text-primary"/> 
					<span>RAM: <NumberFlow value={cleanerStore.stats.memory.used / 1073741824} format={{ maximumFractionDigits: 1 }} />G</span>
				</div>
				<div class="flex items-center gap-2" title="Network Velocity">
					<Globe size={12} class="text-primary"/> 
					<span>{cleanerStore.stats.network.tx_human}↑ {cleanerStore.stats.network.rx_human}↓</span>
				</div>
			{/if}
		</div>

		<div class="flex items-center gap-3">
			<button 
				onclick={() => isConfirmModalOpen = true}
				disabled={totalSelectedSize === 0 || cleanerStore.isCleaning || cleanerStore.isScanning}
				class="bg-red-500/10 hover:bg-red-500/20 text-red-500 border border-red-500/20 px-3 py-1.5 rounded text-xs font-semibold transition-all active:scale-95 disabled:opacity-50 flex items-center gap-2"
			>
				<Trash2 class="w-3.5 h-3.5" />
				SHRED {formatBytes(totalSelectedSize)}
			</button>
			<button 
				onclick={() => cleanerStore.startScan()} 
				disabled={cleanerStore.isScanning || cleanerStore.isCleaning}
				class="bg-primary/10 hover:bg-primary/20 border border-primary/20 text-primary px-3 py-1.5 rounded text-xs font-semibold transition-all active:scale-95 disabled:opacity-50 flex items-center gap-2"
			>
				<Search class="w-3.5 h-3.5" />
				{cleanerStore.isScanning ? 'MAPPING' : 'DEEP SCAN'}
			</button>
		</div>
	</div>

	<!-- Main Workspace -->
	<div class="flex-1 min-h-0 flex flex-col bg-background relative shadow-inner font-mono text-[13px]">
		<!-- Execution Banner -->
		{#if cleanerStore.isScanning || cleanerStore.isCleaning}
			<div class="w-full bg-neutral-900 border-b border-border flex flex-col pt-4 px-6 pb-5 relative" in:fade={{duration: 200}} out:fade={{duration: 200}}>
				<div class="flex items-center justify-between mb-3 text-sm">
					<div class="flex items-center gap-3">
						<div class="w-2 h-2 rounded-full bg-primary animate-ping"></div>
						<span class="text-foreground font-semibold">
							{cleanerStore.isCleaning ? 'Executing SECURE_SHRED' : 'Executing TRACE_SCAN'}
						</span>
						<span class="text-muted-foreground ml-2 truncate max-w-xl text-[12px]">
							{cleanerStore.progress?.current_location || 'Pending allocations...'}
						</span>
					</div>
					<div class="flex gap-6 text-muted-foreground">
						<span>Blocks: <span class="text-primary font-bold"><NumberFlow value={cleanerStore.progress?.found_count || 0} /></span></span>
						<span>Size: <span class="text-foreground">{formatBytes(cleanerStore.progress?.total_size || 0)}</span></span>
						<span>Time: {elapsedSec}s</span>
						<span>[{cleanerStore.progress?.current || 0}/{cleanerStore.progress?.total || 0}]</span>
					</div>
				</div>
				<div class="w-full h-1.5 bg-background rounded-full overflow-hidden shadow-inner">
					<div class="h-full bg-primary transition-all duration-300 ease-out" style="width: {cleanerStore.progress?.percent || 0}%"></div>
				</div>
				<button 
					class="absolute top-4 right-6 text-red-500 hover:text-red-400 bg-red-500/10 px-2 py-1 rounded-sm text-xs font-semibold flex items-center gap-1 transition-colors"
					onclick={() => cleanerStore.abortScan()}
				>
					<XOctagon size={12} /> HALT
				</button>
			</div>
		{/if}

		<!-- Empty States -->
		{#if !cleanerStore.isScanning && activeResults.length === 0}
			<div class="flex-1 flex flex-col items-center justify-center text-muted-foreground p-10 text-center">
				{#if cleanerStore.results.length === 0}
					<Terminal class="w-10 h-10 mb-4 opacity-50" />
					<h3 class="text-lg text-foreground mb-1">Awaiting Commands</h3>
					<p class="text-sm max-w-sm">No data paths mapped. Press `Meta+Enter` or click Deep Scan to start telemetry.</p>
				{:else}
					<ShieldCheck class="w-10 h-10 mb-4 text-green-500/80" />
					<h3 class="text-lg text-foreground mb-1">System Optimal</h3>
					<p class="text-sm max-w-sm">Target branches are clean. No remaining orphan structures inside localized registries.</p>
				{/if}
			</div>
		{:else}
			<!-- Table Header -->
			<div class="bg-neutral-900 border-b border-border sticky top-0 z-20 grid grid-cols-[2.5rem_minmax(0,1fr)_10rem_6rem_2.5rem] items-center text-[11px] font-bold tracking-wider text-muted-foreground uppercase h-8">
				<div class="px-2 border-r border-border/40 h-full flex items-center justify-center">
					<input 
						type="checkbox" 
						aria-label="Select all targets"
						class="rounded bg-background text-primary border border-border h-3 w-3 focus:ring-0 focus:ring-offset-0 focus:border-primary disabled:opacity-50"
						disabled={cleanerStore.isScanning || cleanerStore.isCleaning}
						checked={cleanerStore.results.length > 0 && cleanerStore.results.every(r => r.selected)}
						onchange={(e) => cleanerStore.toggleAll(e.currentTarget.checked)}
					/>
				</div>
				<button disabled={cleanerStore.isScanning || cleanerStore.isCleaning} class="px-3 h-full hover:text-foreground text-left border-r border-border/40 focus:outline-none flex items-center gap-2 group transition-colors" onclick={() => toggleSort('name')}>
					TARGET PATH {#if sortKey === 'name'} <span class="text-primary group-hover:opacity-100 opacity-80">{#if sortAsc}<ArrowUpNarrowWide size={12}/>{:else}<ArrowDownWideNarrow size={12}/>{/if}</span> {/if}
				</button>
				<button disabled={cleanerStore.isScanning || cleanerStore.isCleaning} class="px-3 h-full hover:text-foreground text-left border-r border-border/40 focus:outline-none flex items-center gap-2 group transition-colors" onclick={() => toggleSort('category')}>
					DESCRIPTOR {#if sortKey === 'category'} <span class="text-primary group-hover:opacity-100 opacity-80">{#if sortAsc}<ArrowUpNarrowWide size={12}/>{:else}<ArrowDownWideNarrow size={12}/>{/if}</span> {/if}
				</button>
				<button disabled={cleanerStore.isScanning || cleanerStore.isCleaning} class="px-3 h-full hover:text-foreground text-right border-r border-border/40 focus:outline-none flex items-center justify-end gap-2 group transition-colors" onclick={() => toggleSort('size')}>
					BLOCK_SIZE {#if sortKey === 'size'} <span class="text-primary group-hover:opacity-100 opacity-80">{#if sortAsc}<ArrowUpNarrowWide size={12}/>{:else}<ArrowDownWideNarrow size={12}/>{/if}</span> {/if}
				</button>
				<div class="h-full flex items-center justify-center px-2"></div>
			</div>

			<!-- Virtual Scroll Area -->
			<div class="flex-1 overflow-auto relative bg-background" onscroll={(e) => scrollY = e.currentTarget.scrollTop} bind:clientHeight={viewportHeight}>
				<div class="absolute w-full" style="height: {activeResults.length * rowHeight}px;">
					{#each virtualResults as item (item.id)}
						<div class="absolute w-full hover:bg-neutral-800/80 transition-colors grid grid-cols-[2.5rem_minmax(0,1fr)_10rem_6rem_2.5rem] items-center border-b border-border/20 group" style="top: {item.virtualIndex * rowHeight}px; height: {rowHeight}px;">
							
							<div class="px-2 border-r border-border/10 justify-center flex">
								<input 
									type="checkbox" 
									aria-label="Select row"
									class="rounded bg-background text-primary border border-border h-3 w-3 focus:ring-0"
									checked={item.selected}
									onchange={(e) => cleanerStore.toggleItem(item.id, e.currentTarget.checked)}
								/>
							</div>
							
							<div class="px-3 border-r border-border/10 flex flex-col justify-center overflow-hidden h-full">
								<div class="flex items-baseline gap-2">
									<span class="font-medium text-foreground truncate">{item.name}</span>
									<span class="text-muted-foreground text-[10px] truncate opacity-50 font-sans" title={item.path}>{truncatePath(item.path)}</span>
								</div>
							</div>
							
							<div class="px-3 border-r border-border/10 flex items-center h-full">
								<span class="text-muted-foreground capitalize text-[11px] flex items-center gap-1.5 font-sans font-medium">
									{#if item.category.toLowerCase().includes('privacy')}
										<div class="w-1.5 h-1.5 rounded-full bg-purple-500/80 shadow-[0_0_8px_rgba(168,85,247,0.5)]"></div>
									{:else if item.category.toLowerCase().includes('system')}
										<div class="w-1.5 h-1.5 rounded-full bg-neutral-500"></div>
									{:else}
										<div class="w-1.5 h-1.5 rounded-full bg-primary/80 shadow-[0_0_8px_rgba(56,189,248,0.5)]"></div>
									{/if}
									{item.category}
								</span>
							</div>
							
							<div class="px-3 border-r border-border/10 h-full flex items-center justify-end text-primary/80 font-medium">
								{item.size_human}
							</div>
							
							<div class="h-full flex items-center justify-center">
								<DropdownMenu.Root>
									<DropdownMenu.Trigger class="text-muted-foreground hover:bg-muted p-1 rounded transition-colors" disabled={cleanerStore.isScanning || cleanerStore.isCleaning}>
										<MoreHorizontal size={14} />
									</DropdownMenu.Trigger>
									<DropdownMenu.Content class="w-48 bg-card border border-border rounded-md shadow-2xl py-1 z-50 overflow-hidden font-sans">
										<DropdownMenu.Item class="px-3 py-2 text-[13px] text-neutral-300 hover:bg-primary/20 hover:text-primary cursor-pointer flex items-center gap-2 outline-none" onclick={() => cleanerStore.openFolder(item.path)}>
											<FolderOpen size={14} /> Open Directory
										</DropdownMenu.Item>
										<DropdownMenu.Item class="px-3 py-2 text-[13px] text-neutral-300 hover:bg-neutral-800 hover:text-foreground cursor-pointer flex items-center gap-2 outline-none" onclick={() => cleanerStore.ignoreItem(item.id)}>
											<EyeOff size={14} /> Exclude
										</DropdownMenu.Item>
										<DropdownMenu.Separator class="h-px bg-border my-1" />
										<DropdownMenu.Item class="px-3 py-2 text-[13px] text-neutral-300 hover:bg-neutral-800 hover:text-foreground cursor-pointer flex items-center gap-2 outline-none">
											<Info size={14} /> Stat Inspector
										</DropdownMenu.Item>
									</DropdownMenu.Content>
								</DropdownMenu.Root>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>

<Dialog.Root bind:open={isConfirmModalOpen}>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-background/80 backdrop-blur-[2px] transition-all" />
		<Dialog.Content class="fixed left-1/2 top-1/2 z-50 -translate-x-1/2 -translate-y-1/2 w-full max-w-lg bg-card border border-border p-6 rounded-lg shadow-xl outline-none font-mono text-sm">
			<div class="flex items-start gap-4">
				<AlertTriangle class="text-red-500 w-5 h-5 shrink-0 mt-1" />
				<div>
					<Dialog.Title class="text-base font-bold text-foreground uppercase tracking-wider">Execute Purge Phase</Dialog.Title>
					<Dialog.Description class="text-muted-foreground mt-3 leading-relaxed">
						WARNING: About to terminate <strong>{cleanerStore.results.filter(r => r.selected).length}</strong> node(s) recovering <strong>{formatBytes(totalSelectedSize)}</strong> memory.
						This routine bypasses standard kernel recycling and forces permanent block disposal. Proceed?
					</Dialog.Description>
				</div>
			</div>
			<div class="flex justify-end gap-3 mt-8">
				<Dialog.Close class="px-4 py-2 border border-border hover:bg-neutral-800 rounded text-foreground font-semibold transition-colors">ABORT</Dialog.Close>
				<button 
					class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded font-bold shadow-md shadow-red-500/10 transition-all active:scale-95" 
					onclick={confirmClean}
				>
					S1: SHRED
				</button>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
