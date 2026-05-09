<script lang="ts">
  import DiagnosticsView from '$lib/editor/diagnostics/DiagnosticsView.svelte';
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import ScenarioView from '$lib/editor/scenario/ScenarioView.svelte';
  import type { EditorSectionId } from '$lib/editor/sections';
  import SessionBar from '$lib/layout/SessionBar.svelte';
  import Sidebar from '$lib/layout/Sidebar.svelte';

  let activeSection = $state<EditorSectionId>('scenario');
</script>

<div class="grid min-h-screen grid-cols-[13rem_1fr] bg-background text-foreground">
  <Sidebar {activeSection} onSelect={(section) => (activeSection = section)} />

  <main class="min-w-0">
    <SessionBar />

    <div class="p-4">
      {#if openedSaveSession.currentSave}
        {#if activeSection === 'scenario'}
          <ScenarioView save={openedSaveSession.currentSave} />
        {:else}
          <DiagnosticsView diagnostics={openedSaveSession.currentSave.diagnostics} />
        {/if}
      {:else}
        <section class="rounded border border-border bg-panel p-4">
          <h1 class="text-base font-semibold">Open a save file</h1>
          <p class="mt-1 text-sm text-muted-foreground">No save opened.</p>
        </section>
      {/if}
    </div>
  </main>
</div>
