<script>
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
		DarkMode
	} from 'flowbite-svelte';
	import { SearchOutline } from 'flowbite-svelte-icons';
	import logo from '$lib/images/svelte-logo.svg';
	import { signedIn } from '../store';
	import { goto } from '$app/navigation';
	import { onDestroy } from 'svelte';

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
			goto('/signin');
		}
	}
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
