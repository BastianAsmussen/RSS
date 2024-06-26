<script lang="ts">
	import { FloatingLabelInput, Card, Button, Checkbox } from 'flowbite-svelte';
	import { signedIn } from '../../store';
	import { goto } from '$app/navigation';

	function handleLogin(event: Event): void {
		event.preventDefault();
		signedIn.set(true);
		goto('/');
	}
</script>

<div class="center">
	<Card>
		<form class="flex flex-col space-y-6" on:submit={handleLogin}>
			<h3 class="text-xl font-medium text-gray-900 dark:text-white">Sign in</h3>

			<FloatingLabelInput
				style="outlined"
				id="floating_outlined"
				name="floating_outlined"
				type="text"
			>
				Email
			</FloatingLabelInput>

			<FloatingLabelInput
				style="outlined"
				id="floating_outlined"
				name="floating_outlined"
				type="text"
			>
				Password
			</FloatingLabelInput>

			<div class="flex items-start">
				<Checkbox>Remember me</Checkbox>
				<a href="/" class="ms-auto text-sm text-primary-700 hover:underline dark:text-primary-500">
					Forgot password?
				</a>
			</div>

			<Button type="submit" class="w-full">Login</Button>
			<div class="text-sm font-medium text-gray-500 dark:text-gray-300">
				Not registered? <a
					href="/signup"
					class="text-primary-700 hover:underline dark:text-primary-500"
				>
					Create account
				</a>
			</div>
		</form>
	</Card>
</div>
