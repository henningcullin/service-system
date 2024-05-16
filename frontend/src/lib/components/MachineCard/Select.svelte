<script lang="ts">
    import Label from '$components/ui/label/label.svelte';
    import * as Select from '$components/ui/select/index.js';
    import { Writable } from 'svelte/store';
    import { isViewing } from './common';

    export let properties: { id: string; label: string; placeholder: string };
    export let selected: { label: string; value: string };
    export let onSelectedChange;
    export let errors: string = '';
</script>

<div>
    <Label for={properties.id} class={errors ? 'text-red-800' : ''}>{properties.label}</Label>
    <Select.Root disabled={$isViewing} bind:selected {onSelectedChange}>
        <Select.Trigger>
            <Select.Value placeholder={properties.placeholder} />
        </Select.Trigger>
        <Select.Content>
            <slot />
        </Select.Content>
    </Select.Root>
    {#if errors}
        <p class="text-red-800 ml-auto text-xs pt-1">{errors[0]}</p>
    {/if}
</div>
