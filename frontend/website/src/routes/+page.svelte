<script>
	import { Card, Badge } from 'flowbite-svelte';
	import { ClockSolid } from 'flowbite-svelte-icons';
	import noImage from '../lib/images/noImage.jpg';
</script>

<div class="space-y-4">
	<Card img={noImage} href="/">
		<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
			Text filler for title of video - wooow
		</h5>
		<Badge color="dark">Uploader Name</Badge>
		<Badge color="dark" border>
			<ClockSolid class="me-1.5 h-2.5 w-2.5" />
			3 days ago
		</Badge>
	</Card>
</div>
