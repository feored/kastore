<script lang="ts">
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import Button from '$lib/ui/Button.svelte';
</script>

<header class="flex min-h-14 items-center gap-3 border-b border-border bg-panel-muted px-4">
  <Button type="button" disabled={openedSaveSession.isLoading} onclick={() => openedSaveSession.pickAndOpen()}>
    {openedSaveSession.isLoading ? 'Opening...' : 'Open Save...'}
  </Button>

  {#if openedSaveSession.currentSave}
    <p class="min-w-0 truncate font-mono text-xs text-muted-foreground">{openedSaveSession.currentSave.source.path}</p>
  {/if}

  {#if openedSaveSession.error}
    <p class="ml-auto text-sm text-danger">{openedSaveSession.error}</p>
  {/if}
</header>
