<script lang="ts">
	import { redDir } from '../path/index.js';
</script>

<ul>
	{#each redDir as bin}
		<li>{bin}</li>
	{/each}
</ul>
