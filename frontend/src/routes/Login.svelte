<script>
    let email = "";
    let password = "";
    let code = "";
</script>

<form>
    <input bind:value={email} type="email" name="email" placeholder="Email" />
    <input
        bind:value={password}
        type="password"
        name="password"
        placeholder="Password"
    />
    <input bind:value={code} type="text" name="code" placeholder="Code" />
    <input type="submit" />
</form>
