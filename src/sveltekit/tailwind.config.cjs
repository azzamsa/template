const config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {}
  },

  daisyui: {
    themes: [
      {
        cupcake: {
          ...require('daisyui/src/colors/themes')['[data-theme=cupcake]'],
          '--btn-text-case': 'none'
        },
        dark: {
          ...require('daisyui/src/colors/themes')['[data-theme=dark]'],
          '--btn-text-case': 'none'
        }
      }
    ]
  },

  plugins: [require('daisyui')]
}

module.exports = config
