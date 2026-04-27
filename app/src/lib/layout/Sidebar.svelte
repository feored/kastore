<script lang="ts">
	import { editorSections, type EditorSectionId } from "$lib/editor/sections";

	let {
		activeSection,
		onSelect,
	}: {
		activeSection: EditorSectionId;
		onSelect: (section: EditorSectionId) => void;
	} = $props();
</script>

<aside class="border-r border-border bg-panel-muted">
	<div class="border-b border-border px-4 py-4">
		<strong class="block text-sm font-semibold">Anduran</strong>
	</div>

	<nav class="grid gap-1 p-2" aria-label="Editor sections">
		{#each editorSections as section}
			<button
				type="button"
				aria-current={section.id === activeSection ? "page" : undefined}
				class={[
					"rounded border px-3 py-2 text-left text-sm transition-colors",
					section.id === activeSection
						? "border-accent/50 bg-accent/10 text-accent"
						: "border-transparent text-muted-foreground hover:bg-panel-elevated hover:text-foreground",
				]}
				onclick={() => onSelect(section.id)}
			>
				{section.label}
			</button>
		{/each}
	</nav>
</aside>
