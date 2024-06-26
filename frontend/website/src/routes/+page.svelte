<script lang="ts">
	import { Card, Badge } from 'flowbite-svelte';
	import { ClockSolid } from 'flowbite-svelte-icons';
	import '../app.css';
	import noImage from '../lib/images/noImage.jpg';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>HOME</h1>
</section>

<main class="bg-white dark:bg-gray-800">
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
</main>
