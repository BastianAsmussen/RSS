<script>
	import { FloatingLabelInput, Card, Button } from 'flowbite-svelte';
</script>

<div class="center">
	<Card>
		<form class="flex flex-col space-y-6" action="/">
			<h3 class="text-xl font-medium text-gray-900 dark:text-white">Sign Up</h3>

			<FloatingLabelInput style="outlined" id="floating_outlined" name="floating_outlined" type="text">
				Username
			</FloatingLabelInput>

			<FloatingLabelInput style="outlined" id="floating_outlined" name="floating_outlined" type="text">
				Email
			</FloatingLabelInput>

			<FloatingLabelInput style="outlined" id="floating_outlined" name="floating_outlined" type="text">
				Password
			</FloatingLabelInput>

			<FloatingLabelInput style="outlined" id="floating_outlined" name="floating_outlined" type="text">
				Repeat Password
			</FloatingLabelInput>

			<Button type="submit" class="w-full">Create account</Button>
			<div class="text-sm font-medium text-gray-500 dark:text-gray-300">
				Not registered? <a href="/signin" class="text-primary-700 hover:underline dark:text-primary-500"> Already have an Account? </a>
			</div>
		</form>
	</Card>
</div>

<style>
    .center {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        flex: 0.6;
    }

</style>