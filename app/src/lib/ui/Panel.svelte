<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    title,
    meta,
    class: className = '',
    children,
  }: {
    title?: string;
    meta?: string | number;
    class?: string;
    children?: Snippet;
  } = $props();
</script>

<section class={['rounded border border-border bg-panel text-foreground', className]}>
  {#if title || meta !== undefined}
    <header class="flex items-center justify-between gap-4 border-b border-border px-4 py-3">
      {#if title}
        <h1 class="text-base font-semibold">{title}</h1>
      {/if}

      {#if meta !== undefined}
        <div class="text-sm text-muted-foreground">{meta}</div>
      {/if}
    </header>
  {/if}

  <div class="p-4">
    {@render children?.()}
  </div>
</section>
