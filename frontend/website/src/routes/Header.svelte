<script>
	import logo from '$lib/images/svelte-logo.svg';
	import { Search, Dropdown, DropdownItem, Avatar, Drawer, Button, CloseButton, Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import { HomeOutline, InfoCircleOutline, ArrowRightToBracketOutline, EditOutline } from 'flowbite-svelte-icons';
	import { sineIn } from 'svelte/easing';
	let hidden2 = true;
	let transitionParams = {
		x: -320,
		duration: 200,
		easing: sineIn
	};

</script>

<header>
	<div class="corner">
		<div class="text-center">
			<Button on:click={() => (hidden2 = false)}>Menu</Button>
		</div>
		<Drawer backdrop={false} transitionType="fly" {transitionParams} bind:hidden={hidden2} id="sidebar2">
			<div class="flex items-center">
				<h5 id="drawer-navigation-label-3" class="text-base font-semibold text-gray-500 uppercase dark:text-gray-400">Menu</h5>
				<CloseButton on:click={() => (hidden2 = true)} class="mb-4 dark:text-white" />
			</div>
			<Sidebar>
				<SidebarWrapper divClass="overflow-y-auto py-4 px-3 rounded dark:bg-gray-800">
					<SidebarGroup>

						<SidebarItem label="Home" href="/">
							<svelte:fragment slot="icon">
								<HomeOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
							</svelte:fragment>
						</SidebarItem>


						<SidebarItem label="About" href="/about" >
							<svelte:fragment slot="icon">
								<InfoCircleOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
							</svelte:fragment>
						</SidebarItem>

						<SidebarItem label="Sign In" href="/signin">
							<svelte:fragment slot="icon">
								<ArrowRightToBracketOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
							</svelte:fragment>
						</SidebarItem>

						<SidebarItem label="Sign Up" href="/signup">
							<svelte:fragment slot="icon">
								<EditOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
							</svelte:fragment>
						</SidebarItem>

					</SidebarGroup>
				</SidebarWrapper>
			</Sidebar>
		</Drawer>
	</div>

	<nav>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
		</svg>
		<ul>
			<li>
				<Search class="search">
					<Button>Search</Button>
				</Search>
			</li>
		</ul>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
		</svg>
	</nav>

	<div class="corner">
		<div>
			<Avatar class="acs" src={logo} dot={{ color: 'green' }} />
			<Dropdown triggeredBy=".acs">
				<div slot="header" class="px-4 py-2">
					<span class="block text-sm text-gray-900 dark:text-white">Filler Name</span>
					<span class="block truncate text-sm font-medium">name@filler.com</span>
				</div>
				<DropdownItem>Dashboard</DropdownItem>
				<DropdownItem>Settings</DropdownItem>
				<DropdownItem slot="footer">Sign out</DropdownItem>
			</Dropdown>
		</div>
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	.corner {
		width: 3em;
		height: 3em;
	}


	nav {
		display: flex;
		justify-content: center;
		--background: rgba(255, 255, 255, 0.7);
	}

	svg {
		width: 2em;
		height: 3em;
		display: block;
	}

	path {
		fill: var(--background);
	}

	ul {
		position: relative;
		padding: 0;
		margin: 0;
		height: 3em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background: var(--background);
		background-size: contain;
	}

	li {
		position: relative;
		height: 100%;
	}

</style>
