<script>
    import { Checkbox } from '$components/ui/checkbox';
    import { Label } from '$components/ui/label';
    import { isViewing } from './common';

    export let label;
    export let checked;
</script>

<Label>{label}</Label>
<Checkbox bind:checked disabled={$isViewing} /><br />
