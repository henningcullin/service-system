<script>
    let type = 'email';
    let email = '';
    let password = '';
    let otp = '';

    function submitForm() {}
</script>

<segment>
    <form on:submit|preventDefault={submitForm}>
        <input bind:value={email} type="email" placeholder="Email" />
        <input bind:value={password} type="password" placeholder="Password" />
        <input bind:value={otp} type="text" placeholder="One Time Password" />
        <input type="submit" value={type === 'email' ? 'Send' : 'Login'} />
    </form>
</segment>

<style>
    form {
        display: grid;
        grid-template-columns: 1fr 1fr;
    }
</style>
