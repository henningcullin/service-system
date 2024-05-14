<script lang="ts">
    import * as HoverCard from '$components/ui/hover-card';
    import { Link } from 'svelte-navigator';

    export let value: { id: string; name: string; address?: string };
</script>

{#if value.id}
    <div class="flex space-x-2">
        <span class="max-w-[200px] truncate font-medium">
            <HoverCard.Root>
                <HoverCard.Trigger
                    class="rounded-sm underline-offset-4 hover:underline focus-visible:outline-2 focus-visible:outline-offset-8 focus-visible:outline-black"
                >
                    <Link to="/facility/{value.id}">{value.name}</Link>
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    <div class="flex justify-between space-x-4">
                        <div class="space-y-1">
                            <h4 class="text-sm font-semibold">{value.name}</h4>
                            {#if value.address}
                                <p class="text-sm">{value.address}</p>
                            {/if}
                            <div class="flex items-center pt-2">
                                <span class="text-xs text-muted-foreground"> {value.id} </span>
                            </div>
                        </div>
                    </div>
                </HoverCard.Content>
            </HoverCard.Root>
        </span>
    </div>
{/if}
