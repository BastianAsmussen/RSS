<script>
	import { BottomNav, BottomNavItem, Tooltip } from 'flowbite-svelte';
	import { HomeSolid, ReplyAllOutline, AdjustmentsVerticalOutline, UserCircleSolid, RectangleListOutline } from 'flowbite-svelte-icons';
</script>


<BottomNav position="absolute" navType="application" classInner="grid-cols-5">
	<BottomNavItem btnName="Playlists" appBtnPosition="left">
		<RectangleListOutline class="w-6 h-6 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-primary-600 dark:group-hover:text-primary-500" />
		<Tooltip arrow={false}>Playlists</Tooltip>
	</BottomNavItem>

	<BottomNavItem btnName="History" appBtnPosition="middle">
		<ReplyAllOutline class="w-6 h-6 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-primary-600 dark:group-hover:text-primary-500" />
		<Tooltip arrow={false}>History</Tooltip>
	</BottomNavItem>

	<div class="flex items-center justify-center">
		<BottomNavItem btnName="Home" href="/" appBtnPosition="middle" btnClass="inline-flex items-center justify-center w-10 h-10 font-medium bg-primary-600 rounded-full hover:bg-primary-700 group focus:ring-4 focus:ring-primary-300 focus:outline-none dark:focus:ring-primary-800">
			<HomeSolid class="text-white" />
			<Tooltip arrow={false}>Home</Tooltip>
		</BottomNavItem>
	</div>

	<BottomNavItem btnName="Settings" appBtnPosition="middle">
		<AdjustmentsVerticalOutline class="w-6 h-6 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-primary-600 dark:group-hover:text-primary-500" />
		<Tooltip arrow={false}>Settings</Tooltip>
	</BottomNavItem>

	<BottomNavItem btnName="Profile" href="/signin" appBtnPosition="right">
		<UserCircleSolid  class="w-6 h-6 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-primary-600 dark:group-hover:text-primary-500" />
		<Tooltip arrow={false}>Profile</Tooltip>
	</BottomNavItem>
</BottomNav>