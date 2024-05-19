<script lang="ts">
    import { Input } from '$components/ui/input';
    import Label from '$components/ui/label/label.svelte';
    import { isViewing } from './common';

    export let properties: { id: string; label: string };
    export let value: string;
    export let type: string = 'text';
    export let errors: string = '';
    export let disabled: boolean = false;
</script>

<div>
    <Label for={properties.id} class={errors ? 'text-red-800' : ''}>{properties.label}</Label>
    <Input {type} id={properties.id} bind:value placeholder={properties.label} disabled={$isViewing || disabled} />
    {#if errors}
        <p class="text-red-800 ml-auto text-xs pt-1">{errors[0]}</p>
    {/if}
</div>
