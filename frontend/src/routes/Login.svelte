<script>
	import { navigate } from "svelte-navigator";
	import { getLoggedIn } from "$lib/utils";

	let email = "";
	let type = "EMAIL";
	let password = "";
	let code = "";

	async function submitEmailForm() {
		password = "";
		code = "";

		if (email.length <= 0) return;

		const response = await fetch("/api/login", {
			headers: {
				"Content-Type": "application/json",
			},
			method: "POST",
			body: JSON.stringify({ email }),
		});

		const data = await response.json();

		if (response.status != 200) return;

		type = data;
	}

	async function submitLoginForm() {
		/**
		 *
		 * @param {string} endpoint
		 * @param {object} value
		 */
		async function loginFetch(endpoint, value) {
			return fetch(endpoint, {
				headers: {
					"Content-Type": "application/json",
				},
				method: "POST",
				body: JSON.stringify(value),
			});
		}

		const response =
			type === "PASSWORD"
				? await loginFetch("/api/login/password", {
						email,
						password,
					})
				: type === "OTP"
					? await loginFetch("/api/login/otp", { code })
					: console.error("Unknown login type");

		console.log(response);

		if (!response) return;

		if (response.status != 200) {
			const data = await response.json();
			return alert(data);
		}

		getLoggedIn();

		navigate("/");
	}
</script>

<div class="ui middle aligned center aligned grid">
	<div class="column">
		<h2 class="ui huge centered header">Log in</h2>

		<form
			class="ui large form {type === 'EMAIL' ? '' : 'hidden'}"
			on:submit|preventDefault={submitEmailForm}
		>
			<div class="ui stacked">
				<div class="field">
					<input
						type="email"
						placeholder="email"
						bind:value={email}
					/>
				</div>
				<input
					class="ui fluid large olive submit button"
					type="submit"
					value="Send"
				/>
			</div>
		</form>

		<form
			class="ui large form {type !== 'EMAIL' ? '' : 'hidden'}"
			on:submit|preventDefault={submitLoginForm}
		>
			<div class="ui stacked">
				<div class="field">
					<input
						class="ui input"
						type="email"
						readonly
						bind:value={email}
					/>
				</div>
				<div class="field">
					<input
						type="text"
						placeholder="code"
						class={type === "OTP" ? "" : "hidden"}
						bind:value={code}
					/>
				</div>
				<div class="field">
					<input
						type="password"
						placeholder="password"
						class={type === "PASSWORD" ? "" : "hidden"}
						bind:value={password}
					/>
				</div>
				<input
					class="ui fluid large olive submit button"
					type="submit"
					value="Login"
				/>
			</div>
		</form>
	</div>
</div>

<style>
	.ui.middle.aligned.center.aligned.grid {
		min-height: 90dvh;
	}

	.column {
		max-width: 55em;
	}
</style>
