<script>

    import { navigate } from 'svelte-navigator';

    const MachineStatus = ['Active', 'Inactive'];

    const newMachine = {
        name: '',
        make: '',
        machine_type: '',
        status: 'Inactive'
    };

    async function createMachine() {
        if (newMachine.name.length <= 0) return;

        const response = await fetch('/api/auth/machine', {
            headers: {
                'Content-Type':'application/json'
            },
            method: 'POST',
            body: JSON.stringify(newMachine)
        });

        const data = await response.json();

        if (response.status != 201) return alert(data.message);

        newMachine.name = '';
        newMachine.make = '';
        newMachine.machine_type = '';
        newMachine.status = 'Inactive';

        navigate('/machines/');
    }
    
</script>

<div class='segment'>
    <h2> Create new Machine </h2>

    <form on:submit|preventDefault={createMachine}>
        <input type='text' placeholder='Name' bind:value={newMachine.name}>
        <input type='text' placeholder='Make' bind:value={newMachine.make}>
        <input type='text' placeholder='Type' bind:value={newMachine.machine_type}>
        <select bind:value={newMachine.status}>
            {#each MachineStatus as option}
                <option value='{option}'>{option}</option>
            {/each}
        </select>
        <input type='submit' value='Create'>
    </form>
</div>

<style>

    h2 {
        margin-bottom: 50px;
    }

</style>