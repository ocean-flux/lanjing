<script lang="ts">
  import { m, getLocale, setLocale, locales } from '$lib/i18n';

  // 当前语言用于切换按钮的高亮状态
  let current = $state(getLocale());

  function switchTo(locale: (typeof locales)[number]) {
    setLocale(locale);
    current = locale;
  }
</script>

<div class="flex flex-col items-center justify-center h-full gap-4 text-muted-foreground">
  <h1 class="text-2xl font-semibold">{m.welcome()}</h1>

  <div class="flex gap-2">
    {#each locales as locale (locale)}
      <button
        type="button"
        class="px-3 py-1 rounded-full border text-sm transition-colors
        {current === locale
          ? 'bg-primary/10 border-primary/30 text-foreground'
          : 'border-border hover:bg-accent'}"
        onclick={() => switchTo(locale)}
      >
        {locale}
      </button>
    {/each}
  </div>
</div>
