<script lang="ts">
  import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';
  import Panel from '$lib/ui/Panel.svelte';
  import ReadonlyField from '$lib/ui/ReadonlyField.svelte';

  let { save }: { save: OpenedSaveDto } = $props();

  const mapRows = $derived([
    ['Display name', save.header.mapName.text || 'Untitled'],
    ['Map file', save.header.mapFilename.text || 'Unknown'],
    ['Size', `${save.header.width} x ${save.header.height}`],
    ['Difficulty', save.header.difficulty],
    ['Language', save.header.language],
    ['Game type', save.header.gameType],
  ]);

  const sourceRows = $derived([
    ['Source file', save.source.fileName],
    ['Requires Price of Loyalty', save.header.requiresPol ? 'Yes' : 'No'],
    ['Save version', save.source.saveVersion.toString()],
  ]);
</script>

<Panel title="Header" meta={save.source.fileName}>
  <div class="grid gap-5">
    <section class="grid gap-3 border-b border-border pb-4">
      <div class="flex min-w-0 items-start justify-between gap-4">
        <div class="min-w-0">
          <p class="text-xs font-medium uppercase tracking-wide text-muted">Map</p>
          <h2 class="mt-1 truncate text-xl font-semibold text-foreground">{save.header.mapName.text || 'Untitled save'}</h2>
        </div>

        <div class="shrink-0 rounded border border-border bg-panel-muted px-2 py-1 font-mono text-xs text-muted-foreground">
          {save.header.width} x {save.header.height}
        </div>
      </div>

      {#if save.header.mapDescription.text}
        <p class="max-w-4xl text-sm leading-6 text-muted-foreground">{save.header.mapDescription.text}</p>
      {:else}
        <p class="text-sm text-muted">No map description.</p>
      {/if}
    </section>

    <div class="grid gap-5 xl:grid-cols-[minmax(0,1fr)_22rem]">
      <section class="min-w-0">
        <h3 class="mb-2 text-xs font-semibold uppercase tracking-wide text-muted">Map properties</h3>
        <dl class="divide-y divide-border rounded border border-border bg-panel-muted">
          {#each mapRows as [label, value]}
            <ReadonlyField {label} {value} />
          {/each}
        </dl>
      </section>

      <section class="min-w-0">
        <h3 class="mb-2 text-xs font-semibold uppercase tracking-wide text-muted">Save properties</h3>
        <dl class="divide-y divide-border rounded border border-border bg-panel-muted">
          {#each sourceRows as [label, value]}
            <ReadonlyField {label} {value} mono={label === 'Save version'} />
          {/each}
        </dl>
      </section>
    </div>
  </div>
</Panel>
