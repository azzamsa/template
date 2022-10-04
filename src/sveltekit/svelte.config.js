import preprocess from 'svelte-preprocess'
import nodeAdapter from '@sveltejs/adapter-node'
import netlifyAdapter from '@sveltejs/adapter-netlify'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      postcss: true
    })
  ],

  kit: {
    adapter: process.env.NODE_ENV === 'production' ? netlifyAdapter() : nodeAdapter(),
    alias: {
      $stores: 'src/stores'
    }
  }
}

export default config
