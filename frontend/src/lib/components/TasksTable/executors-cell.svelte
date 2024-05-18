<script lang="ts">
    import * as HoverCard from '$components/ui/hover-card';
    import { Separator } from '$components/ui/separator';
    import { Link } from 'svelte-navigator';

    export let value: [{ id: string; first_name: string; last_name: string; email: string }];
</script>

{#if Array.isArray(value) && value.length}
    <div class="flex space-x-2">
        <span class="max-w-[200px] truncate font-medium">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    {value.length}
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    {#each value as executor}
                        <div class="flex justify-between space-x-4">
                            <div class="space-y-1">
                                <Link
                                    to="/user/{executor.id}"
                                    class="rounded-sm underline-offset-2 hover:underline font-semibold"
                                    >{executor.first_name} {executor.last_name}
                                </Link><br />
                                <a class="text-sm" href="mailto:{executor.email}">{executor.email}</a><br/>
                                <div class="flex items-center pt-2">
                                    <span class="text-xs text-muted-foreground"> {executor.id} </span>
                                </div>
                            </div>
                        </div>
                        <Separator class="my-1" />
                    {/each}
                </HoverCard.Content>
            </HoverCard.Root>
        </span>
    </div>
{/if}
