<script lang="ts">
  import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';
  import Panel from '$lib/ui/Panel.svelte';
  import ReadonlyField from '$lib/ui/ReadonlyField.svelte';

  let { save }: { save: OpenedSaveDto } = $props();

  const scenarioRows = $derived([
    ['Scenario name', save.scenario.name.text || 'Untitled'],
    ['Scenario file', save.scenario.fileName.text || 'Unknown'],
    ['Size', `${save.scenario.width} x ${save.scenario.height}`],
    ['Difficulty', save.scenario.difficulty],
    ['Language', save.scenario.language],
    ['Game type', save.scenario.gameType],
  ]);

  const sourceRows = $derived([
    ['Source file', save.source.fileName],
    ['Requires Price of Loyalty', save.scenario.requiresPol ? 'Yes' : 'No'],
    ['Save version', save.source.saveVersion.toString()],
  ]);
</script>

<Panel title="Scenario" meta={save.source.fileName}>
  <div class="grid gap-5">
    <section class="grid gap-3 border-b border-border pb-4">
      <div class="flex min-w-0 items-start justify-between gap-4">
        <div class="min-w-0">
          <p class="text-xs font-medium uppercase tracking-wide text-muted">Scenario</p>
          <h2 class="mt-1 truncate text-xl font-semibold text-foreground">{save.scenario.name.text || 'Untitled scenario'}</h2>
        </div>

        <div class="shrink-0 rounded border border-border bg-panel-muted px-2 py-1 font-mono text-xs text-muted-foreground">
          {save.scenario.width} x {save.scenario.height}
        </div>
      </div>

      {#if save.scenario.description.text}
        <p class="max-w-4xl text-sm leading-6 text-muted-foreground">{save.scenario.description.text}</p>
      {:else}
        <p class="text-sm text-muted">No scenario description.</p>
      {/if}
    </section>

    <div class="grid gap-5 xl:grid-cols-[minmax(0,1fr)_22rem]">
      <section class="min-w-0">
        <h3 class="mb-2 text-xs font-semibold uppercase tracking-wide text-muted">Scenario properties</h3>
        <dl class="divide-y divide-border rounded border border-border bg-panel-muted">
          {#each scenarioRows as [label, value]}
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
