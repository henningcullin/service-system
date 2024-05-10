<script>
    import { getloggedIn, postJSON } from "$lib/utils";
    import { navigate } from "svelte-navigator";

    let type = "email";
    let email = "";
    let password = "";
    let code = "";

    function submitForm() {
        switch (type) {
            case "email":
                return loginEmail();
            case "password":
                return loginPassword();
            case "otp":
                return loginOtp();
        }
    }

    async function loginEmail() {
        try {
            const response = await postJSON("/api/login", { email });
            const data = await response.json();

            switch (data) {
                case "PASSWORD":
                    return (type = "password");
                case "OTP":
                    return (type = "otp");
            }
        } catch (error) {}
    }

    async function loginPassword() {
        try {
            const response = await postJSON("/api/login/password", {
                password,
            });
            if (response.status === 200) {
                await getloggedIn();
                navigate("/");
            }
        } catch (error) {}
    }

    async function loginOtp() {
        try {
            const response = await postJSON("/api/login/otp", { code });
            if (response.status === 200) {
                await getloggedIn();
                navigate("/");
            }
        } catch (error) {}
    }
</script>

<form on:submit|preventDefault={submitForm}>
    <input bind:value={email} type="email" name="email" placeholder="Email" />
    <input
        bind:value={password}
        type="password"
        name="password"
        placeholder="Password"
        class={type === "password" ? "" : "hidden"}
    />
    <input
        bind:value={code}
        type="text"
        name="code"
        placeholder="Code"
        class={type === "otp" ? "" : "hidden"}
    />
    <input type="submit" />
</form>
