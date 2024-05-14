<script lang="ts">
    export let value: { id: string; name: string };
    import { Link } from 'svelte-navigator';
</script>

<div class="flex w-[100px] items-center">
    <Link to="/machine/type/{value.id}" class="rounded-sm underline-offset-2 hover:underline">{value.name}</Link>
</div>
