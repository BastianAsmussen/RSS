const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],

	plugins: [require('flowbite/plugin')],

	darkMode: 'selector',

	theme: {
		extend: {
			colors: {
				// flowbite-svelte
				primary: {
					50: '#FFF5F2',
					100: '#FFF1EE',
					200: '#b2e3ed',
					300: '#8aa1c8',
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
