<script>
	import Header from './Header.svelte';
	import Footer from './Footer.svelte';
</script>

<nav>
	<Header></Header>
	<Footer></Footer>
</nav>

<slot></slot>

<style lang="postcss">
	:global(body) {
		@apply bg-white dark:bg-gray-800;
	}
</style>
