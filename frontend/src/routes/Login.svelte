<script>
    import { account } from '$stores';
    import { getLoggedIn, sendJSON } from '$utils';
    import { onMount } from 'svelte';
    import { navigate } from 'svelte-navigator';

    let emailInput;
    let passwordInput;
    let otpInput;

    let processing = false;
    let type = 'email';
    let email = '';
    let password = '';
    let otp = '';

    onMount(() => {
        emailInput.focus();
    });

    account.subscribe((user) => {
        if (user?.id) {
            navigate('/');
        }
    });

    async function submitForm() {
        if (processing) return;
        processing = true;
        await submit(type);
        processing = false;
    }

    async function submit(type) {
        switch (type) {
            case 'email':
                return await submitEmail();
            case 'password':
                return await submitPassword();
            case 'otp':
                return await submitOtp();
        }
    }

    async function submitEmail() {
        try {
            const response = await sendJSON('/api/login', 'POST', { email });
            const data = await response.json();
            if (response.status === 200) {
                type = data.toLowerCase();
                setTimeout(() => {
                    if (type === 'password') passwordInput.focus();
                    else otpInput.focus();
                }, 1);
            }
        } catch (error) {
            console.error(error);
        }
    }

    async function submitPassword() {
        try {
            const response = await sendJSON('/api/login/password', 'POST', { email, password });
            if (response.status === 200) {
                getLoggedIn();
            }
        } catch (error) {
            console.error(error);
        }
    }

    async function submitOtp() {
        try {
            const response = await sendJSON('/api/login/otp', 'POST', { code: otp });
            if (response.status === 200) {
                getLoggedIn();
            }
        } catch (error) {
            console.error(error);
        }
    }
</script>

<div
    class="container relative hidden h-[800px] flex-col items-center justify-center md:grid lg:max-w-none lg:grid-cols-1 lg:px-0"
>
    <div class="lg:p-8">
        <div class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
            <div class="flex flex-col space-y-2 text-center">
                <h1 class="text-2xl font-semibold tracking-tight">Login</h1>
                <p class="text-sm text-muted-foreground">Enter your email below to start logging in</p>
            </div>

            <div class="grid gap-6">
                <div class="grid gap-4"></div>
                <form on:submit|preventDefault={submitForm}>
                    <div class="grid gap-1">
                        <input
                            type="email"
                            placeholder="Email"
                            required
                            bind:value={email}
                            readonly={type !== 'email'}
                            disabled={type !== 'email'}
                            bind:this={emailInput}
                        />
                    </div>
                    <div class="grid gap-1">
                        <input
                            type="password"
                            placeholder="Password"
                            bind:value={password}
                            class={type !== 'password' ? 'hidden' : ''}
                            bind:this={passwordInput}
                        />
                    </div>
                    <div class="grid gap-1">
                        <input
                            type="text"
                            placeholder="One Time Password"
                            bind:value={otp}
                            class={type !== 'otp' ? 'hidden' : ''}
                            bind:this={otpInput}
                        />
                    </div>
                    <div class="grid gap-1">
                        <button class={processing ? 'disabled' : ''}>{type === 'email' ? 'Send' : 'Login'}</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</div>
