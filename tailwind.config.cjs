const config = {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			keyframes: {
				'slide-up': {
					from: {
						transform: 'translateY(25%)',
						opacity: '0'
					},
					to: {
						transform: 'unset',
						opacity: '1'
					}
				},
				'text-appear': {
					from: {
						opacity: '0'
					},
					to: {
						opacity: '1'
					}
				}
			},
			animation: {
				'slide-up': 'slide-up 0.5s cubic-bezier(0.5, 1.26, 1, 1.05)',
				'text-appear': 'text-appear 0.1s linear'
			}
		}
	},

	plugins: []
};

module.exports = config;
