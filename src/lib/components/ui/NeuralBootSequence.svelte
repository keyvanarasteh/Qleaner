<script lang="ts">
	import { onMount } from 'svelte';
	import './NeuralBootSequence.css';

	let { onComplete = () => {} } = $props();

	let splashClass = $state('');
	let show = $state(true);

	onMount(() => {
		// Timing sequence based on Kadir's 5.0s cinematic 3D loading overlay
		const timers = [
			setTimeout(() => {
				splashClass = 'split';
			}, 3000),
			setTimeout(() => {
				splashClass = 'split explode';
			}, 4200),
			setTimeout(() => {
				splashClass = 'split explode fade-out';
			}, 5000),
			setTimeout(() => {
				show = false;
				onComplete();
			}, 5800)
		];

		return () => {
			timers.forEach(clearTimeout);
		};
	});
</script>

{#if show}
	<div id="splashScreen" class="{splashClass}">
		<div class="glassmorphism-sweep"></div>
		<div class="cyber-grid"></div>
		
		<div class="tech-ring"></div>
		<div class="tech-ring inner"></div>
		<div class="tech-ring reverse"></div>
		
		<div class="shockwave"></div>
		
		<div class="cyber-lines">
			<!-- Lines -->
			<div class="line" style="transform: rotate(30deg); animation-delay: 0.8s;"></div>
			<div class="line" style="transform: rotate(150deg); animation-delay: 0.9s;"></div>
			<div class="line" style="transform: rotate(270deg); animation-delay: 1.0s;"></div>
			<div class="line" style="transform: rotate(210deg); animation-delay: 1.1s;"></div>
			<div class="line" style="transform: rotate(330deg); animation-delay: 1.2s;"></div>
			
			<!-- Nodes -->
			<div class="node" style="top: calc(50% + 10vw); left: calc(50% + 17.32vw); animation-delay: 1.2s;"></div>
			<div class="node" style="top: calc(50% + 10vw); left: calc(50% - 17.32vw); animation-delay: 1.3s;"></div>
			<div class="node" style="top: calc(50% - 20vw); left: 50%; animation-delay: 1.4s;"></div>
			<div class="node" style="top: calc(50% - 10vw); left: calc(50% - 17.32vw); animation-delay: 1.5s;"></div>
			<div class="node" style="top: calc(50% - 10vw); left: calc(50% + 17.32vw); animation-delay: 1.6s;"></div>
		</div>
		
		<div class="core-dot"></div>
		
		<div class="micro-terminal">
			<div class="mt-line">> MOUNTING QLEANER_CORE v1.0.0...</div>
			<div class="mt-line">> ESTABLISHING RUST_IPC BRIDGES...</div>
			<div class="mt-line">> INITIALIZING SYSTEM DIAGNOSTICS... [OK]</div>
			<div class="mt-line">> BYPASSING NEURAL BLOCKS...</div>
		</div>
		
		<div class="splash-logo">
			<span class="logo-q">Q</span><span class="logo-ap">LE</span><span class="logo-ri">ANER</span>
		</div>
	</div>
{/if}
