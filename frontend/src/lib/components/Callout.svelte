<script lang="ts">
	export let type: 'info' | 'warning' | 'note' | 'danger' = 'info';
	export let title: string = type.charAt(0).toUpperCase() + type.slice(1);
</script>

<div class="callout callout-{type}">
	<div class="callout-header">
		<span class="callout-icon"></span>
		<span class="callout-title">{title}</span>
	</div>
	<div class="callout-body">
		<slot />
	</div>
</div>
