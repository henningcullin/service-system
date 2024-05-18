<script lang="ts">
    import * as HoverCard from '$components/ui/hover-card';
    import { Link } from 'svelte-navigator';

    export let value: { id: string; first_name: string; last_name: string; email: string };
</script>

{#if value.id}
    <div class="flex space-x-2">
        <span class="max-w-[200px] truncate font-medium">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <Link to="/user/{value.id}" class="rounded-sm underline-offset-2 hover:underline"
                        >{value.first_name}
                    </Link>
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    <div class="flex justify-between space-x-4">
                        <div class="space-y-1">
                            <h4 class="text-sm font-semibold">{value.first_name} {value.last_name}</h4>
                            <a class="text-sm" href="mailto:{value.email}">{value.email}</a>
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
