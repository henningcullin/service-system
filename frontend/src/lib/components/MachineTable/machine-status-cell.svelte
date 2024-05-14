<script>
    import * as HoverCard from '$components/ui/hover-card/index.js';
    export let value;
</script>

<div class="flex items-center">
    <HoverCard.Root>
        <HoverCard.Trigger>
            {value?.name}
        </HoverCard.Trigger>
    </HoverCard.Root>
</div>
