<script>
    import { account } from '$stores';
    import { getLoggedIn, sendJSON } from '$utils';
    import { onMount } from 'svelte';
    import { navigate } from 'svelte-navigator';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Button } from '$lib/components/ui/button/index.js';
    import LoaderCircle from 'lucide-svelte/icons/loader-circle';

    let emailInput;
    let passwordInput;
    let otpInput;

    let isProcessing = false;
    let type = 'email';
    let email = '';
    let password = '';
    let otp = '';

    onMount(() => {
        emailInput?.focus();
    });

    account.subscribe((user) => {
        if (user?.id) {
            navigate('/');
        }
    });

    async function submitForm() {
        if (isProcessing) return;
        isProcessing = true;
        await submit(type);
        isProcessing = false;
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
                    if (type === 'password') passwordInput?.focus();
                    else otpInput?.focus();
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
    class="container relative min-h-[90vh] flex items-center justify-center md:grid lg:max-w-none lg:grid-cols-1 lg:px-0"
>
    <div class="mx-auto flex w-full h-full flex-col justify-center items-center space-y-6 transform -translate-y-10">
        <div class="flex flex-col space-y-2 text-center">
            <h1 class="text-2xl font-semibold tracking-tight">Login</h1>
            <p class="text-sm text-muted-foreground">Enter your email below to start logging in</p>
        </div>

        <div class="grid gap-6 sm:w-[350px]">
            <form on:submit|preventDefault={submitForm}>
                <div class="grid gap-2">
                    <Input
                        type="email"
                        placeholder="Email"
                        autocapitalize="none"
                        autocomplete="email"
                        autocorrect="off"
                        required
                        bind:value={email}
                        readonly={type !== 'email'}
                        disabled={type !== 'email'}
                    />
                    <Input
                        type="password"
                        autocapitalize="none"
                        autocorrect="off"
                        placeholder="Password"
                        bind:value={password}
                        class={type !== 'password' ? 'hidden' : ''}
                    />
                    <Input
                        type="text"
                        autocapitalize="none"
                        autocorrect="off"
                        placeholder="One Time Password"
                        bind:value={otp}
                        class={type !== 'otp' ? 'hidden' : ''}
                    />
                    <Button type="submit" disabled={isProcessing}>
                        {#if isProcessing}
                            <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
                        {/if}
                        {type === 'email' ? 'Send' : 'Login'}
                    </Button>
                </div>
            </form>
        </div>
    </div>
</div>
