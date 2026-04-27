<script lang="ts">
  import type { Snippet } from 'svelte';

  type BadgeTone = 'default' | 'success' | 'warning' | 'danger' | 'muted';

  const toneClasses: Record<BadgeTone, string> = {
    default: 'border-accent/45 bg-accent/10 text-accent',
    success: 'border-success/45 bg-success/10 text-success',
    warning: 'border-warning/45 bg-warning/10 text-warning',
    danger: 'border-danger/45 bg-danger/10 text-danger',
    muted: 'border-border bg-panel-muted text-muted-foreground',
  };

  let {
    tone = 'default',
    children,
  }: {
    tone?: BadgeTone;
    children?: Snippet;
  } = $props();
</script>

<span class={['inline-flex items-center rounded border px-2 py-0.5 text-xs font-medium', toneClasses[tone]]}>
  {@render children?.()}
</span>
