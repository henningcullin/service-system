<script>
    import * as HoverCard from '$components/ui/hover-card';
    import { Link } from 'svelte-navigator';

    export let value;
</script>

{#if value.id}
    <div class="flex space-x-2">
        <span class="max-w-[200px] truncate font-medium">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <Link to="/role/{value.id}" class="rounded-sm underline-offset-2 hover:underline"
                        >{value.name}
                    </Link>
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    <div class="flex justify-between space-x-4">
                        <div class="space-y-1">
                            <h4 class="text-sm font-semibold">{value.name}</h4>
                            <p class="text-sm">Level {value.level}</p>
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
