<script lang="ts">
    import { currentBookId } from "./lib/stores";
    import MainPage from "./routes/MainPage.svelte";
    import BookPage from "./routes/BookPage.svelte";
    import SettingsPage from "./routes/SettingsPage.svelte";
</script>

{#if $currentBookId === null}
    <MainPage />
{:else if $currentBookId === "settings"}
    <SettingsPage />
{:else}
    <BookPage bookId={$currentBookId} />
{/if}
