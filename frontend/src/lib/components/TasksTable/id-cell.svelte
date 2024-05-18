<script lang="ts">
    import { Link } from 'svelte-navigator';

    export let value: string;
</script>

<div class="flex space-x-4">
    <span class="max-w-[350px] truncate font-medium">
        <Link to="/task/{value}" class="rounded-sm underline-offset-2 hover:underline">{value}</Link>
    </span>
</div>
