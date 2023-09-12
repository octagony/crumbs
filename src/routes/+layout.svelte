<script lang="ts">
	let searchBin = '';
	import { binaries } from '../path/index.js';
</script>

<input type="search" bind:value={searchBin} />
<ul>
	{#each binaries as bin}
		<li>{bin}</li>
	{/each}
</ul>
