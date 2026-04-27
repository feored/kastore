<script lang="ts">
  import type { DiagnosticDto } from '$lib/backend/bindings/DiagnosticDto';
  import Badge from '$lib/ui/Badge.svelte';
  import Panel from '$lib/ui/Panel.svelte';

  let { diagnostics }: { diagnostics: DiagnosticDto[] } = $props();
</script>

<Panel title="Diagnostics" meta={`${diagnostics.length} reported`}>
  {#if diagnostics.length === 0}
    <p class="text-sm text-muted-foreground">No parser diagnostics were reported for this save.</p>
  {:else}
    <div class="grid gap-3">
      {#each diagnostics as diagnostic}
        {@const kind = diagnostic.kind.toLowerCase()}
        <article class="rounded border border-border bg-panel-muted p-3">
          <div class="flex items-center gap-2">
            <Badge tone={kind.includes('error') ? 'danger' : kind.includes('warn') ? 'warning' : 'muted'}>
              {diagnostic.kind}
            </Badge>
            <span class="font-mono text-xs text-muted-foreground">
              {diagnostic.section}{diagnostic.field ? `.${diagnostic.field}` : ''}
            </span>
          </div>

          <p class="mt-2 text-sm text-foreground">{diagnostic.message}</p>

          {#if diagnostic.offset !== null}
            <small class="mt-2 block font-mono text-xs text-muted">Offset {diagnostic.offset}</small>
          {/if}
        </article>
      {/each}
    </div>
  {/if}
</Panel>
