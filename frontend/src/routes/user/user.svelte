<script lang="ts">
    import * as Tabs from '$components/ui/tabs/index.js';
    import UserCard from '$components/UserCard/UserCard.svelte';

    document.title = 'User';
</script>

<div class="flex flex-col items-center min-h-screen pt-8">
    <div class="flex flex-col min-w-full">
        <UserCard />
    </div>
</div>