<script>
    import { Writable } from 'svelte/store';
    import { Checkbox } from '$lib/components/ui/checkbox/index.js';

    /**
     * @type Writable<Boolean>
     */
    export let checked;
</script>

<Checkbox bind:checked={$checked} {...$$restProps} />
