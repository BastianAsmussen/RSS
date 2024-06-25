<script lang="ts">
	import {
		Navbar,
		NavBrand,
		NavLi,
		NavUl,
		NavHamburger,
		Button,
		Search,
		Avatar,
		Dropdown,
		DropdownItem,
		DropdownHeader,
		DropdownDivider,
		DarkMode,
		Checkbox,
		Label,
		Modal,
		FloatingLabelInput,
		Helper
	} from 'flowbite-svelte';
	import { SearchOutline } from 'flowbite-svelte-icons';
	import logo from '$lib/images/svelte-logo.svg';
	import { signedIn } from '../store';
	import { onDestroy } from 'svelte';
	import checkmark from '$lib/icons/checkmark.svg';
	import failmark from '$lib/icons/fail.svg';

	let isSignedIn: boolean;

	const unsubscribe = signedIn.subscribe((value) => {
		isSignedIn = value;
	});

	onDestroy(() => {
		unsubscribe();
	});

	function handleSignInSignOut(): void {
		if (isSignedIn) {
			signedIn.set(false);
		} else {
			signin = true;
		}
	}

	let signin = false;
	let signup = false;
	let email = '';
	let password = '';
	let repeatPassword = '';

	function handleLogin(event: Event): void {
		event.preventDefault();
		signedIn.set(true);
	}

	function validatePassword(password: string) {
		const length = password.length >= 12;
		const upperLower = /[a-z]/.test(password) && /[A-Z]/.test(password);
		const lettersNumbers = /[a-zA-Z]/.test(password) && /\d/.test(password);
		const specialChar = /[!@#?\]]/.test(password) && !/[<>]/.test(password);

		return {
			length,
			upperLower,
			lettersNumbers,
			specialChar,
			all: length && upperLower && lettersNumbers && specialChar
		};
	}

	function validateEmail(email: string) {
		return email.includes('@');
	}

	function handleCreateAccount(event: Event): void {
		event.preventDefault();
		if (validation.all && passwordsMatch && validateEmail(email)) {
			// Implement account creation logic here
			console.log('Account created successfully');
			signedIn.set(true);
			signup = false;
		} else {
			console.log('Validation failed');
		}
	}

	$: validation = validatePassword(password);
	$: passwordsMatch = validation.all && password === repeatPassword;
	$: emailValid = validateEmail(email);
</script>

<Navbar>
	<NavBrand href="/">
		<img src={logo} class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">RSS</span>
	</NavBrand>
	<div class="flex md:order-2">
		<Search size="md" class="rounded-none py-2.5" placeholder="Search..." />
		<Button class="rounded-s-none !p-2.5">
			<SearchOutline class="h-6 w-6" />
		</Button>
		<NavHamburger />
		<DarkMode class="border text-primary-500 dark:border-gray-800 dark:text-primary-600" />
	</div>
	<NavUl>
		<NavLi href="/" active={true}>Home</NavLi>
		<NavLi href="/about">About</NavLi>
	</NavUl>
	<div class="flex items-center md:order-2">
		<Avatar id="avatar-menu" src={logo} />
	</div>
	<Dropdown placement="bottom" triggeredBy="#avatar-menu">
		{#if isSignedIn}
			<DropdownHeader>
				<span class="block text-sm">Filler Name</span>
				<span class="block truncate text-sm font-medium">name@Filler.com</span>
			</DropdownHeader>
			<DropdownItem>Profile</DropdownItem>
			<DropdownItem>Playlist</DropdownItem>
		{/if}
		<DropdownItem>History</DropdownItem>
		<DropdownDivider />
		<DropdownItem>Settings</DropdownItem>
		<DropdownItem>Help</DropdownItem>
		<DropdownItem>Send feedback</DropdownItem>
		<DropdownDivider />
		<DropdownItem on:click={handleSignInSignOut}>{isSignedIn ? 'Sign out' : 'Sign in'}</DropdownItem
		>
	</Dropdown>
</Navbar>

<!-- Modal for SignIn & SignUp -->
<Modal bind:open={signin} size="xs" class="w-full" autoclose outsideclose>
	<form class="flex flex-col space-y-6" action="#" on:submit={handleLogin}>
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Sign In</h3>
		<Label class="space-y-2">
			<FloatingLabelInput style="filled" type="email" required>Email</FloatingLabelInput>
		</Label>
		<Label class="space-y-2">
			<FloatingLabelInput style="filled" type="password" required>Password</FloatingLabelInput>
		</Label>
		<div class="flex items-start">
			<Checkbox>Remember me</Checkbox>
			<a href="/" class="ms-auto text-sm text-primary-700 hover:underline dark:text-primary-500">
				Lost password?
			</a>
		</div>
		<Button type="submit" class="w-full1" on:click={handleLogin}>Login to your account</Button>
		<div class="text-sm font-medium text-gray-500 dark:text-gray-300">
			<Button on:click={() => (signup = true)}>Create account</Button>
		</div>
	</form>
</Modal>

<Modal bind:open={signup} size="xs" class="w-full" autoclose outsideclose>
	<form class="flex flex-col space-y-6" action="#" on:submit={handleCreateAccount}>
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Sign Up</h3>
		<Label class="space-y-2">
			<FloatingLabelInput style="filled" type="email" required bind:value={email}
				>Email</FloatingLabelInput
			>
			<Helper>
				<li style="display: flex; align-items: center; margin-top: 4px;">
					<img
						src={emailValid ? checkmark : failmark}
						alt="Checkmark"
						style="width: 16px; height: 16px; margin-right: 8px;"
					/>
					<span>Valid email address</span>
				</li>
			</Helper>
		</Label>
		<Label class="space-y-2">
			<FloatingLabelInput style="filled" type="password" required bind:value={password}
				>Password</FloatingLabelInput
			>
			<Helper>
				<div style="display: flex; align-items: center;">
					<span>Password must meet the following requirements:</span>
				</div>
				<ul style="list-style: none; padding: 0; margin: 0;">
					<li style="display: flex; align-items: center; margin-top: 4px;">
						<img
							src={validation.length ? checkmark : failmark}
							alt="Checkmark"
							style="width: 16px; height: 16px; margin-right: 8px;"
						/>
						<span>At least 12 characters long but 14 or more is better</span>
					</li>
					<li style="display: flex; align-items: center; margin-top: 4px;">
						<img
							src={validation.upperLower ? checkmark : failmark}
							alt="Checkmark"
							style="width: 16px; height: 16px; margin-right: 8px;"
						/>
						<span>A mixture of both uppercase and lowercase letters</span>
					</li>
					<li style="display: flex; align-items: center; margin-top: 4px;">
						<img
							src={validation.lettersNumbers ? checkmark : failmark}
							alt="Checkmark"
							style="width: 16px; height: 16px; margin-right: 8px;"
						/>
						<span>A mixture of letters and numbers</span>
					</li>
					<li style="display: flex; align-items: center; margin-top: 4px;">
						<img
							src={validation.specialChar ? checkmark : failmark}
							alt="Checkmark"
							style="width: 16px; height: 16px; margin-right: 8px;"
						/>
						<span
							>Inclusion of at least one special character, e.g., ! @ # ? ] - NOT ALLOWED &lt; or
							&gt;</span
						>
					</li>
				</ul>
			</Helper>
		</Label>
		<Label class="space-y-2">
			<FloatingLabelInput style="filled" type="password" required bind:value={repeatPassword}
				>Repeat Password</FloatingLabelInput
			>
			<Helper>
				<li style="display: flex; align-items: center; margin-top: 4px;">
					<img
						src={passwordsMatch ? checkmark : failmark}
						alt="Checkmark"
						style="width: 16px; height: 16px; margin-right: 8px;"
					/>
					<span>Password must match.</span>
				</li>
			</Helper>
		</Label>
		<Button
			type="submit"
			class="w-full1"
			on:click={handleCreateAccount}
			disabled={!validation.all || !passwordsMatch || !emailValid}>Create your account</Button
		>
		<div class="text-sm font-medium text-gray-500 dark:text-gray-300">
			<Button on:click={() => (signin = true)}>Already have an Account?</Button>
		</div>
	</form>
</Modal>
