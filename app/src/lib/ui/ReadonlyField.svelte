<script lang="ts">
  let {
    label,
    value,
    mono = false,
    emptyLabel = 'Unknown',
  }: {
    label: string;
    value: string | number | boolean | null | undefined;
    mono?: boolean;
    emptyLabel?: string;
  } = $props();

  const displayValue = $derived(
    value === null || value === undefined || value === '' ? emptyLabel : value === true ? 'Yes' : value === false ? 'No' : String(value)
  );
</script>

<div class="grid grid-cols-[8rem_1fr] items-baseline gap-3 px-3 py-2">
  <dt class="text-xs font-medium text-muted">{label}</dt>
  <dd class={['min-w-0 truncate text-sm text-foreground', mono ? 'font-mono' : '']}>{displayValue}</dd>
</div>
