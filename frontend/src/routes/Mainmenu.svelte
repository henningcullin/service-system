<script>
    import Button from '$components/ui/button/button.svelte';
    import { toggleMode } from 'mode-watcher';
</script>

<Button on:click={toggleMode}>My Cool Button</Button>
