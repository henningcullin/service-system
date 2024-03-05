<script>

    /** @param {string} selector */
    function el(selector) {
        return document.querySelector(selector);
    }

    let email = '';
    let password = '';
    let code = '';


    async function submitEmailForm() {

        if (email.length <= 0) return;

        const response = await fetch('/api/user/login', {
            headers: {
                'Content-Type':'application/json'
            },
            method: 'POST',
            body: JSON.stringify({email})
        });

        console.log(await response.json());
        
        if (response.status != 200) {
            return;
        }

        el('#emailForm')?.classList.add('hidden');
        el('#passwordForm')?.classList.remove('hidden');
    }

</script>

<div class="segment">
    <h2>Log in</h2>
    
    <form id="emailForm" on:submit|preventDefault={submitEmailForm}>
        <input type="email" placeholder="email" bind:value={email}><br>
        <input type="submit" value="Send">
    </form>

    <form id="passwordForm" class="hidden">
        <input type="email" readonly bind:value={email}><br>
        <input type="password" placeholder="password" bind:value={password}><br>
        <input type="submit" value="Login">
    </form>
</div>

<style>

    .segment {
        height:95%;
        width:95%;
        background-color: #202020;
        padding:7%;
        border-radius: 3px;
        justify-content: center;
        text-align: center;
    }

</style>