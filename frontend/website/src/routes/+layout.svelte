<script>
	import '../app.css';
	import Header from './Header.svelte';
	import Footer from './Footer.svelte';
	import './styles.css';
</script>

<div class="app">
	<Header></Header>

	<main class="bg-white dark:bg-gray-800">
		<slot></slot>
	</main>

	<Footer></Footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-width: 100vw;
		min-height: 100vh;
		margin: 0;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>
