const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],

	plugins: ['flowbite/plugin'],

	darkMode: 'class',

	theme: {
		extend: {
			colors: {
				// flowbite-svelte
				primary: {
					50: '#ffffff',
					100: '#c9ced5',
					200: '#b2e3ed',
					300: '#acc4ed',
					400: '#83b8ed',
					500: '#6bcada',
					600: '#00d9ff',
					700: '#0095FFFF',
					800: '#006fff',
					900: '#0033ff'
				}
			}
		}
	}
};

module.exports = config;
